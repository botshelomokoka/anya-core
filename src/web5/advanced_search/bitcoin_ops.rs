use anyhow::Result;
use bitcoin::blockdata::opcodes::all::*;
use bitcoin::blockdata::script::Instruction;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct OpCodeExecutor {
    op_handlers: Arc<HashMap<bitcoin::blockdata::opcodes::All, OpHandler>>,
    stack: Vec<Vec<u8>>,
    alt_stack: Vec<Vec<u8>>,
    pub trace: Vec<String>,
}

type OpHandler = Box<dyn Fn(&mut OpCodeExecutor) -> Result<()> + Send + Sync>;

#[derive(Debug, Clone)]
pub struct ScriptExecutionContext {
    pub stack_state: Vec<Vec<u8>>,
    pub alt_stack_state: Vec<Vec<u8>>,
    pub op_count: usize,
    pub taproot_leaf_version: Option<u8>,
    pub sigversion: SignatureVersion,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SignatureVersion {
    Base,
    WitnessV0,
    Taproot,
    Tapscript,
}

impl OpCodeExecutor {
    pub fn new() -> Self {
        let mut executor = Self {
            op_handlers: Arc::new(HashMap::new()),
            stack: Vec::new(),
            alt_stack: Vec::new(),
            trace: Vec::new(),
        };
        
        executor.register_default_handlers();
        executor
    }

    fn register_default_handlers(&mut self) {
        let mut handlers = HashMap::new();

        // Stack Operations
        handlers.insert(OP_DUP, Box::new(|exec| {
            if let Some(top) = exec.stack.last() {
                exec.stack.push(top.clone());
                exec.trace.push("OP_DUP: Duplicated top stack item".into());
            }
            Ok(())
        }) as OpHandler);

        handlers.insert(OP_DROP, Box::new(|exec| {
            exec.stack.pop();
            exec.trace.push("OP_DROP: Removed top stack item".into());
            Ok(())
        }));

        // Arithmetic Operations
        handlers.insert(OP_ADD, Box::new(|exec| {
            if exec.stack.len() < 2 {
                return Err(anyhow::anyhow!("OP_ADD: Stack underflow"));
            }
            let b = exec.parse_number(&exec.stack.pop().unwrap())?;
            let a = exec.parse_number(&exec.stack.pop().unwrap())?;
            exec.stack.push((a + b).to_le_bytes().to_vec());
            exec.trace.push(format!("OP_ADD: {} + {} = {}", a, b, a + b));
            Ok(())
        }));

        // Cryptographic Operations
        handlers.insert(OP_HASH160, Box::new(|exec| {
            if let Some(data) = exec.stack.pop() {
                let hash = bitcoin::hashes::hash160::Hash::hash(&data);
                exec.stack.push(hash[..].to_vec());
                exec.trace.push("OP_HASH160: Computed RIPEMD160(SHA256())".into());
            }
            Ok(())
        }));

        handlers.insert(OP_CHECKSIG, Box::new(|exec| {
            if exec.stack.len() < 2 {
                return Err(anyhow::anyhow!("OP_CHECKSIG: Stack underflow"));
            }
            let pubkey = exec.stack.pop().unwrap();
            let sig = exec.stack.pop().unwrap();
            // TODO: Implement actual signature verification
            exec.stack.push(vec![1]);
            exec.trace.push("OP_CHECKSIG: Signature verification placeholder".into());
            Ok(())
        }));

        // Taproot-specific Operations
        handlers.insert(OP_CHECKSIGADD, Box::new(|exec| {
            if exec.stack.len() < 3 {
                return Err(anyhow::anyhow!("OP_CHECKSIGADD: Stack underflow"));
            }
            let n = exec.parse_number(&exec.stack.pop().unwrap())?;
            let pubkey = exec.stack.pop().unwrap();
            let sig = exec.stack.pop().unwrap();
            // TODO: Implement actual signature verification
            exec.stack.push((n + 1).to_le_bytes().to_vec());
            exec.trace.push(format!("OP_CHECKSIGADD: Added 1 to {}", n));
            Ok(())
        }));

        // Control Flow Operations
        handlers.insert(OP_IF, Box::new(|exec| {
            if let Some(condition) = exec.stack.pop() {
                exec.trace.push(format!("OP_IF: Condition = {:?}", condition));
            }
            Ok(())
        }));

        handlers.insert(OP_ELSE, Box::new(|exec| {
            exec.trace.push("OP_ELSE: Switched to else branch".into());
            Ok(())
        }));

        handlers.insert(OP_ENDIF, Box::new(|exec| {
            exec.trace.push("OP_ENDIF: End of conditional".into());
            Ok(())
        }));

        // Time Lock Operations
        handlers.insert(OP_CHECKLOCKTIMEVERIFY, Box::new(|exec| {
            if let Some(locktime) = exec.stack.last() {
                let lock_value = exec.parse_number(locktime)?;
                exec.trace.push(format!("OP_CHECKLOCKTIMEVERIFY: Locktime = {}", lock_value));
            }
            Ok(())
        }));

        handlers.insert(OP_CHECKSEQUENCEVERIFY, Box::new(|exec| {
            if let Some(sequence) = exec.stack.last() {
                let seq_value = exec.parse_number(sequence)?;
                exec.trace.push(format!("OP_CHECKSEQUENCEVERIFY: Sequence = {}", seq_value));
            }
            Ok(())
        }));

        // Tapscript Extensions
        handlers.insert(OP_SUCCESS126, Box::new(|exec| {
            exec.trace.push("OP_SUCCESS126: Future tapscript extension".into());
            Ok(())
        }));

        handlers.insert(OP_SUCCESS127, Box::new(|exec| {
            exec.trace.push("OP_SUCCESS127: Future tapscript extension".into());
            Ok(())
        }));

        self.op_handlers = Arc::new(handlers);
    }

