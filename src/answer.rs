use std::fmt;

pub type AdventResult = anyhow::Result<(Answer, Answer)>;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Answer {
    Unavailable,
    Usize(usize),
}

impl Default for Answer {
    fn default() -> Self {
        Answer::Unavailable
    }
}

impl From<usize> for Answer {
    fn from(input: usize) -> Self {
        Answer::Usize(input)
    }
}

impl From<Option<usize>> for Answer {
    fn from(input: Option<usize>) -> Self {
        match input {
            None => Answer::Unavailable,
            Some(n) => Answer::Usize(n),
        }
    }
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Answer::Unavailable => write!(f, "n/a"),
            Answer::Usize(n) => write!(f, "{}", n),
        }
    }
}
