mod answer;
mod day01;
mod shell;
mod util;

pub use answer::{AdventResult, Answer};
pub use shell::Shell;
pub use util::read_lines;

fn main() {
    Shell::new().solve(1, day01::solution);
}
