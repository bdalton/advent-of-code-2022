use crate::read_input;
use anyhow::{anyhow, Context};
use std::ops::RangeInclusive;

pub fn solution() -> anyhow::Result<(usize, usize)> {
    let input = read_input("day04.txt")?
        .iter()
        .map(parse)
        .collect::<Result<Vec<_>, _>>()
        .context("pre-processing input")?;

    let phase1_score = input
        .iter()
        .filter(|(a, b)| subseteq(a, b) || subseteq(b, a))
        .count();
    let phase2_score = input
        .iter()
        .filter(|(a, b)| non_empty_intersection(a, b))
        .count();

    Ok((phase1_score, phase2_score))
}

fn parse(input: &String) -> anyhow::Result<(RangeInclusive<usize>, RangeInclusive<usize>)> {
    let input = input.as_bytes();
    let (first, input) = parse_range_inclusive(input).context("parsing first range")?;
    let input = expect_ch(input, b',')?;
    let (second, _input) = parse_range_inclusive(input).context("parsing second range")?;

    Ok((first, second))
}

fn parse_range_inclusive(input: &[u8]) -> anyhow::Result<(RangeInclusive<usize>, &[u8])> {
    let (start, input) = parse_number(input).context("parsing lower bound")?;
    let input = expect_ch(input, b'-').context("parsing range")?;
    let (end, input) = parse_number(input).context("parsing upper bound")?;

    Ok((start..=end, input))
}

fn parse_number(input: &[u8]) -> anyhow::Result<(usize, &[u8])> {
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

fn expect_ch(input: &[u8], ch: u8) -> anyhow::Result<&[u8]> {
    if input.is_empty() || input[0] != ch {
        Err(anyhow!("expected character '{}'", ch as char))
    } else {
        Ok(&input[1..])
    }
}

/// the range a is contained within b
fn subseteq(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
    b.contains(a.start()) && b.contains(a.end())
}

/// the range a is disjoint from the range b
fn disjoint(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
    a.end() < b.start() || b.end() < a.start()
}

fn non_empty_intersection(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
    !disjoint(a, b)
}
