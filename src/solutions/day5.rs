use crate::utils::input_reader::read_input_as_string;
use std::collections::HashSet;
use std::str::FromStr;

const DAY: u8 = 5;

pub fn solve() {
    let input = read_input_as_string(DAY);
    let part_one_solution = solve_part_one(&input);
    println!("Day 5, part one: {part_one_solution}");
    let part_two_solution = solve_part_two(&input);
    println!("Day 5, part two: {part_two_solution}");
}

fn solve_part_one(input: &String) -> i64 {
    let fresh = get_fresh(input);
    let available = get_available(input);

    let mut counter = 0i64;
    for a in available {
        for (start, end) in &fresh {
            if a >= *start && a <= *end {
                counter += 1;
                break;
            }
        }
    }
    counter
}

fn solve_part_two(input: &String) -> i64 {
    let fresh = get_fresh(input);
    let mut ranges: Vec<(i64, i64)> = Vec::new();

    for (curr_start, curr_end) in fresh {
        let mut overlapping = Vec::new();
        for r in ranges.clone() {
            if curr_start <= r.1 && curr_end >= r.0 {
                overlapping.push(r);
            }
        }

        if overlapping.is_empty() {
            ranges.push((curr_start, curr_end));
        } else {
            ranges = ranges
                .into_iter()
                .filter(|it| !overlapping.contains(&it))
                .collect();
            let mut min: i64 = curr_start;
            let mut max: i64 = curr_end;
            for o in overlapping {
                if o.0 < min {
                    min = o.0;
                }
                if o.1 > max {
                    max = o.1;
                }
            }

            ranges.push((min, max));
        }
    }

    let mut counter = 0;
    for (start, end) in ranges {
        counter += end - start + 1;
    }
    counter
}

fn get_fresh(input: &String) -> Vec<(i64, i64)> {
    let mut fresh = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        let line = line.trim();
        let parts: Vec<i64> = line
            .split("-")
            .map(|line| i64::from_str(line).unwrap())
            .collect();

        assert_eq!(2, parts.len());
        fresh.push((parts[0], parts[1]));
    }
    fresh
}

fn get_available(input: &String) -> HashSet<i64> {
    let mut available = HashSet::new();
    let mut should_add = false;
    for line in input.lines() {
        if should_add {
            let line = line.trim();
            let num = i64::from_str(line).unwrap();
            available.insert(num);
        }

        if line.is_empty() {
            should_add = true;
        }
    }
    available
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_solve_part_one() {
        let input = INPUT.to_string();
        let actual = solve_part_one(&input);
        assert_eq!(3, actual);
    }

    #[test]
    fn test_solve_part_two() {
        let input = INPUT.to_string();
        let actual = solve_part_two(&input);
        assert_eq!(14, actual);
    }
}
