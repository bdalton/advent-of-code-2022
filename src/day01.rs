use crate::read_input;
use anyhow::{anyhow, Context};

pub fn solution() -> anyhow::Result<(usize, usize)> {
    let input: Vec<Option<usize>> = read_input("day01.txt")
        .context("reading input")?
        .iter()
        .enumerate()
        .map(parse)
        .collect::<Result<Vec<_>, _>>()
        .context("pre-processing input")?;

    let mut elf_calories = Vec::new();
    let mut accumulator = 0;
    for line in input.into_iter().chain([None]) {
        match line {
            None => {
                elf_calories.push(accumulator);
                accumulator = 0;
            }
            Some(n) => accumulator += n,
        }
    }
    let num_elves = elf_calories.len();
    let (_, _, suffix) = elf_calories.select_nth_unstable(num_elves - 4);

    let phase1_answer = *suffix
        .iter()
        .max()
        .ok_or_else(|| anyhow!("not enough elves"))?;
    let phase2_answer = suffix.iter().sum::<usize>();

    Ok((phase1_answer, phase2_answer))
}

/// blank lines are mapped to Ok(None) and lines with numbers to Some(n)
/// A parsing error maps to Err(...)
fn parse((line_index, line): (usize, &String)) -> anyhow::Result<Option<usize>> {
    if line.is_empty() {
        Ok(None)
    } else {
        let n = line
            .parse()
            .context(format!("failed to parse line {}", line_index + 1))?;
        Ok(Some(n))
    }
}
