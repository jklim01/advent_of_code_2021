use std::fs;
use std::process;
mod day20;

fn main() {
    // read input data from file
    let filepath = "input_data/day20_input.txt";
    let file_data = fs::read_to_string(filepath).unwrap_or_else(|e| {
        eprintln!("Error reading file \"{}\"! : {}", filepath, e);
        process::exit(1);
    });

    day20::day20_main(&file_data);
}
