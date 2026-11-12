# Sim86

An 8086 simulator written in Rust.

## Binaries

- **`decode`**: disassembles a compiled 8086 binary
- **`sim`**: executes an 8086 program in the simulator

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (stable, 2024 edition)
- [NASM](https://www.nasm.us/) to assemble `.asm` files

### Build

```sh
cargo build --release
```

### Usage

Disassemble a binary:

```sh
cargo run --release --bin decode -- <path/to/binary>
```

Run a program in the simulator:

```sh
cargo run --release --bin sim -- <path/to/assembly>
```

## Example: TicTacToe

```sh
cargo run --release --bin sim -- tictactoe.asm
```

## Supported Instructions

| Category       | Instructions |
|----------------|--------------|
| Data transfer  | `mov` |
| Stack          | `push`, `pop`, `call`, `ret`, `retf` |
| Arithmetic     | `add`, `adc`, `sub`, `sbb`, `cmp`, `inc`, `dec`, `neg` |
| Logic          | `and`, `or`, `xor`, `not`, `test` |
| Control flow   | `jmp`, `loop`, `loopz`, `loopnz`, `jcxz`, and all conditional jumps |
| I/O            | `in`, `out` (simulated ports: `00h` = stdin, `01h` = stdout) |
| Misc           | `hlt`, `lea`, `cbw` |

## Acknowledgements

Built as part of Casey Muratori's [Performance-Aware Programming Series](https://www.computerenhance.com/p/table-of-contents) (Computer, Enhance!)
