use std::env;

use sim86::types::Instruction;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: sim86_decode <file_path>");
        std::process::exit(1);
    }

    let file_path = &args[1];

    let data: Box<[u8]> = std::fs::read(file_path)?.into();

    let len = data.len();
    let mut count = len;
    while count > 0 {
        let i = len - count;

        let instruction = Instruction::from(&data[i..]);

        println!("{}", instruction);

        count -= instruction.size;
    }

    Ok(())
}
