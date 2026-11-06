use std::env;

use sim86::sim::Sim86;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: sim86_sim <file_path>");
        std::process::exit(1);
    }

    let file_path = &args[1];

    let program = std::fs::read(file_path)?;

    let mut sim86 = Sim86::default();
    sim86.load(&program);

    sim86.run(true);

    Ok(())
}
