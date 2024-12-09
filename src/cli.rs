use clap::Parser;

macro_rules! day {
    ($name:ident) => {
        (
            crate::$name::part_1_answer as fn() -> String,
            crate::$name::part_2_answer as fn() -> String,
        )
    };
}

#[allow(clippy::type_complexity)]
const SOLUTIONS: &[(fn() -> String, fn() -> String)] = &[
    day!(day_1),
    day!(day_2),
    day!(day_3),
    day!(day_4),
    day!(day_5),
    day!(day_6),
    day!(day_7),
    day!(day_8),
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
