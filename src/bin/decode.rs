use std::env;

use sim86::instruction::Instruction;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: sim86_decode <file_path>");
        std::process::exit(1);
    }

    let file_path = &args[1];

    let data: Box<[u8]> = std::fs::read(file_path).unwrap().into();

    let len = data.len();
    let mut count = len;
    while count > 0 {
        let i = len - count;
        let mut bytes = [0u8; 6];

        let bytes_left = count.min(6);

        bytes[..bytes_left].copy_from_slice(&data[i..i + bytes_left]);

        let instruction = Instruction::from(bytes);

        println!("{}", instruction);

        count -= instruction.size;
    }
}
