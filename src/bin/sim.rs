use std::env;

use sim86::sim::Sim86;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: sim86_sim <file_path>");
        std::process::exit(1);
    }

    let file_path = &args[1];

    let program: Box<[u8]> = std::fs::read(file_path)?.into();

    let mut sim86 = Sim86::new(program);

    sim86.run(true);

    Ok(())
}
