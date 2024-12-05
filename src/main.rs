use clap::Parser;
use cli::CliArgs;

mod cli;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

fn main() {
    CliArgs::parse().run();
}
