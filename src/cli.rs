use clap::Parser;

use crate::{day_1, day_2, day_3, day_4, day_5, day_6};

#[allow(clippy::type_complexity)]
const SOLUTIONS: &[(fn() -> String, fn() -> String)] = &[
    (
        day_1::part_1_answer as fn() -> String,
        day_1::part_2_answer as fn() -> String,
    ),
    (
        day_2::part_1_answer as fn() -> String,
        day_2::part_2_answer as fn() -> String,
    ),
    (
        day_3::part_1_answer as fn() -> String,
        day_3::part_2_answer as fn() -> String,
    ),
    (
        day_4::part_1_answer as fn() -> String,
        day_4::part_2_answer as fn() -> String,
    ),
    (
        day_5::part_1_answer as fn() -> String,
        day_5::part_2_answer as fn() -> String,
    ),
    (
        day_6::part_1_answer as fn() -> String,
        day_6::part_2_answer as fn() -> String,
    ),
];

#[derive(Debug, Parser)]
pub struct CliArgs {
    #[clap(long, short)]
    pub day: u8,
    #[clap(long, short)]
    pub part: u8,
}

impl CliArgs {
    pub fn run(&self) {
        assert!(self.day != 0);
        assert!(self.day as usize <= SOLUTIONS.len());

        let part_1 = match self.part {
            1 => true,
            2 => false,
            _ => panic!("`part` must be 1 or 2"),
        };

        let (f1, f2) = SOLUTIONS[self.day as usize - 1];
        let s = match part_1 {
            true => f1(),
            false => f2(),
        };
        println!("{s}");
    }
}
