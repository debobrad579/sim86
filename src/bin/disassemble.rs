use clap::Parser;
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use sim86::types::instruction::Instruction;

fn main() -> std::io::Result<()> {
    let mut args = Args::parse();

    let output_path = args.output.take().unwrap_or_else(|| {
        let mut path = args.path.clone();
        path.set_extension("asm");
        path
    });

    let data: Box<[u8]> = std::fs::read(args.path)?.into();

    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);

    let len = data.len();
    let mut count = len;
    while count > 0 {
        let i = len - count;

        let instruction = Instruction::from(&data[i..]);

        writer.write_all(&format!("{}\n", instruction).into_bytes())?;

        count -= instruction.size as usize;
    }

    writer.flush()?;

    Ok(())
}

#[derive(Parser)]
#[clap()]
struct Args {
    path: PathBuf,

    #[clap(short, long)]
    output: Option<PathBuf>,
}
