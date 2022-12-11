use crate::read_input;
use anyhow::{anyhow, Context};
use std::slice;

pub fn solution() -> anyhow::Result<(usize, usize)> {
    let input = read_input("day07.txt")?;
    let lines = input
        .iter()
        .map(|line| Line::parse(line.as_str()))
        .collect::<Result<Vec<_>, _>>()
        .context("pre-processing")?;

    let (phase1_answer, total_used, _) = phase1(lines.iter()).context("phase 1")?;

    let unused_space = 70000000 - total_used;
    let space_needed = 30000000 - unused_space;
    let (phase2_answer, _, _) = phase2(space_needed, lines.iter()).context("phase 2")?;

    Ok((phase1_answer, phase2_answer))
}

fn phase1<'a, 'b>(
    mut lines: slice::Iter<'b, Line<'a>>,
) -> anyhow::Result<(usize, usize, slice::Iter<'b, Line<'a>>)> {
    use Line::*;
    let mut total_dir_size = 0;
    let mut partial_answer = 0;
    loop {
        match lines.next() {
            Some(CmdChangeDirRoot) => return phase1(lines),
            None | Some(CmdChangeDirOut) => {
                if total_dir_size < 100000 {
                    partial_answer += total_dir_size;
                }
                return Ok((partial_answer, total_dir_size, lines));
            }
            Some(CmdChangeDirIn(_)) => {
                let (subtree_answer, dir_size, remaining_lines) = phase1(lines)?;
                total_dir_size += dir_size;
                partial_answer += subtree_answer;
                lines = remaining_lines;
            }
            Some(CmdList) | Some(OutDir(_)) => {}
            Some(OutFile(file_size, _)) => total_dir_size += file_size,
        }
    }
}

fn phase2<'a, 'b>(
    space_needed: usize,
    mut lines: slice::Iter<'b, Line<'a>>,
) -> anyhow::Result<(usize, usize, slice::Iter<'b, Line<'a>>)> {
    use Line::*;
    let mut total_dir_size = 0;
    let mut partial_answer = usize::MAX;
    loop {
        match lines.next() {
            Some(CmdChangeDirRoot) => return phase2(space_needed, lines),
            None | Some(CmdChangeDirOut) => {
                if total_dir_size >= space_needed {
                    partial_answer = partial_answer.min(total_dir_size);
                }
                return Ok((partial_answer, total_dir_size, lines));
            }
            Some(CmdChangeDirIn(_)) => {
                let (subtree_answer, dir_size, remaining_lines) = phase2(space_needed, lines)?;
                total_dir_size += dir_size;
                partial_answer = partial_answer.min(subtree_answer);
                lines = remaining_lines;
            }
            Some(CmdList) | Some(OutDir(_)) => {}
            Some(OutFile(file_size, _)) => total_dir_size += file_size,
        }
    }
}

#[derive(Debug)]
enum Line<'a> {
    CmdChangeDirRoot,
    CmdChangeDirIn(&'a str),
    CmdChangeDirOut,
    CmdList,
    OutFile(usize, &'a str),
    OutDir(&'a str),
}

impl<'a> Line<'a> {
    fn parse(input: &str) -> anyhow::Result<Line> {
        let mut tokens = input.split_whitespace();
        let tok0 = tokens
            .next()
            .ok_or_else(|| anyhow!("couldn't determine line type"))?;
        match tok0 {
            "$" => {
                let cmd = tokens.next().ok_or_else(|| anyhow!("missing command"))?;
                match cmd {
                    "cd" => {
                        let cd_arg = tokens.next().ok_or_else(|| anyhow!("missing arg for cd"))?;
                        match cd_arg {
                            "/" => Ok(Line::CmdChangeDirRoot),
                            ".." => Ok(Line::CmdChangeDirOut),
                            _ => Ok(Line::CmdChangeDirIn(cd_arg)),
                        }
                    }
                    "ls" => Ok(Line::CmdList),
                    _ => Err(anyhow!("invalid command")),
                }
            }
            "dir" => {
                let dir_name = tokens.next().ok_or_else(|| anyhow!("missing dir name"))?;
                Ok(Line::OutDir(dir_name))
            }
            _ => {
                let size = tok0.parse::<usize>().context("expected integer")?;
                let filename = tokens.next().ok_or_else(|| anyhow!("missing file name"))?;
                Ok(Line::OutFile(size, filename))
            }
        }
    }
}
