use std::process;
use std::collections::HashMap;

type LanternfishAge = u8;
fn next_day_part_1(all_fish: &mut Vec<LanternfishAge>) {
    let mut new_count = 0;
    all_fish.iter_mut().for_each(|lanternfish| {
        if *lanternfish == 0 {
            *lanternfish = 6;
            new_count += 1;
        }
        else { *lanternfish -= 1; }
    });
    all_fish.reserve(new_count);
    for _ in 0..new_count { all_fish.push(8); }
}

struct LanternfishCollection(HashMap<LanternfishAge, u64>);
impl LanternfishCollection {
    fn new() -> Self {
        let mut ret_val = LanternfishCollection(HashMap::with_capacity(9));
        for i in 0..=8 { ret_val.0.insert(i, 0); }
        ret_val
    }

    fn next_day(&mut self) {
        let temp = *self.0.get(&0).unwrap();
        for key in 0..=7 {
            *self.0.get_mut(&key).unwrap() = *self.0.get(&(key+1)).unwrap();
        }
        *self.0.get_mut(&8).unwrap() = temp;
        *self.0.get_mut(&6).unwrap() += temp;
    }

    fn fish_count(&self) -> u64 {
        self.0.values().fold(0, |acc, count| acc + *count)
    }
}

pub fn day6_main(file_data: &str) -> (usize, Vec<LanternfishAge>, u64) {
    // Part 1
    let mut all_lanternfish = file_data
        .split(',').enumerate()
        .map(|(i, c)| {
            c.parse::<LanternfishAge>().map_err(|e| (i, e))
        })
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|(i, e)| {
            eprintln!("Error parsing the age of lanternfish {}! :", i);
            eprintln!("{}", e);
            process::exit(1);
        });
    for _ in 1..=18 { next_day_part_1(&mut all_lanternfish); }
    let day_18_snapshot = all_lanternfish.clone();
    for _ in 19..=80 { next_day_part_1(&mut all_lanternfish); }
    let day_80_total = all_lanternfish.len();
    println!("There will be {} lanternfish after 80 days.", day_80_total);


    // Part 2
    let mut all_lanternfish = LanternfishCollection::new();
    file_data.split(',').enumerate()
        .map(|(i, c)| {
            let num = c.parse::<LanternfishAge>().map_err(|_| i)?;
            if num > 8 { return Err(i); }
            Ok(num)
        })
        .for_each(|x| {
            let age = x.unwrap_or_else(|i| {
                eprintln!("Error parsing the age of lanternfish {}! :", i);
                eprintln!("The age must be an integer in 0..=8!");
                process::exit(1);
            });
            *all_lanternfish.0.get_mut(&age).unwrap() += 1;
        });
    for _ in 1..=256 { all_lanternfish.next_day(); }
    let day_256_total = all_lanternfish.fish_count();
    println!("There will be {} lanternfish after 256 days.", day_256_total);

    (day_80_total, day_18_snapshot, day_256_total)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_data = "3,4,3,1,2";
        let string_of_day_18_lanternfish = "6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8"
            .split(',')
            .map(|c| c.parse::<LanternfishAge>().unwrap())
            .collect::<Vec<_>>();
        let (day_80_total, day_18_snapshot, day_256_total) = day6_main(test_data);
        assert_eq!(day_18_snapshot, string_of_day_18_lanternfish, "Not the same string of lanterfish on day 18. :(");
        assert_eq!(day_80_total, 5934, "Number of lanternfish on day 80 incorrect!");
        assert_eq!(day_256_total, 26984457539, "Number of lanternfish on day 256 incorrect!");
    }
}



