use std::{fs::File, io::Read, path::PathBuf};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// A single .c source file to compile
    source_file: PathBuf,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let mut source_file = File::open(args.source_file)?;
    let mut source_code = String::new();
    source_file.read_to_string(&mut source_code)?;
    let preprocessed = c_urchin::preprocess(&source_code)?;
    let parsed = c_urchin::parse(preprocessed)?;
    dbg!(parsed);
    Ok(())
}
