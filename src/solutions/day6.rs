use crate::utils::input_reader::read_input_as_string;
use std::str::FromStr;

const DAY: u8 = 6;

pub fn solve() {
    let input = read_input_as_string(DAY);
    let part_one_solution = solve_part_one(&input);
    println!("Day 6, part one: {part_one_solution}");
    let part_two_solution = solve_part_two(&input);
    println!("Day 6, part two: {part_two_solution}");
}

fn solve_part_one(input: &String) -> i64 {
    let lines_count = input.lines().count();
    let grid: Vec<Vec<i64>> = input
        .lines()
        .take(lines_count - 1)
        .map(|it| read_numbers(it))
        .collect();
    let operations = read_operations(input.lines().last().unwrap());

    let mut counter = 0i64;
    for (idx, op) in operations.into_iter().enumerate() {
        if op == '+' {
            let mut curr = 0i64;
            for line in &grid {
                curr += line[idx];
            }
            counter += curr;
        } else {
            let mut curr = 1i64;
            for line in &grid {
                curr *= line[idx];
            }
            counter += curr;
        }
    }
    counter
}

fn solve_part_two(input: &String) -> i64 {
    let lines_count = input.lines().count();
    let grid: Vec<Vec<i64>> = input
        .lines()
        .take(lines_count - 1)
        .map(|it| read_numbers(it))
        .collect();
    let positions = get_operator_positions(input.lines().last().unwrap());
    let operations = read_operations(input.lines().last().unwrap());
    let alignments = get_alignments(input, positions, operations.len());
    let grid = rearrange(&grid, &alignments);

    let mut counter = 0i64;
    for (idx, op) in operations.into_iter().enumerate() {
        if op == '+' {
            let mut curr = 0i64;
            for num in &grid[idx] {
                curr += num;
            }
            counter += curr;
        } else {
            let mut curr = 1i64;
            for num in &grid[idx] {
                curr *= num;
            }
            counter += curr;
        }
    }
    counter
}

fn read_numbers(line: &str) -> Vec<i64> {
    line.split(" ")
        .filter(|it| !it.is_empty())
        .map(|it| i64::from_str(it).unwrap())
        .collect()
}

fn read_operations(line: &str) -> Vec<char> {
    line.split(" ")
        .filter(|it| !it.is_empty())
        .map(|it| char::from_str(it).unwrap())
        .collect()
}

fn rearrange(grid: &Vec<Vec<i64>>, alignments: &Vec<bool>) -> Vec<Vec<i64>> {
    let mut buckets = vec![Vec::new(); grid.first().unwrap().len()];
    for line in grid {
        for (idx, num) in line.iter().enumerate() {
            buckets[idx].push(*num);
        }
    }

    let mut result = Vec::new();
    for (idx, bucket) in buckets.iter().enumerate() {
        let bucket: Vec<String> = bucket.iter().map(|it| it.to_string()).collect();
        let num_count = bucket.iter().map(|it| it.len()).max().unwrap();

        let mut vec = vec![String::new(); num_count];

        for num in bucket {
            let mut chars = num.chars();
            for i in 0..num_count {
                if alignments[idx] && (num_count as i32 - num.len() as i32 - i as i32) > 0 {
                    continue;
                }
                if !alignments[idx] && num.len() < i + 1 {
                    continue;
                }

                let char = chars.next().unwrap();
                vec[i].push_str(&char.to_string());
            }
        }

        let vec = vec.iter().map(|it| i64::from_str(it).unwrap()).collect();
        result.push(vec);
    }
    result
}

fn get_operator_positions(line: &str) -> Vec<usize> {
    line.chars()
        .enumerate()
        .filter(|(_, c)| *c != ' ')
        .map(|(i, _)| i)
        .collect()
}

// false mean left-aligned, true -> right-aligned
fn get_alignments(
    input: &str,
    operator_positions: Vec<usize>,
    operations_count: usize,
) -> Vec<bool> {
    let mut alignments = vec![false; operations_count];
    for (i, pos) in operator_positions.iter().enumerate() {
        for line in input.lines() {
            for (idx, char) in line.chars().enumerate() {
                if idx == *pos && char == ' ' {
                    alignments[i] = true
                }
            }
        }
    }
    alignments
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_solve_part_one() {
        let input = INPUT.to_string();
        let actual = solve_part_one(&input);
        assert_eq!(4277556, actual);
    }

    #[test]
    fn test_solve_part_two() {
        let input = INPUT.to_string();
        let actual = solve_part_two(&input);
        assert_eq!(3263827, actual);
    }

    #[test]
    fn test_read_numbers() {
        let input = "123 328  51 64";
        let actual = read_numbers(input);
        assert_eq!(vec![123, 328, 51, 64], actual);
    }

    #[test]
    fn test_read_operations() {
        let input = "*   +   *   + ";
        let actual = read_operations(input);
        assert_eq!(vec!['*', '+', '*', '+'], actual);
    }

    #[test]
    fn test_get_operator_positions() {
        let input = "*   +   *   + ";
        let actual = get_operator_positions(input);
        assert_eq!(vec![0, 4, 8, 12], actual);
    }
}
