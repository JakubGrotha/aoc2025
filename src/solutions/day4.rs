use crate::utils::input_reader::read_input_as_string;

const DAY: u8 = 4;

pub fn solve() {
    let input = read_input_as_string(DAY);
    let part_one_solution = solve_part_one(&input);
    println!("Day 4, part one: {part_one_solution}");
    let part_two_solution = solve_part_two(&input);
    println!("Day 4, part two: {part_two_solution}");
}

fn solve_part_one(input: &String) -> i32 {
    let len = input.lines().count();
    let mut grid = vec![vec!['.'; len]; len];
    for (x, line) in input.lines().enumerate() {
        for (y, char) in line.chars().enumerate() {
            grid[x][y] = char;
        }
    }

    let mut counter = 0i32;
    for x in 0..grid.len() {
        for y in 0..grid.len() {
            let curr = grid[x][y];
            if curr == '@' && can_access(&grid, x, y) {
                counter += 1;
            }
        }
    }

    counter
}

fn solve_part_two(input: &String) -> i32 {
    let len = input.lines().count();
    let mut grid = vec![vec!['.'; len]; len];
    for (x, line) in input.lines().enumerate() {
        for (y, char) in line.chars().enumerate() {
            grid[x][y] = char;
        }
    }

    let mut counter = 0i32;
    let mut changed = true;
    while changed {
        changed = false;

        let mut removed = Vec::new();
        for x in 0..grid.len() {
            for y in 0..grid.len() {
                let curr = grid[x][y];
                if curr == '@' && can_access(&grid, x, y) {
                    removed.push((x, y));
                }
            }
        }
        counter += removed.len() as i32;

        if removed.len() > 0 {
            changed = true;
        }

        for (x, y) in removed {
            grid[x][y] = '.';
        }
    }

    counter
}

fn can_access(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let mut counter = 0u8;
    if x > 0 {
        if grid[x - 1][y] == '@' {
            counter += 1;
        }
        if y > 0 {
            if grid[x - 1][y - 1] == '@' {
                counter += 1;
            }
        }
        if y < grid.len() - 1 {
            if grid[x - 1][y + 1] == '@' {
                counter += 1;
            }
        }
    }
    if y > 0 {
        if grid[x][y - 1] == '@' {
            counter += 1;
        }

        if x < grid.len() - 1 {
            if grid[x + 1][y - 1] == '@' {
                counter += 1;
            }
        }
    }

    if x < grid.len() - 1 {
        if grid[x + 1][y] == '@' {
            counter += 1;
        }

        if y < grid.len() - 1 {
            if grid[x + 1][y + 1] == '@' {
                counter += 1;
            }
        }
    }

    if y < grid.len() - 1 {
        if grid[x][y + 1] == '@' {
            counter += 1;
        }
    }

    counter < 4
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_solve_part_one() {
        let input = INPUT.to_string();
        let actual = solve_part_one(&input);
        assert_eq!(13, actual);
    }

    #[test]
    fn test_solve_part_two() {
        let input = INPUT.to_string();
        let actual = solve_part_two(&input);
        assert_eq!(43, actual);
    }
}
