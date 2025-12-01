use crate::utils::input_fetcher::fetch_input_as_string;
use std::str::FromStr;

const DAY: i8 = 1;

pub fn solve() {
    let input = fetch_input_as_string(DAY);
    let part_one_solution = solve_part_one(&input);
    println!("Day 1, part one: {part_one_solution}");
    let part_two_solution = solve_part_two(&input);
    println!("Day 2, part two: {part_two_solution}");
}

fn solve_part_one(input: &String) -> i32 {
    let mut counter = 0i32;
    let mut idx = 50i32;
    for line in input.lines() {
        if line.starts_with("R") {
            let value =
                i32::from_str(&line.replace("R", "")).expect("unable to parse to i32 from string");
            idx += value;
        } else {
            let value =
                i32::from_str(&line.replace("L", "")).expect("unable to parse to i32 from string");
            idx -= value;
        }
        if idx % 100 == 0 {
            counter += 1;
        }
    }
    counter
}

fn solve_part_two(input: &String) -> i32 {
    let mut counter = 0i32;
    let mut idx = 50i32;
    for line in input.lines() {
        if line.starts_with("R") {
            let value =
                i32::from_str(&line.replace("R", "")).expect("unable to parse to i32 from string");
            let count = count(idx, value);
            counter += count;
            idx += value;
        } else {
            let value =
                i32::from_str(&line.replace("L", "")).expect("unable to parse to i32 from string");
            let count = count(idx, value * -1);
            counter += count;
            idx -= value;
        }
    }
    counter
}

fn count(idx: i32, value: i32) -> i32 {
    let mut counter = 0i32;
    if value >= 0 {
        for curr in idx + 1..=idx + value {
            if curr % 100 == 0 {
                counter += 1;
            }
        }
    } else {
        for curr in idx + value..idx {
            if curr % 100 == 0 {
                counter += 1;
            }
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn test_solve_part_one() {
        let actual = solve_part_one(&String::from(INPUT));
        assert_eq!(3, actual);
    }

    #[test]
    fn test_solve_part_two() {
        let actual = solve_part_two(&String::from(INPUT));
        assert_eq!(6, actual);
    }
}
