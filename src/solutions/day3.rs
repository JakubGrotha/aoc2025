use crate::utils::input_reader::read_input_as_string;
use std::str::FromStr;

const DAY: u8 = 3;

pub fn solve() {
    let input = read_input_as_string(DAY);
    let part_one_solution = solve_part_one(&input);
    println!("Day 3, part one solution: {part_one_solution}");
    let part_two_solution = solve_part_two(&input);
    println!("Day 3, part two solution: {part_two_solution}");
}

fn solve_part_one(input: &String) -> i32 {
    let mut result = 0i32;
    for line in input.lines() {
        let digits: Vec<i32> = line
            .chars()
            .map(|char| char.to_string())
            .map(|char| i32::from_str(&char).expect("unable to parse from string to i32"))
            .collect();

        let mut max = 0;
        let mut max_idx = 0;
        for (idx, digit) in digits
            .clone()
            .into_iter()
            .take(digits.len() - 1)
            .enumerate()
        {
            if digit > max {
                max = digit;
                max_idx = idx;
            }
        }
        let second = &digits[max_idx + 1..].iter().max().unwrap();

        let num = i32::from_str(&(max.to_string() + &second.to_string())).unwrap();
        result += num;
    }
    result
}

fn solve_part_two(input: &String) -> u64 {
    let mut result = 0u64;
    for line in input.lines() {
        let digits: Vec<u64> = line
            .chars()
            .map(|char| char.to_string())
            .map(|char| u64::from_str(&char).expect("unable to parse from string to i32"))
            .collect();

        let mut d: Vec<String> = Vec::new();
        let mut prev_idx = 0;
        for i in 0..12 {
            let mut max = 0u64;
            let mut max_idx = 0;

            for (idx, digit) in digits.iter().enumerate() {
                if i != 0 && idx <= prev_idx {
                    continue;
                }
                if idx > digits.len() - (12 - i) {
                    continue;
                }
                if *digit > max {
                    max = *digit;
                    max_idx = idx;
                }
                prev_idx = max_idx;
            }

            d.push(max.to_string());
        }

        let num = u64::from_str(&d.join("")).unwrap();
        result += num;
    }
    result
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_solve_part_one() {
        let input = INPUT.to_string();
        let actual = solve_part_one(&input);
        assert_eq!(357, actual);
    }

    #[test]
    fn test_solve_part_two() {
        let input = INPUT.to_string();
        let actual = solve_part_two(&input);
        assert_eq!(3121910778619, actual);
    }
}
