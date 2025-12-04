use crate::solutions::{day1, day2, day3, day4};
use std::env::args;
use std::str::FromStr;

mod solutions {
    pub mod day1;
    pub mod day2;
    pub mod day3;
    pub mod day4;
}

mod utils {
    pub mod input_reader;
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        panic!("You need to specify which puzzle you want to solve (1 - 12)");
    }
    let exercise_number =
        u8::from_str(&args[1]).expect("invalid argument - must be a positive integer");
    match exercise_number {
        1 => day1::solve(),
        2 => day2::solve(),
        3 => day3::solve(),
        4 => day4::solve(),
        _ => panic!("invalid puzzle number!"),
    };
}
