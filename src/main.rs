use std::io;
mod days;

fn main() {
    println!("Advent of Code\nSelect the day to run: ");
    let mut selected_day = String::new();
    io::stdin().read_line(&mut selected_day).expect("");

    match selected_day.trim() {
        "1" => days::day_1::run().unwrap(),
        _ => println!("Day not implemented"),
    }
}
