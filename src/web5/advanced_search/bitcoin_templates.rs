use anyhow::Result;
use bitcoin::blockdata::script::{Builder, Script};
use bitcoin::blockdata::opcodes::all::*;
use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey};
use bitcoin::util::hash::Hash;
use bitcoin::hashes::{sha256, ripemd160, Hash as HashTrait};
use std::time::{SystemTime, UNIX_EPOCH};
use super::bitcoin_ops::OpCodeExecutor;

pub struct ScriptTemplate {
    secp: Secp256k1<bitcoin::secp256k1::All>,
}

impl ScriptTemplate {
    pub fn new() -> Self {
        Self {
            secp: Secp256k1::new(),
        }
    }

    /// Multisignature (m-of-n) Script Template
    pub fn create_multisig(&self, m: usize, public_keys: &[PublicKey]) -> Result<Script> {
        if m > public_keys.len() {
            return Err(anyhow::anyhow!("Required signatures exceeds number of keys"));
        }

        let mut builder = Builder::new();
        
        // Push m (required signatures)
        builder = builder.push_int(m as i64);
        
        // Push public keys
        for pubkey in public_keys {
            builder = builder.push_key(&pubkey);
        }
        
        // Push n (total keys)
        builder = builder.push_int(public_keys.len() as i64);
        
        // Add CHECKMULTISIG
        builder = builder.push_opcode(OP_CHECKMULTISIG);

        Ok(builder.into_script())
    }

    /// Hash Time-Locked Contract (HTLC) Template
    pub fn create_htlc(
        &self,
        recipient_pubkey: &PublicKey,
        sender_pubkey: &PublicKey,
        hash: sha256::Hash,
        timeout: u32,
    ) -> Result<Script> {
        let builder = Builder::new()
            .push_opcode(OP_IF)
                // Hash branch
                .push_opcode(OP_SHA256)
                .push_slice(&hash[..])
                .push_opcode(OP_EQUALVERIFY)
                .push_key(&recipient_pubkey)
            .push_opcode(OP_ELSE)
                // Timeout branch
                .push_int(timeout as i64)
                .push_opcode(OP_CHECKLOCKTIMEVERIFY)
                .push_opcode(OP_DROP)
                .push_key(&sender_pubkey)
            .push_opcode(OP_ENDIF)
            .push_opcode(OP_CHECKSIG);

        Ok(builder.into_script())
    }

    /// Taproot Multi-Path Template
    pub fn create_taproot_multipath(
        &self,
        internal_key: &PublicKey,
        scripts: &[(Script, u8)], // (Script, weight)
    ) -> Result<(Script, Vec<u8>)> {
        use bitcoin::util::taproot::{TaprootBuilder, LeafVersion, TapTweakHash};
        
        let mut builder = TaprootBuilder::new();
        
        // Add each script as a leaf
        for (i, (script, weight)) in scripts.iter().enumerate() {
            builder = builder.add_leaf(i as u8, script.clone(), LeafVersion::TapScript)?;
        }

        // Finalize Taproot construction
        let (output_key, merkle_root) = builder.finalize(&self.secp, *internal_key)?;
        
        // Create output script
        let script = Builder::new()
            .push_slice(&output_key.serialize())
            .push_opcode(OP_CHECKSIG)
            .into_script();

        Ok((script, merkle_root.as_inner().to_vec()))
    }

    /// Pay-to-Script-Hash (P2SH) Wrapped Segwit Template
    pub fn create_p2sh_wrapped_segwit(
        &self,
        internal_script: &Script,
    ) -> Result<(Script, Script)> {
        // Create witness script
        let witness_script = Builder::new()
            .push_int(0)
            .push_slice(&internal_script.serialize())
            .into_script();

        // Create P2SH redeem script
        let redeem_script_hash = ripemd160::Hash::hash(&sha256::Hash::hash(&witness_script.serialize())[..]);
        
        let p2sh_script = Builder::new()
            .push_opcode(OP_HASH160)
            .push_slice(&redeem_script_hash[..])
            .push_opcode(OP_EQUAL)
            .into_script();

        Ok((p2sh_script, witness_script))
    }

    /// Relative Time-Locked Refund Template
    pub fn create_timelock_refund(
        &self,
        recipient_pubkey: &PublicKey,
        refund_pubkey: &PublicKey,
        relative_locktime: u32,
    ) -> Result<Script> {
        let builder = Builder::new()
            .push_opcode(OP_IF)
                // Normal spend path
                .push_key(&recipient_pubkey)
            .push_opcode(OP_ELSE)
                // Refund path with relative timelock
                .push_int(relative_locktime as i64)
                .push_opcode(OP_CHECKSEQUENCEVERIFY)
                .push_opcode(OP_DROP)
                .push_key(&refund_pubkey)
            .push_opcode(OP_ENDIF)
            .push_opcode(OP_CHECKSIG);

        Ok(builder.into_script())
    }

    /// Multi-Signature with Timeout Template
    pub fn create_multisig_timeout(
        &self,
        required_sigs: usize,
        pubkeys: &[PublicKey],
        timeout_pubkey: &PublicKey,
        timeout: u32,
    ) -> Result<Script> {
        let mut builder = Builder::new()
            .push_opcode(OP_IF)
                // Multi-sig branch
                .push_int(required_sigs as i64);

        // Add public keys
        for pubkey in pubkeys {
            builder = builder.push_key(&pubkey);
        }
        
        builder = builder
            .push_int(pubkeys.len() as i64)
            .push_opcode(OP_CHECKMULTISIG)
            .push_opcode(OP_ELSE)
                // Timeout branch
                .push_int(timeout as i64)
                .push_opcode(OP_CHECKLOCKTIMEVERIFY)
                .push_opcode(OP_DROP)
                .push_key(&timeout_pubkey)
                .push_opcode(OP_CHECKSIG)
            .push_opcode(OP_ENDIF);

        Ok(builder.into_script())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::secp256k1::SecretKey;

    #[test]
    fn test_multisig_template() -> Result<()> {
        let template = ScriptTemplate::new();
        let secp = Secp256k1::new();

        // Generate test keys
        let secret_keys: Vec<SecretKey> = (0..3)
            .map(|_| SecretKey::new(&mut rand::thread_rng()))
            .collect();
        let public_keys: Vec<PublicKey> = secret_keys
            .iter()
            .map(|sk| PublicKey::from_secret_key(&secp, sk))
            .collect();

        let script = template.create_multisig(2, &public_keys)?;
        
        // Verify script structure
        let mut executor = OpCodeExecutor::new();
        let result = executor.execute_script(&script)?;
        
        assert!(result);
        Ok(())
    }

    #[test]
    fn test_htlc_template() -> Result<()> {
        let template = ScriptTemplate::new();
        let secp = Secp256k1::new();

        // Generate test keys
        let recipient_secret = SecretKey::new(&mut rand::thread_rng());
        let sender_secret = SecretKey::new(&mut rand::thread_rng());
        let recipient_pubkey = PublicKey::from_secret_key(&secp, &recipient_secret);
        let sender_pubkey = PublicKey::from_secret_key(&secp, &sender_secret);

        // Create hash
        let preimage = b"test_preimage";
        let hash = sha256::Hash::hash(preimage);

        let script = template.create_htlc(
            &recipient_pubkey,
            &sender_pubkey,
            hash,
            1000,
        )?;

        // Verify script structure
        let mut executor = OpCodeExecutor::new();
        let result = executor.execute_script(&script)?;
        
        assert!(result);
        Ok(())
    }
}
