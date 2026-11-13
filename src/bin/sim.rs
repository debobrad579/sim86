use clap::Parser;
use std::{fs, path::PathBuf, process::Command};

use sim86::sim::Sim86;

fn main() {
    let args = Args::parse();

    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");

    let compiled_path = temp_dir.path().join("compiled");

    Command::new("nasm")
        .arg("-o")
        .arg(&compiled_path)
        .arg(args.path.into_os_string())
        .output()
        .expect("Failed to execute nasm");

    let program: Box<[u8]> = fs::read(&compiled_path)
        .expect("Failed to read binary")
        .into();

    let mut sim86 = Sim86::default();
    sim86.load(&program);
    sim86.run(args.print, args.output);
}

#[derive(Parser)]
#[clap()]
struct Args {
    path: PathBuf,

    #[clap(short, long, default_value_t = false)]
    print: bool,

    #[clap(short, long)]
    output: Option<PathBuf>,
}
