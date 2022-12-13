use std::io::{BufRead, Write};

use anyhow::Result;
use clap::Parser;

use crate::cedict_entry::CedictEntry;

mod cedict_entry;
mod hsk;

/// Program to create chinese dictionary for cedict
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the dictionary
    #[arg(short, long, default_value = "data/cedict_ts.u8")]
    in_path: std::path::PathBuf,

    #[arg(short, long, default_value = "cedict.tsv")]
    out_path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let f = std::fs::File::open(&args.in_path)?;
    let reader = std::io::BufReader::new(f);

    let entries = reader
        .lines()
        .flatten()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| line.parse::<CedictEntry>().ok())
        .map(|entry| entry.to_string());

    let f = std::fs::File::create(&args.out_path)?;
    let mut writer = std::io::BufWriter::new(f);
    for entry in entries {
        writeln!(writer, "{}", entry)?;
    }

    Ok(())
}
