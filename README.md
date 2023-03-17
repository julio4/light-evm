# Light EVM

A simple Ethereum Virtual Machine (EVM) implementation in Rust.
This is a WIP flawed version, mostly to get better at rust and have a better understanding of the EVM.

## Features

- Executes supported Ethereum bytecode
- Supports basic arithmetic operations and stack manipulation
- Optional logging of EVM operations (Program Counter and Stack vizualisation)

## Usage

Build the project with:

```bash
cargo build --release
```

To run the EVM interpreter with a bytecode input and logging enabled, use the following command:

```bash
$ cargo run -- -v -b "0x600560040100"
```

You can also activate step by step mode to block on each PC incrementation with the flag `-s`:

```bash
$ cargo run -- -v -s -b "0x600560040100"
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
- ... more to come
