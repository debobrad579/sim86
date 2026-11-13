# Sim86

An 8086 simulator written in Rust.

## Binaries

- **`disasseble`**: disassembles a compiled 8086 binary
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
cargo run --release --bin disasseble -- <path/to/binary> [--output <path>]
```

Run a program in the simulator:

```sh
cargo run --release --bin sim -- <path/to/assembly> [--output <path>] [--print]
```

## Example: TicTacToe

```sh
cargo run --release --bin sim -- tictactoe.asm
```

## Supported Instructions

| Category          | Instructions |
|-------------------|--------------|
| Data transfer     | `mov`, `xchg`, `xlat`, `lea`, `lds`, `les`, `lahf`, `sahf` |
| Stack             | `push`, `pop`, `pushf`, `popf`, `call`, `ret`, `retf` |
| Arithmetic        | `add`, `adc`, `sub`, `sbb`, `cmp`, `inc`, `dec`, `neg`, `cbw` |
| Logic             | `and`, `or`, `xor`, `not`, `test` |
| Control flow      | `jmp`, `loop`, `loopz`, `loopnz`, `jcxz`, and all conditional jumps |
| I/O               | `in`, `out` (simulated ports: `00h` = stdin, `01h` = stdout) |
| Processor control | `clc`, `cmc`, `stc`, `cld`, `std`, `cli`, `sti`, `hlt`, `wait` |

## Acknowledgements

Built as part of Casey Muratori's [Performance-Aware Programming Series](https://www.computerenhance.com/p/table-of-contents) (Computer, Enhance!)
