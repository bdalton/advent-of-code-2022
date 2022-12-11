use anyhow::Context;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::{Path, PathBuf};

pub fn read_input(p: impl AsRef<Path>) -> anyhow::Result<Vec<String>> {
    let input_path = PathBuf::from("inputs").join(p);
    let input = File::open(input_path).context("opening input")?;
    let output = io::BufReader::new(input)
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .context("reading input")?;
    Ok(output)
}
