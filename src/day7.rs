use std::collections::BTreeMap;
use std::cmp::Ordering;

type CrabPos = u16;

#[inline]
fn fuel_formula_p1(distance: CrabPos) -> u32 {
    distance as u32
}
#[inline]
fn fuel_formula_p2(distance: CrabPos) -> u32 {
    (distance as u32 * (distance + 1) as u32) / 2
}
fn find_ideal_alignment(all_pos: &[CrabPos], min_max_pos: (CrabPos, CrabPos),
    fuel_calculator: fn(CrabPos) -> u32) -> Option<(CrabPos, u32)> {
    if all_pos.is_empty() { return None; }
    let mut min_fuel = u32::MAX;
    let mut ideal_pos = 0;

    for i in min_max_pos.0..min_max_pos.1 {
        let current = all_pos.iter().fold(0, |acc, pos| {
            let abs_distance = match *pos > i {
                true => pos - i,
                false => i - pos
            };
            acc + (fuel_calculator(abs_distance) as u32)
        });
        if current < min_fuel {
            min_fuel = current;
            ideal_pos = i;
        }
    }
    Some((ideal_pos, min_fuel))
}

struct CrabPositionMap {
    pos_map: BTreeMap<CrabPos, u32>,
    total: u32
}
impl CrabPositionMap {
    fn new() -> Self {
        CrabPositionMap {
            pos_map: BTreeMap::new(), total: 0
        }
    }

    fn add_crabs(&mut self, pos: CrabPos, count: u32) {
        *self.pos_map.entry(pos).or_insert(0) += count;
        self.total += count;
    }

    #[inline]
    fn calc_fuel(&self, align_pos: CrabPos, fuel_formula: fn(CrabPos) -> u32) -> u32 {
        self.pos_map.iter().fold(0, |acc, (pos, count)| {
            let distance = match *pos > align_pos {
                true => pos - align_pos,
                false => align_pos - pos
            };
            acc + count * fuel_formula(distance)
        })
    }
    #[inline]
    fn calc_fuel_p1(&self, align_pos: CrabPos) -> u32 {
        self.calc_fuel(align_pos, |distance| distance as u32)
    }
    #[inline]
    fn calc_fuel_p2(&self, align_pos: CrabPos) -> u32 {
        self.calc_fuel(align_pos, |distance| (distance as u32 * (distance + 1) as u32) / 2)
    }

    fn find_ideal_pos_p1(&self) -> Option<(CrabPos, CrabPos, u32)> {
        if self.total == 0 { return None; }
        let mut crab_count = 0;
        let mut start = CrabPos::MAX;
        let mut end = 0;
        for (pos, count) in self.pos_map.iter() {   // BTreeMap lets us iterate pos from left to right
            crab_count += count;
            // cost function slope to the right of pos = 2*crab_count - self.total
            match (2*crab_count).cmp(&self.total) {     // don't compute slope to avoid casting to signed integer
                Ordering::Less => (),   // -ve slope, minimum not reached yet
                Ordering::Equal => {    // 0 slope, minimum occurs on an interval, and the left edge is found
                    start = *pos;
                },
                Ordering::Greater => {  // +ve slope, end of minimum zone
                    if *pos < start {   // check if start was set, if not, the minimum occurs at only 1 point
                        start = *pos;
                    }
                    end = *pos;
                    break;
                }
            }
        }
        Some((start, end, self.calc_fuel_p1(start)))
    }

    fn find_ideal_pos_p2(&self) -> Option<(CrabPos, u32)> {
        if self.total == 0 { return None; }
        let mean = self.pos_map.iter()
            .fold(0, |acc, (pos, count)| acc + (*pos as u32)*(*count)) as f32
            / self.total as f32;
        let rounded_mean = mean.round() as CrabPos;
        let candidates = [rounded_mean-1, rounded_mean, rounded_mean+1];

        candidates.iter()
            .map(|align_pos| (*align_pos, self.calc_fuel_p2(*align_pos)))
            .min_by(|x, y| {
                x.1.cmp(&y.1)
            })
    }
}

pub fn day7_main(file_data: &str) -> ((CrabPos, u32), (CrabPos, u32)) {
    // Method 1: Brute force search
    let all_positions = file_data
        .split(',').enumerate()
        .map(|(i, s)| s.parse::<CrabPos>().unwrap_or_else(|e| {
            eprintln!("Error parsing the position of crab {}! : ", i);
            eprintln!("{}", e);
            panic!();
        }))
        .collect::<Vec<CrabPos>>();
    let min_max_pos = all_positions.iter()
        .fold((CrabPos::MAX, CrabPos::MIN), |mut acc, pos| {
            if *pos < acc.0 { acc.0 = *pos; }
            if *pos > acc.1 { acc.1 = *pos; }
            acc
        });
    let (ideal_pos_p1, min_fuel_p1) =
        find_ideal_alignment(&all_positions, min_max_pos, fuel_formula_p1)
        .expect("Error finding part 1 answer using method 1!");
    let (ideal_pos_p2, min_fuel_p2) =
        find_ideal_alignment(&all_positions, min_max_pos, fuel_formula_p2)
        .expect("Error finding part 2 answer using method 1!");


    // Method 2: Optimized by doing some math
    let mut all_positions = CrabPositionMap::new();
    file_data.split(',').enumerate()
        .for_each(|(i, s)| {
            let pos = s.parse::<CrabPos>().unwrap_or_else(|e| {
                eprintln!("Error parsing the position of crab {}! : ", i);
                eprintln!("{}", e);
                panic!();
            });
            all_positions.add_crabs(pos, 1);
        });
    let (p1_min_left, p1_min_right, p1_min_fuel) = all_positions.find_ideal_pos_p1()
        .expect("Error finding part 1 answer using method 2!");
    let (p2_min_pos, p2_min_fuel) = all_positions.find_ideal_pos_p2()
        .expect("Error finding part 2 answer using method 2!");


    // Check if different methods give same result
    if !((p1_min_left <= ideal_pos_p1) && (ideal_pos_p1 <= p1_min_right) && (min_fuel_p1 == p1_min_fuel)) {
        eprintln!("Part 1:");
        eprintln!("Method 1 Answer: Position {}, Fuel {}", ideal_pos_p1, min_fuel_p1);
        eprintln!("Method 2 Answer: Position {}..={}, Fuel {}", p1_min_left, p1_min_right, p1_min_fuel);
        panic!("Part 1 answers don't match!");
    }
    if !((p2_min_pos == ideal_pos_p2) && (min_fuel_p2 == p2_min_fuel)) {
        eprintln!("Part 2:");
        eprintln!("Method 1 Answer: Position {}, Fuel {}", ideal_pos_p2, min_fuel_p2);
        eprintln!("Method 2 Answer: Position {}, Fuel {}", p2_min_pos, p2_min_fuel);
        panic!("Part 2 answers don't match!");
    }

    // Output answers
    print!("[Part 1] The horizontal position of {}..={} ", p1_min_left, p1_min_right);
    println!("gives the minimum fuel consumption of {}.", p1_min_fuel);
    print!("[Part 2] The horizontal position of {} ", p2_min_pos);
    println!("gives the minimum fuel consumption of {}.", p2_min_fuel);
    ((ideal_pos_p1, min_fuel_p1), (ideal_pos_p2, min_fuel_p2))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_data = "16,1,2,0,4,2,7,1,2,14";
        let (part1, part2) = day7_main(test_data);
        assert_eq!(part1, (2, 37));
        assert_eq!(part2, (5, 168));
    }
}