use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

pub fn run() -> io::Result<()> {
    let mut result = 0;
    let file_path = Path::new("./src/days/day_1/calibration_document.txt");

    let file = File::open(&file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        result += calibration_number(line.unwrap());
    }
    println!("result: {}", result);
    Ok(())
}

fn find_first_numeric_char(line: &str) -> char {
    for char in line.chars() {
        if char.is_numeric() {
            return char;
        }
    }
    return '\0';
}

fn calibration_number(file_line: String) -> i32 {
    let mut calibration_number = String::from("");

    let first_numeric_char = find_first_numeric_char(&file_line);
    let reversed_line = file_line.chars().rev().collect::<String>();
    let last_numeric_char = find_first_numeric_char(&reversed_line);

    calibration_number.push(first_numeric_char);
    calibration_number.push(last_numeric_char);

    return calibration_number.parse::<i32>().unwrap();
}