    pub fn execute_script(&mut self, script: &bitcoin::blockdata::script::Script) -> Result<bool> {
        self.trace.clear();
        self.trace.push("=== Script Execution Start ===".into());

        for instruction in script.iter() {
            match instruction {
                Ok(Instruction::Op(op)) => {
                    if let Some(handler) = self.op_handlers.get(&op) {
                        handler(self)?;
                    } else {
                        self.trace.push(format!("Unhandled opcode: {:?}", op));
                    }
                }
                Ok(Instruction::PushBytes(data)) => {
                    self.stack.push(data.to_vec());
                    self.trace.push(format!("Push bytes: {:?}", data));
                }
                Err(e) => {
                    self.trace.push(format!("Invalid instruction: {:?}", e));
                    return Ok(false);
                }
            }
            self.trace.push(format!("Stack state: {:?}", self.stack));
        }

        self.trace.push("=== Script Execution End ===".into());
        Ok(!self.stack.is_empty() && self.stack.last().unwrap() != &vec![0])
    }

    pub fn execute_tapscript(
        &mut self,
        script: &bitcoin::blockdata::script::Script,
        leaf_version: u8,
    ) -> Result<bool> {
        self.trace.push(format!("=== Tapscript Execution (Leaf Version: {}) ===", leaf_version));
        
        // Special handling for different leaf versions
        match leaf_version {
            0xc0 => {
                // BIP342 default leaf version
                self.execute_script(script)
            }
            0xc1..=0xcf => {
                // Future leaf versions
                self.trace.push(format!("Future leaf version: {}", leaf_version));
                Ok(true)
            }
            _ => {
                self.trace.push(format!("Invalid leaf version: {}", leaf_version));
                Ok(false)
            }
        }
    }

    fn parse_number(&self, data: &[u8]) -> Result<i64> {
        if data.is_empty() {
            return Ok(0);
        }
        
        let mut result = 0i64;
        for (i, &byte) in data.iter().enumerate() {
            result |= (byte as i64) << (8 * i);
        }
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::blockdata::script::Builder;

    #[test]
    fn test_p2pkh_script() -> Result<()> {
        let mut executor = OpCodeExecutor::new();

        // Create a basic P2PKH script
        let script = Builder::new()
            .push_opcode(OP_DUP)
            .push_opcode(OP_HASH160)
            .push_bytes(&[0; 20]) // Dummy pubkey hash
            .push_opcode(OP_EQUALVERIFY)
            .push_opcode(OP_CHECKSIG)
            .into_script();

        let result = executor.execute_script(&script)?;
        
        // Print execution trace
        for line in &executor.trace {
            println!("{}", line);
        }

        assert!(result);
        Ok(())
    }

    #[test]
    fn test_taproot_script() -> Result<()> {
        let mut executor = OpCodeExecutor::new();

        // Create a basic Taproot script
        let script = Builder::new()
            .push_opcode(OP_CHECKSIGADD)
            .push_int(1)
            .into_script();

        let result = executor.execute_tapscript(&script, 0xc0)?;
        
        for line in &executor.trace {
            println!("{}", line);
        }

        assert!(result);
        Ok(())
    }
}
