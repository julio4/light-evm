use light_evm::{evm::Evm, opcode::parse_bytecode, cli::create_cli};

fn main() {
    let matches = create_cli().get_matches();
    let bytecode_str = matches.get_one::<String>("bytecode").unwrap();
    let logging_enabled = *matches.get_one::<bool>("verbose").unwrap_or(&false);
    let step_enabled = *matches.get_one::<bool>("step").unwrap_or(&false);

    let bytecode = parse_bytecode(bytecode_str).expect("Failed to parse bytecode");
    let mut evm = Evm::new(bytecode, logging_enabled, step_enabled);

    match evm.execute() {
        Ok(()) => println!("Execution completed successfully"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
