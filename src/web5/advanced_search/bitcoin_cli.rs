use anyhow::Result;
use bitcoin::blockdata::script::{Script, Builder};
use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey};
use bitcoin::util::key::PublicKey as BitcoinPublicKey;
use clap::{App, Arg, SubCommand};
use std::str::FromStr;
use super::bitcoin_ops::OpCodeExecutor;
use super::bitcoin_templates::ScriptTemplate;
use colored::*;

pub struct BitcoinScriptCLI {
    executor: OpCodeExecutor,
    template: ScriptTemplate,
    secp: Secp256k1<bitcoin::secp256k1::All>,
}

impl BitcoinScriptCLI {
    pub fn new() -> Self {
        Self {
            executor: OpCodeExecutor::new(),
            template: ScriptTemplate::new(),
            secp: Secp256k1::new(),
        }
    }

    pub fn run() -> Result<()> {
        let app = App::new("Bitcoin Script Debugger")
            .version("1.0")
            .author("Anya Project")
            .about("Debug and analyze Bitcoin scripts")
            .subcommand(SubCommand::with_name("analyze")
                .about("Analyze a Bitcoin script")
                .arg(Arg::with_name("script")
                    .help("The script hex to analyze")
                    .required(true)
                    .index(1)))
            .subcommand(SubCommand::with_name("create-multisig")
                .about("Create a multisig script")
                .arg(Arg::with_name("required")
                    .help("Required signatures")
                    .required(true)
                    .index(1))
                .arg(Arg::with_name("pubkeys")
                    .help("Comma-separated public keys")
                    .required(true)
                    .index(2)))
            .subcommand(SubCommand::with_name("create-htlc")
                .about("Create an HTLC script")
                .arg(Arg::with_name("recipient")
                    .help("Recipient public key")
                    .required(true)
                    .index(1))
                .arg(Arg::with_name("sender")
                    .help("Sender public key")
                    .required(true)
                    .index(2))
                .arg(Arg::with_name("hash")
                    .help("Hash value (hex)")
                    .required(true)
                    .index(3))
                .arg(Arg::with_name("timeout")
                    .help("Timeout value")
                    .required(true)
                    .index(4)))
            .subcommand(SubCommand::with_name("debug")
                .about("Debug script execution")
                .arg(Arg::with_name("script")
                    .help("The script hex to debug")
                    .required(true)
                    .index(1))
                .arg(Arg::with_name("stack")
                    .help("Initial stack items (comma-separated hex)")
                    .required(false)
                    .index(2)));

        let cli = Self::new();
        let matches = app.get_matches();

        match matches.subcommand() {
            ("analyze", Some(sub_m)) => {
                let script_hex = sub_m.value_of("script").unwrap();
                cli.analyze_script(script_hex)
            }
            ("create-multisig", Some(sub_m)) => {
                let required: usize = sub_m.value_of("required").unwrap().parse()?;
                let pubkeys = sub_m.value_of("pubkeys").unwrap();
                cli.create_multisig(required, pubkeys)
            }
            ("create-htlc", Some(sub_m)) => {
                let recipient = sub_m.value_of("recipient").unwrap();
                let sender = sub_m.value_of("sender").unwrap();
                let hash = sub_m.value_of("hash").unwrap();
                let timeout: u32 = sub_m.value_of("timeout").unwrap().parse()?;
                cli.create_htlc(recipient, sender, hash, timeout)
            }
            ("debug", Some(sub_m)) => {
                let script_hex = sub_m.value_of("script").unwrap();
                let stack = sub_m.value_of("stack").unwrap_or("");
                cli.debug_script(script_hex, stack)
            }
            _ => Ok(()),
        }
    }

    fn analyze_script(&self, script_hex: &str) -> Result<()> {
        let script_bytes = hex::decode(script_hex)?;
        let script = Script::from(script_bytes);

        println!("{}", "=== Script Analysis ===".green().bold());
        println!("Script Size: {} bytes", script.len());
        println!("OP Codes:");

        for instruction in script.iter() {
            match instruction {
                Ok(bitcoin::blockdata::script::Instruction::Op(op)) => {
                    println!("  {}", op.to_string().yellow());
                }
                Ok(bitcoin::blockdata::script::Instruction::PushBytes(data)) => {
                    println!("  Push: {}", hex::encode(data).blue());
                }
                Err(e) => println!("  {}: {}", "Error".red(), e),
            }
        }

        Ok(())
    }

    fn create_multisig(&self, required: usize, pubkeys: &str) -> Result<()> {
        let pubkey_list: Vec<PublicKey> = pubkeys
            .split(',')
            .map(|key| PublicKey::from_str(key))
            .collect::<Result<Vec<_>, _>>()?;

        let script = self.template.create_multisig(required, &pubkey_list)?;

        println!("{}", "=== Generated Multisig Script ===".green().bold());
        println!("Script Hex: {}", hex::encode(script.serialize()).yellow());
        println!("Required Signatures: {}", required);
        println!("Total Public Keys: {}", pubkey_list.len());

        Ok(())
    }

    fn create_htlc(
        &self,
        recipient: &str,
        sender: &str,
        hash: &str,
        timeout: u32,
    ) -> Result<()> {
        let recipient_key = PublicKey::from_str(recipient)?;
        let sender_key = PublicKey::from_str(sender)?;
        let hash_bytes = hex::decode(hash)?;
        let hash = bitcoin::hashes::sha256::Hash::from_slice(&hash_bytes)?;

        let script = self.template.create_htlc(
            &recipient_key,
            &sender_key,
            hash,
            timeout,
        )?;

        println!("{}", "=== Generated HTLC Script ===".green().bold());
        println!("Script Hex: {}", hex::encode(script.serialize()).yellow());
        println!("Timeout: {}", timeout);
        println!("Hash: {}", hash);

        Ok(())
    }

    fn debug_script(&self, script_hex: &str, stack_hex: &str) -> Result<()> {
        let script_bytes = hex::decode(script_hex)?;
        let script = Script::from(script_bytes);

        let mut executor = OpCodeExecutor::new();
        
        // Initialize stack if provided
        if !stack_hex.is_empty() {
            for item in stack_hex.split(',') {
                let bytes = hex::decode(item)?;
                executor.push_stack(bytes);
            }
        }

        let result = executor.execute_script(&script)?;

        println!("{}", "=== Script Execution Trace ===".green().bold());
        for (i, line) in executor.trace.iter().enumerate() {
            println!("{:3}. {}", i, line);
        }

        println!("\n{}", "=== Execution Result ===".green().bold());
        println!("Success: {}", if result { "Yes".green() } else { "No".red() });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_analysis() -> Result<()> {
        let cli = BitcoinScriptCLI::new();
        
        // Test P2PKH script analysis
        let script_hex = "76a914000000000000000000000000000000000000000088ac";
        cli.analyze_script(script_hex)?;

        Ok(())
    }

    #[test]
    fn test_script_debug() -> Result<()> {
        let cli = BitcoinScriptCLI::new();
        
        // Test basic arithmetic script debugging
        let script_hex = "52935293"; // 2 OP_ADD 3 OP_ADD
        cli.debug_script(script_hex, "")?;

        Ok(())
    }
}
