mod day01;
mod day02;
mod day03;
mod shell;
mod util;

pub use shell::Shell;
pub use util::read_input;

fn main() {
    Shell::new()
        .solve(1, day01::solution)
        .solve(2, day02::solution)
        .solve(3, day03::solution);
}
