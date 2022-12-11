use crate::read_input;
use anyhow::{anyhow, Context};
use itertools::Itertools;

pub fn solution() -> anyhow::Result<(usize, usize)> {
    let input = read_input("day06.txt")?;

    let phase1_answer = phase1(input[0].as_bytes()).context("phase 1")?;
    let phase2_answer = phase2(input[0].as_bytes()).context("phase 2")?;

    Ok((phase1_answer, phase2_answer))
}

fn is_unique(input: &[u8]) -> bool {
    let n = input
        .iter()
        .fold(0, |acc, x| {
            let idx = *x - ('a' as u8);
            acc | (1u32 << idx)
        })
        .count_ones() as usize;

    n == input.len()
}

fn phase1(input: &[u8]) -> anyhow::Result<usize> {
    let (pos, _) = input
        .windows(4)
        .find_position(|window| is_unique(*window))
        .ok_or_else(|| anyhow!("no marker found"))?;
    Ok(pos + 4)
}

fn phase2(input: &[u8]) -> anyhow::Result<usize> {
    let (pos, _) = input
        .windows(14)
        .find_position(|window| is_unique(*window))
        .ok_or_else(|| anyhow!("no marker found"))?;
    Ok(pos + 14)
}
