# Light EVM Interpreter

A simple Ethereum Virtual Machine (EVM) interpreter implemented in Rust.
It was made mostly to get better at rust.

## Features

- Executes supported Ethereum bytecode
- Supports basic arithmetic operations and stack manipulation
- Optional logging of EVM operations

## Usage

Build the project with:

```bash
cargo build
```

To run the EVM interpreter with a bytecode input and logging enabled, use the following command:

```bash
$ cargo run -- -l -b "0x6005600401"
```

Run the tests with:

```bash
cargo test
```

Open the documentation with:

```bash
cargo doc --open
```

## Supported OPCODE

- ADD
- MUL
- PUSH1
- STOP
