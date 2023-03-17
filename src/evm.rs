use crate::opcode::Opcode;
use crate::cli::wait_for_press;

/// The EVM interpreter, responsible for executing Ethereum bytecode.
pub struct Evm {
    pub stack: Vec<u32>,
    pub memory: Vec<u8>,
    pub pc: usize,
    pub code: Vec<u8>,
    // pub storage
    // pub calldata
    // pub returndata
    // Used for logs
    pub logs: bool,
    pub step_by_step: bool,
}

impl Evm {
    /// Creates a new EVM instance with the given bytecode and logging configuration.
    ///
    /// # Arguments
    ///
    /// * `code` - A vector of bytes representing the Contract bytecode.
    /// * `logs` - A boolean flag indicating whether to enable logging for EVM operations.
    /// * `step_by_step` - A boolean flag indicating whether to enable step_by_step execution
    pub fn new(code: Vec<u8>, logs: bool, step_by_step: bool) -> Self {
        Self {
            stack: Vec::new(),
            memory: Vec::new(),
            pc: 0,
            code,
            logs,
            step_by_step
        }
    }

    /// Step in the execution. Get and execute the next opcode.
    pub fn step(&mut self) -> Result<(), &'static str> {
        let opcode = self.next_opcode()?;

        if self.step_by_step {
            wait_for_press(&'p');
        }

        self.log("┅┅┅┅┅┅┅┅┅┅");
        self.log_progress();
        opcode.execute(self)?;
        self.log_stack();
        Ok(())
    }

    /// Get the next opcode in the current execution
    ///
    /// # Returns
    ///
    /// * An option containing the Opcode
    pub fn next_opcode(&self) -> Result<Opcode, &'static str> {
        self.code
            .get(self.pc)
            .and_then(Opcode::from)
            .ok_or("Invalid opcode")
    }

    /// Add value to stack
    /// [...] -> [..., value]
    ///
    /// # Arguments
    ///
    /// * `value` - the value to add
    pub fn stack_push(&mut self, value: u32) {
        self.stack.push(value);
    }

    /// Pop value from stack 
    /// [..., x] -> [...]
    ///
    /// # Returns
    ///
    /// * An option containing the top value of the stack
    pub fn stack_pop(&mut self) -> Result<u32, &'static str> {
        self.stack.pop().ok_or("Stack underflow")
    }

    /// Execute the bytecode
    pub fn execute(&mut self) -> Result<(), &'static str> {
        if self.step_by_step {
            self.log("Press 'p' to proceed to the next step.");
        }
        loop {
            match self.step() {
                Ok(()) => {}
                Err("STOP") => return Ok(()),
                Err(e) => return Err(e),
            }
        }
    }

    /// helper function to log a msg
    ///
    /// # Arguments
    ///
    /// * `msg` - the message to log
    pub fn log(&self, msg: &str) {
        if self.logs {
            println!("{msg}");
        }
    }

    /// helper function to log the stack and execution
    fn log_stack(&self) {
        if !self.logs {
            return;
        }

        let separator = "├────┤";
        let empty_slot = "│    │";

        // Print the stack header
        println!("Stack:");
        println!("╭────╮");

        // Print the stack slots
        for i in (0..self.stack.len()).rev() {
            println!("│ {:02x} │", self.stack[i]);
            println!("{}", separator);
        }

        // Print the remaining empty slots up to a predefined maximum
        let max_stack_size = 1;
        for _ in self.stack.len()..max_stack_size {
            println!("{}", empty_slot);
            println!("{}\n", separator);
        }
    }

    /// helper function to log bytecode and point to current byte opcode
    fn log_progress(&self) {
        if !self.logs {
            return;
        }

        // Print the bytecode header
        println!("Bytecode:");

        // Print the bytecode in its raw form
        for (index, byte) in self.code.iter().enumerate() {
            print!("{:02x} ", byte);
            if (index + 1) % 16 == 0 {
                println!();
            }
        }
        println!();

        // Print an arrow pointing to the current opcode position
        for _ in 0..(self.pc * 3) {
            print!(" ");
        }
        println!("↑");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_bytecode() {
        let bytecode = vec![
            0x60, 0x05, // PUSH 5
            0x60, 0x06, // PUSH 6
            0x01,       // ADD
            0x60, 0x02, // PUSH 2
            0x02,       // MUL
            0x00,       // STOP
        ];

        let mut evm = Evm::new(bytecode, true, false);
        assert!(evm.execute().is_ok());
        assert_eq!(evm.stack, vec![22]);
    }
}
