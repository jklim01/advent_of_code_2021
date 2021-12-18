use std::fs;
use std::process;
mod day12;

fn main() {
    // read input data from file
    let filepath = "input_data/day12_input.txt";
    let file_data = fs::read_to_string(filepath)
        .unwrap_or_else(|e| {
            eprintln!("Error reading file \"{}\"! : {}", filepath, e);
            process::exit(1);
        });

    day12::day12_main(&file_data);
}
