use std::process;

enum Command {
    Forward(i8),
    Down(i8)
}

pub fn day2_main(file_data: &str) {
    // parse lines to Command iterator
    let commands = file_data.lines().enumerate().map(|(i, line)| {
        let tokens: Vec<_> = line.split_whitespace().collect();
        let line_number = i + 1;

        // check number of tokens
        if tokens.len() != 2 {
            eprintln!("Parse Error (line {}): each line can only have 2 tokens", i+1);
            process::exit(1);
        }

        // parse the units to move
        let move_units = tokens[1].parse::<i8>().unwrap_or_else(|e| {
            eprintln!("Parse Error (line {}): ParseIntError{{ {} }}", i+1, e);
            process::exit(1);
        });

        // parse command
        match tokens[0] {
            "forward" => Command::Forward(move_units),
            "down" => Command::Down(move_units),
            "up" => Command::Down(-move_units),
            _ => {
                eprintln!("Parse Error (line {}): invalid command", i+1);
                process::exit(1);
            }
        }
    });


    // Part 1
    let mut result: [i32; 2] = [0, 0];
    for command in commands.clone() {
        match command {
            Command::Forward(move_units) => result[0] += move_units as i32,
            Command::Down(move_units) => result[1] += move_units as i32,
        }
    }
    println!("Part 1:");
    println!("The final position of the submarine is {:?}.", result);
    println!("The answer is {}!", result[0] * result[1]);


    // Part 1
    let mut result: [i32; 2] = [0, 0];
    let mut aim = 0;
    for command in commands {
        match command {
            Command::Forward(move_units) => {
                result[0] += move_units as i32;
                result[1] += aim * (move_units as i32);
            }
            Command::Down(move_units) => aim += move_units as i32
        }
    }
    println!("\nPart 2:");
    println!("The final position of the submarine is {:?}.", result);
    println!("The answer is {}!", result[0] * result[1]);
}