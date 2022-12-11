use crate::util::*;
use anyhow::{anyhow, Context};
use std::str::FromStr;

pub fn solution() -> anyhow::Result<(String, String)> {
    let input = read_input("day05.txt")?;
    let (stacks, cursor) = Stacks::parse(&input)?;
    let commands = input[cursor..]
        .iter()
        .map(|line| Command::from_str(line.as_str()))
        .collect::<Result<Vec<_>, _>>()
        .context("parsing commands")?;

    let phase1_answer = phase1(stacks.clone(), &commands).context("phase 1")?;
    let phase2_answer = phase2(stacks, &commands).context("phase 2")?;

    Ok((phase1_answer, phase2_answer))
}

fn phase1(mut stacks: Stacks, commands: &[Command]) -> anyhow::Result<String> {
    for Command {
        count,
        source,
        dest,
    } in commands
    {
        let mut crates = stacks
            .pickup_crates(*source, *count)
            .context(anyhow!("picking up {count} crate(s) from stack {source}"))?;
        crates.reverse();
        stacks
            .drop_crates(*dest, crates)
            .context(anyhow!("dropping {count} crates onto stack {dest}"))?;
    }

    Ok(stacks.top())
}

fn phase2(mut stacks: Stacks, commands: &[Command]) -> anyhow::Result<String> {
    for Command {
        count,
        source,
        dest,
    } in commands
    {
        let crates = stacks
            .pickup_crates(*source, *count)
            .context("picking up {count} crate(s) from stack {source}")?;
        stacks
            .drop_crates(*dest, crates)
            .context("dropping {count} crates onto stack {dest}")?;
    }

    Ok(stacks.top())
}

#[derive(Debug, Clone)]
struct Stacks(Vec<Vec<char>>);

#[derive(Debug, Clone)]
struct Command {
    count: usize,
    source: usize,
    dest: usize,
}

impl Stacks {
    pub fn parse(input: &[String]) -> anyhow::Result<(Stacks, usize)> {
        let mut tmp = Vec::new();
        let mut cursor = 0;
        while input[cursor].contains('[') {
            // the pattern is regular, the letters (if any) have index 1 modulo 4
            // Let's keep Some(ch) at those positions if there is a character there
            // and None if there isn't.
            let line = input[cursor]
                .chars()
                .enumerate()
                .filter_map(|(pos, ch)| {
                    if pos % 4 != 1 {
                        None
                    } else if ch.is_ascii_alphabetic() {
                        Some(Some(ch))
                    } else {
                        Some(None)
                    }
                })
                .collect::<Vec<_>>();
            tmp.push(line);
            cursor += 1;
        }

        // the number of stacks we need matches the number of elements in the bottom row.
        // create a stack for each element there.
        let mut stacks = tmp
            .last()
            .ok_or_else(|| anyhow!("no stacks available"))?
            .iter()
            .map(|_| Vec::<char>::new())
            .collect::<Vec<_>>();

        // now we go in reverse and transpose the elements.
        // the bottom row will be processed first.
        while let Some(line) = tmp.pop() {
            for (stack_index, value) in line.iter().enumerate() {
                if let Some(ch) = value {
                    stacks[stack_index].push(*ch);
                }
            }
        }

        Ok((Stacks(stacks), cursor + 2))
    }

    fn count(&self, stack_index: usize) -> Option<usize> {
        if stack_index == 0 || stack_index > self.0.len() {
            None
        } else {
            Some(self.0[stack_index - 1].len())
        }
    }

    fn pickup_crates(&mut self, source: usize, count: usize) -> anyhow::Result<Vec<char>> {
        let n = match self.count(source) {
            None => return Err(anyhow!("invalid source stack {source}")),
            Some(n) if n < count => {
                return Err(anyhow!("source stack {source} doesn't have {count} crates"))
            }
            Some(n) => n,
        };

        let crates = self.0[source - 1].drain(n - count..).collect::<Vec<_>>();
        Ok(crates)
    }

    fn drop_crates(&mut self, dest: usize, crates: Vec<char>) -> anyhow::Result<()> {
        if self.count(dest).is_none() {
            return Err(anyhow!("invalid dest stack {dest}"));
        }
        self.0[dest - 1].extend(crates);
        Ok(())
    }

    fn top(&self) -> String {
        self.0
            .iter()
            .map(|stack| stack.last().cloned().unwrap_or(' '))
            .collect()
    }
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> anyhow::Result<Command> {
        let input = input.as_bytes();
        let input = match_str(input, b"move ")?;
        let (count, input) = parse_number(input).context("parsing number of containers")?;
        let input = match_str(input, b" from ")?;
        let (source, input) = parse_number(input).context("parsing source stack")?;
        let input = match_str(input, b" to ")?;
        let (dest, _input) = parse_number(input).context("parsing destination stack")?;

        Ok(Command {
            count,
            source,
            dest,
        })
    }
}
