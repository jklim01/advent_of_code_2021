use std::fs;
use std::process;
mod day18;

fn main() {
    // read input data from file
    let filepath = "input_data/day18_input.txt";
    let file_data = fs::read_to_string(filepath)
        .unwrap_or_else(|e| {
            eprintln!("Error reading file \"{}\"! : {}", filepath, e);
            process::exit(1);
        });

    day18::day18_main(&file_data);
}
