use crate::utils::input_reader::read_input_as_string;
use std::str::FromStr;

const DAY: u8 = 2;

pub fn solve() {
    let input = read_input_as_string(DAY);
    let part_one_solution = solve_both(&input, is_invalid_one);
    println!("Day 2, part one: {part_one_solution}");
    let part_two_solution = solve_both(&input, is_invalid_two);
    println!("Day 2, part two: {part_two_solution}");
}

fn solve_both(input: &String, is_invalid_fn: fn(i64) -> bool) -> i64 {
    let mut invalid_ids = Vec::new();
    let ranges = input.split(",");
    for range in ranges {
        let start_and_end: Vec<&str> = range.split("-").collect();
        assert_eq!(2, start_and_end.len());
        let start =
            i64::from_str(start_and_end[0]).expect("unable to parse from string slice to i64");
        let end =
            i64::from_str(start_and_end[1]).expect("unable to parse from string slice to i64");
        for num in start..=end {
            if is_invalid_fn(num) {
                invalid_ids.push(num);
            }
        }
    }
    invalid_ids.into_iter().sum()
}

fn is_invalid_one(num: i64) -> bool {
    let num = num.to_string();
    let first = &num[..num.len() / 2];
    let second = &num[num.len() / 2..];
    first == second
}

fn is_invalid_two(num: i64) -> bool {
    let num = num.to_string();
    for i in 1..num.len() {
        if num.len() % i != 0 {
            continue;
        }
        let mut parts = Vec::new();
        let mut curr = 0;
        while curr < num.len() {
            let part = &num[curr..curr + i];
            parts.push(part);
            curr += i;
        }
        let first_part = parts[0];
        if parts.into_iter().all(|part| part == first_part) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    const INPUT: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_solve_part_one() {
        let input = &INPUT.to_string();
        let actual = solve_both(input, is_invalid_one);
        assert_eq!(1227775554, actual);
    }

    #[test]
    fn test_solve_part_two() {
        let input = &INPUT.to_string();
        let actual = solve_both(input, is_invalid_two);
        assert_eq!(4174379265, actual);
    }

    #[test]
    fn test_is_invalid_one() {
        let input_to_expected = HashMap::from([
            (11, true),
            (99, true),
            (1010, true),
            (123123, true),
            (101, false),
        ]);
        for (input, expected) in input_to_expected {
            let actual = is_invalid_one(input);
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_is_invalid_two() {
        let input_to_expected = HashMap::from([
            (11, true),
            (99, true),
            (1010, true),
            (123123, true),
            (1111111, true),
            (101, false),
        ]);
        for (input, expected) in input_to_expected {
            let actual = is_invalid_two(input);
            assert_eq!(expected, actual, "should be {expected} for {input}");
        }
    }
}
