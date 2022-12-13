use std::{
    io::{BufRead, Write},
    path::PathBuf,
};

use anyhow::{Context, Result};
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
    in_path: PathBuf,

    #[arg(short, long, default_value = "cedict.csv")]
    out_path: PathBuf,
}

fn get_entries(path: &PathBuf) -> Result<impl Iterator<Item = CedictEntry>> {
    let f =
        std::fs::File::open(path).with_context(|| format!("could not read file `{:?}`", path))?;
    let reader = std::io::BufReader::new(f);

    Ok(reader
        .lines()
        .flatten()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| line.parse::<CedictEntry>().ok()))
}

fn main() -> Result<()> {
    let args = Args::parse();

    let f = std::fs::File::create(&args.out_path)
        .with_context(|| format!("could not create file `{:?}`", &args.out_path))?;
    let mut writer = std::io::BufWriter::new(f);

    // entries for traditional chinese
    for entry in get_entries(&args.in_path)? {
        writeln!(writer, "{}", entry.to_traditional_csv_line())?;
    }

    // entries for simplified chinese
    for entry in get_entries(&args.in_path)?.filter(|e| e.has_simplified()) {
        writeln!(writer, "{}", entry.to_simplified_csv_line())?;
    }

    Ok(())
}
