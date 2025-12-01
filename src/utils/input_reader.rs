use std::fs;

pub fn read_input_as_string(day: i8) -> String {
    let path = format!("./input/day{}.txt", day);
    fs::read_to_string(path)
        .expect(format!("Failed to find and parse input file for day {}", day).as_str())
}
