use anyhow::anyhow;
use arrayvec::ArrayVec;
use colored::Colorize;
use std::fmt::Display;
use std::str::{Chars, FromStr};

pub struct Shell {
    day_bg: Colour,
    day_fg: Colour,
    answer_bg: Colour,
    label_fg: Colour,
    answer_fg: Colour,
}

impl Shell {
    pub fn new() -> Shell {
        Self::default()
    }

    pub fn solve<S, T, U>(&mut self, day_index: usize, solution: S) -> &mut Shell
    where
        S: Fn() -> anyhow::Result<(T, U)>,
        T: Display,
        U: Display,
    {
        self.emit_day_span(day_index);
        self.spacer();
        match solution() {
            Err(e) => {
                self.emit_error_span(e);
            }
            Ok((answer1, answer2)) => {
                self.emit_answer_span("  phase 1: ", answer1.to_string());
                self.spacer();
                self.emit_answer_span("  phase 2: ", answer2.to_string());
                println!();
            }
        }

        self
    }

    fn spacer(&self) {
        print!(" ")
    }

    fn emit_day_span(&self, day_index: usize) {
        Self::span(
            format!("  day {:02}  ", day_index).as_str(),
            self.day_bg,
            self.day_fg,
        );
    }

    fn emit_error_span(&self, e: anyhow::Error) {
        Self::span(format!(" {:<53} ", e.to_string()), self.day_bg, self.day_fg);
        let mut it = e.source();
        while let Some(inner) = it {
            println!();
            print!("           ");
            Self::span(
                format!(" {:<53} ", inner.to_string()),
                self.day_bg,
                self.day_fg,
            );
            it = inner.source();
        }
        println!();
    }

    fn emit_answer_span(&self, label: &'static str, answer: String) {
        Self::span(label, self.answer_bg, self.label_fg);
        Self::span(
            format!("  {:>12}  ", answer),
            self.answer_bg,
            self.answer_fg,
        );
    }

    fn span(text: impl AsRef<str>, bg: Colour, fg: Colour) {
        print!(
            "{}",
            text.as_ref()
                .on_truecolor(bg.r, bg.g, bg.b)
                .truecolor(fg.r, fg.g, fg.b)
        );
    }
}

impl Default for Shell {
    fn default() -> Self {
        let day_bg = Colour::from_str("#C21010").expect("invalid day background colour");
        let day_fg = Colour::from_str("#FFFDE3").expect("invalid day foreground colour");
        let answer_bg = Colour::from_str("#446A46").expect("invalid answer background colour");
        let label_fg = Colour::from_str("#82A284").expect("invalid label foreground colour");
        let answer_fg = Colour::from_str("#EFF5F5").expect("invalid answer foreground colour");

        Shell {
            day_bg,
            day_fg,
            answer_bg,
            label_fg,
            answer_fg,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl FromStr for Colour {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Colour> {
        let mut it = s.chars();
        if it.next() != Some('#') {
            return Err(anyhow!("malformed colour"));
        }
        let r = hex_pair(&mut it)?;
        let g = hex_pair(&mut it)?;
        let b = hex_pair(&mut it)?;

        Ok(Colour { r, g, b })
    }
}

/// parses a pair of hex digits from the input and returns it as a u8
fn hex_pair(input: &mut Chars) -> anyhow::Result<u8> {
    let pair = input
        .take(2)
        .map(|c| c.to_digit(16))
        .collect::<ArrayVec<_, 2>>();
    if pair.len() != 2 {
        Err(anyhow!("truncated input"))
    } else if pair.iter().any(Option::is_none) {
        Err(anyhow!("expected hex digits"))
    } else {
        let c0 = pair[0].unwrap() as u8;
        let c1 = pair[1].unwrap() as u8;
        Ok(c0 * 16 + c1)
    }
}
