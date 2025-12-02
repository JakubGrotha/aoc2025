use crate::solutions::{day1, day2};
use std::env::args;
use std::process;
use std::str::FromStr;

mod solutions {
    pub mod day1;
    pub mod day2;
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
        _ => panic!("invalid puzzle number!"),
    };
    process::exit(0);
}
