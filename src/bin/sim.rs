use std::{env, fs, process::Command};

use sim86::sim::Sim86;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: sim86_sim <file_path>");
        std::process::exit(1);
    }

    let file_path = &args[1];

    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");

    let compiled_path = temp_dir.path().join("compiled");

    Command::new("nasm")
        .arg("-o")
        .arg(&compiled_path)
        .arg(file_path)
        .output()
        .expect("Failed to execute nasm");

    let program: Box<[u8]> = fs::read(&compiled_path)
        .expect("Failed to read binary")
        .into();

    let mut sim86 = Sim86::default();
    sim86.load(&program);
    sim86.run(false);
}
