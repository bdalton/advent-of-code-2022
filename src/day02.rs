use crate::read_input;
use anyhow::{anyhow, Context};

pub fn solution() -> anyhow::Result<(usize, usize)> {
    let input = read_input("day02.txt")?;

    let mut line_number = 1;
    let mut phase1_accum = 0;
    let mut phase2_accum = 0;

    for line in input {
        let (column1, column2) =
            parse(line.as_str()).context(format!("parsing line {line_number}"))?;
        let (phase1_score, phase2_score) =
            turn_scores(column1, column2).context(format!("line {line_number}"))?;
        phase1_accum += phase1_score;
        phase2_accum += phase2_score;
        line_number += 1;
    }
    Ok((phase1_accum, phase2_accum))
}

fn parse(line: &str) -> anyhow::Result<(char, char)> {
    let mut chars = line.chars();
    let column1 = chars
        .next()
        .ok_or_else(|| anyhow!("missing entry in first column"))?;
    let column2 = chars
        .skip(1)
        .next()
        .ok_or_else(|| anyhow!("missing entry in second column"))?;

    Ok((column1, column2))
}

fn turn_scores(column1: char, column2: char) -> anyhow::Result<(usize, usize)> {
    let opponents_action = Action::try_from(column1).context("parsing opponent's action")?;
    let your_phase1_action = Action::try_from(column2).context("parsing your action")?;

    let phase2_outcome = Outcome::try_from(column2).context("parsing desired outcome")?;
    let your_phase2_action = phase2_outcome.with(opponents_action);

    let phase1_score = score(opponents_action, your_phase1_action);
    let phase2_score = score(opponents_action, your_phase2_action);

    Ok((phase1_score, phase2_score))
}

#[inline]
fn score(opponents_action: Action, your_action: Action) -> usize {
    let outcome = Outcome::from(opponents_action, your_action);
    (your_action as usize) + (outcome as usize)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Action {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, Copy, Clone)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl Action {
    /// the action that will lose against `self`
    fn inferior_action(self) -> Action {
        use Action::*;
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    /// the action that will win against self
    fn superior_action(self) -> Action {
        use Action::*;
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }
}

impl Outcome {
    /// the action needed to achieve `self's` outcome given the opponent's action.
    fn with(&self, opponents_action: Action) -> Action {
        use Outcome::*;
        match self {
            Lose => opponents_action.inferior_action(),
            Draw => opponents_action,
            Win => opponents_action.superior_action(),
        }
    }

    fn from(opponents_action: Action, your_action: Action) -> Outcome {
        use Outcome::*;
        if your_action == opponents_action {
            Draw
        } else if your_action == opponents_action.inferior_action() {
            Lose
        } else if your_action == opponents_action.superior_action() {
            Win
        } else {
            unreachable!()
        }
    }
}

impl TryFrom<char> for Action {
    type Error = anyhow::Error;

    fn try_from(c: char) -> anyhow::Result<Action> {
        use Action::*;
        match c {
            'A' | 'X' => Ok(Rock),
            'B' | 'Y' => Ok(Paper),
            'C' | 'Z' => Ok(Scissors),
            _ => Err(anyhow!("invalid action '{c}'")),
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = anyhow::Error;

    fn try_from(c: char) -> anyhow::Result<Outcome> {
        use Outcome::*;
        match c {
            'X' => Ok(Lose),
            'Y' => Ok(Draw),
            'Z' => Ok(Win),
            _ => Err(anyhow!("invalid outcome '{c}'")),
        }
    }
}
