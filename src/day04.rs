use crate::util::*;
use anyhow::Context;
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
