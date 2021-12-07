use std::vec;

struct SlidingWindow {
    prev1: u16,
    prev2: u16,
    remaining: vec::IntoIter<u16>
}
impl SlidingWindow {
    fn new(measurements: Vec<u16>) -> SlidingWindow {
        let mut remaining =  measurements.into_iter();
        return SlidingWindow {
            prev1: remaining.next().unwrap_or_default(),
            prev2: remaining.next().unwrap_or_default(),
            remaining: remaining
        }
    }
}
impl Iterator for SlidingWindow {
    type Item = u16;
    fn next(&mut self) -> Option<Self::Item> {
        self.remaining.next().map(|x| {
            let window_sum = self.prev1 + self.prev2 + x;
            self.prev1 = self.prev2;
            self.prev2 = x;
            return window_sum;
        })

    }
}

pub fn day1_main(file_data: &str) {
    // parse to vector of u16 values
    let measurements: Vec<u16> = file_data.trim().lines()
        .enumerate().map(|(i, line)| {
            line.parse::<u16>().unwrap_or_else(|e| {
                eprintln!("ParseIntError on line {}: {}", i+1, e);
                std::process::exit(1);
            })
        }).collect();


    // Part 1
    let mut measurement_iter = measurements.iter();
    let mut increment_count = 0;
    let mut prev  = match measurement_iter.next() {
        Some(val) => val,
        None => {
            println!("File is empty!");
            return;
        }
    };

    for current in measurement_iter {
        if current > prev {
            increment_count += 1;
        }
        prev = current;
    }
    println!("{} measurements are larger than the previous.", increment_count);


    // Part 2
    let mut window_iter = SlidingWindow::new(measurements);
    increment_count = 0;
    let mut prev_sum = match window_iter.next() {
        Some(val) => val,
        None => {
            println!("File is empty!");
            return;
        }
    };
    for current_sum in window_iter {
        if current_sum > prev_sum {
            increment_count += 1;
        }
        prev_sum = current_sum;
    }
    println!("{} measurements windows are larger than the previous.", increment_count);
}
