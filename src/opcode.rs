use crate::evm::Evm;

/// An enumeration of supported Ethereum opcodes.
#[derive(Debug, PartialEq)]
pub enum Opcode {
    STOP,
    ADD,
    MUL,
    PUSH1,
}

impl Opcode {
    /// Get the corresponding opcode of a byte
    ///
    /// # Arguments
    ///
    /// * `value` - A byte that match an opcode
    ///
    /// # Returns
    ///
    /// * An option containing the Opcode
    pub fn from(value: &u8) -> Option<Self> {
        match value {
            0x00 => Some(Self::STOP),
            0x01 => Some(Self::ADD),
            0x02 => Some(Self::MUL),
            0x60 => Some(Self::PUSH1),
            _ => None,
        }
    }

    /// Execute the opcode in the given evm execution context
    ///
    /// # Arguments
    ///
    /// * `evm` - Instance of the evm execution
    pub fn execute(&self, evm: &mut Evm) -> Result<(), &'static str> {
        match self {
            Opcode::STOP => Err("STOP"),
            Opcode::ADD => add(evm),
            Opcode::MUL => mul(evm),
            Opcode::PUSH1 => push1(evm),
        }
    }
}

/// ADD execution: [..., b, a] -> [..., a + b]
fn add(evm: &mut Evm) -> Result<(), &'static str> {
    let a = evm.stack_pop()?;
    let b = evm.stack_pop()?;
    evm.stack_push(a.wrapping_add(b));
    evm.log(&format!("ADD: {} + {} => {}", a, b, a.wrapping_add(b)));
    evm.pc += 1;
    Ok(())
}

/// MUL execution: [..., b, a] -> [..., a * b]
fn mul(evm: &mut Evm) -> Result<(), &'static str> {
    let a = evm.stack_pop()?;
    let b = evm.stack_pop()?;
    evm.stack_push(a.wrapping_mul(b));
    evm.log(&format!("MUL: {} * {} => {}", a, b, a.wrapping_mul(b)));
    evm.pc += 1;
    Ok(())
}

/// PUSH1 execution: -> [..., u8]
///
/// # Arguments
///
/// * `byte` - The byte to push on the stack
fn push1(evm: &mut Evm) -> Result<(), &'static str> {
    let start = evm.pc + 1;
    let end = start + 1;
    let value = evm.code[start..end]
        .iter()
        .fold(0, |acc, &x| acc * 256 + x as u32);
    evm.stack_push(value);
    evm.log(&format!("PUSH1({})", value));
    evm.pc = end;
    Ok(())
}

/// Parse bytecode from it's hex string
///
/// # Arguments
///
/// * `bytecode_str` - An hexadecimal string (start with 0x) corresponding to the bytecode
///
/// # Returns
///
/// * An option containing the parsed bytecode in a Vec of u8
pub fn parse_bytecode(bytecode_str: &str) -> Result<Vec<u8>, &str> {
    let bytecode_str = bytecode_str.trim_start_matches("0x");
    let bytecode = hex::decode(bytecode_str).map_err(|_| "Invalid bytecode")?;
    Ok(bytecode)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_opcode_parsing() {
        // TODO .. test hex to Opcode
    }
}
