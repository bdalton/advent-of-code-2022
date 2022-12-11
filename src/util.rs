use anyhow::{anyhow, Context};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::RangeInclusive;
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

pub fn parse_range_inclusive(input: &[u8]) -> anyhow::Result<(RangeInclusive<usize>, &[u8])> {
    let (start, input) = parse_number(input).context("parsing lower bound")?;
    let input = expect_ch(input, b'-').context("parsing range")?;
    let (end, input) = parse_number(input).context("parsing upper bound")?;

    Ok((start..=end, input))
}

pub fn parse_number(input: &[u8]) -> anyhow::Result<(usize, &[u8])> {
    let mut cursor = 0;
    let mut accum = 0;
    while cursor < input.len() && input[cursor].is_ascii_digit() {
        let digit = (input[cursor] as char).to_digit(10).unwrap() as usize;
        accum = accum * 10 + digit;
        cursor += 1;
    }
    if cursor > 0 {
        Ok((accum, &input[cursor..]))
    } else {
        Err(anyhow!("expected a number"))
    }
}

pub fn expect_ch(input: &[u8], ch: u8) -> anyhow::Result<&[u8]> {
    if input.is_empty() || input[0] != ch {
        Err(anyhow!("expected character '{}'", ch as char))
    } else {
        Ok(&input[1..])
    }
}

pub fn match_str<'a>(input: &'a [u8], expected: &'_ [u8]) -> anyhow::Result<&'a [u8]> {
    let n = expected.len();
    if input.len() < n || &input[..n] != expected {
        Err(anyhow!(""))
    } else {
        Ok(&input[n..])
    }
}
