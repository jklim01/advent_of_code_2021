use std::iter;
use std::collections::HashSet;

type Point = (u8, u8);
type Basin = HashSet<Point>;    // groups a set of points into a Basin

fn group_into_basins(height_map: &[Vec<u8>]) -> Vec<Basin> {
    let mut map = height_map.iter().enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().map(move |(j, &x)| ((i as u8, j as u8), x))
        })
        .filter_map(|(p, height)| {
            if height == 9 { None }
            else { Some(p) }
        })
        .collect::<HashSet<Point>>();

    let num_of_rows = height_map.len() as u8;
    let row_length = height_map[0].len() as u8;
    let mut basins = Vec::new();
    let mut stack = Vec::new();
    let mut current_basin;
    while let Some(&p) = map.iter().next() {
        current_basin = HashSet::new();
        stack.push(p);
        while let Some(p) = stack.pop() {
            map.remove(&p);
            current_basin.insert(p);
            if p.0 != 0             && map.contains(&(p.0-1, p.1)) { stack.push((p.0-1, p.1)); }    // up
            if p.0 != num_of_rows-1 && map.contains(&(p.0+1, p.1)) { stack.push((p.0+1, p.1)); }    // down
            if p.1 != 0             && map.contains(&(p.0, p.1-1)) { stack.push((p.0, p.1-1)); }    // left
            if p.1 != row_length-1  && map.contains(&(p.0, p.1+1)) { stack.push((p.0, p.1+1)); }    // right
        }
        basins.push(current_basin);
    }

    basins
}

// searches for minima along a line where the height at all points is given
fn find_local_minima_1d<'a>(mut height_map: std::slice::Iter<'a, u8>)
    -> Box<dyn Iterator<Item = usize> + 'a> {
    // the height_map is padded on the left and right with u8::MAX
    let mut prev = u8::MAX; // pad on the left
    let mut current = match height_map.next() {
        Some(x) => *x,
        None => u8::MAX   // default to max value, then empty vector will be returned
    };
    Box::new(height_map.chain(iter::repeat(&u8::MAX).take(1))   // pad on the right
        .enumerate()
        .filter_map(move |(i, &next)| {
            let ret_val = match (current < prev) && (current < next) {
                true => Some(i),
                false => None
            };
            prev = current;
            current = next;
            ret_val
        }))
}
// calculates the risk level as per the criteria
fn get_total_risk_level(height_map: &[Vec<u8>]) -> u16 {
    // find points which are minima in the horizontal direction
    let horizontal_minima_points = height_map.iter().enumerate()
        .map(|(i, row)| {
            iter::repeat(i).zip(find_local_minima_1d(row.iter()))
        })
        .flatten();

    // horizontal minima which are also vertical minima are the true minima
    let num_of_rows = height_map.len();
    let true_minima_points = horizontal_minima_points
        .filter(|(i, j)| {
            // compare with the points above and below (where they exist)
            if *i != 0 && height_map[*i-1][*j] <= height_map[*i][*j] { return false; }
            if *i != num_of_rows-1 && height_map[*i+1][*j] <= height_map[*i][*j] { return false; }
            true
        });

    true_minima_points.fold(0, |acc, (i, j)| {
        acc + 1 + height_map[i][j] as u16
    })
}


pub fn day9_main(file_data: &str) -> (u16, u32) {
    // Part 1
    let row_len = file_data.lines().next()
        .unwrap_or_else(|| {
            panic!("File is empty!");
        })
        .trim().chars().count();
    let height_map = file_data.lines().enumerate()
        .map(|(i, line)| {
            let ret_val = line.trim().chars().enumerate()
                .map(|(j, c)| c.to_digit(10).unwrap_or_else(|| {
                    panic!("Error parsing the height at position ({}, {}).", i, j);
                }) as u8)
                .collect::<Vec<_>>();
            if ret_val.len() != row_len {
                panic!("Error creating height map at line {}! All rows must have the same length.", i+1);
            }
            ret_val
        })
        .collect::<Vec<Vec<_>>>();

    let part1_answer = get_total_risk_level(&height_map);
    println!("[Part 1] The total risk level is {}.", part1_answer);

    // Part 2
    let basins = group_into_basins(&height_map);
    let mut basin_sizes = basins.iter().map(|basin| basin.len() as u32)
        .collect::<Vec<_>>();
    basin_sizes.sort_unstable();
    let part2_answer: u32 = basin_sizes.iter().rev().take(3).product();

    // let basin_map = p2_first_try::BasinMap::from_height_map(&height_map);
    // let mut basin_sizes = basin_map.basins.values()
    //     .map(|basin| basin.len() as u32).collect::<Vec<_>>();
    // basin_sizes.sort_unstable();
    // let part2_answer2: u32 = basin_sizes.iter().rev().take(3).product();
    // if part2_answer2 != part2_answer { panic!("Part 2 answers don't match!"); }
    println!("[Part 2] The product is {}.", part2_answer);

    (part1_answer, part2_answer)
}


#[allow(dead_code)]
// pretty terrible lol, but might as well keep it since it's got some ideas I'm not used to
mod p2_first_try {
    use std::collections::HashMap;
    use std::collections::BTreeSet;
    use std::collections::HashSet;
    type BasinID = u16;
    type Point = (u8, u8);
    type Basin = HashSet<Point>;    // groups a set of points into a Basin
    struct IDEqualitySet(Vec<BTreeSet<BasinID>>);
    // each hashset is a set of BasinID which all correspond to the same basin
    impl IDEqualitySet {
        fn new() -> Self {
            IDEqualitySet(Vec::new())
        }
        fn add_equality(&mut self, id1: BasinID, id2: BasinID) {
            if id1 == id2 { return; }
            let set1_index =  self.0.iter().position(|set| set.contains(&id1));
            let set2_index = self.0.iter().position(|set| set.contains(&id2));

            // both id's are in some set already
            if let (Some(mut index1), Some(index2)) = (set1_index, set2_index) {
                if index1 == index2 { return; }
                let set2 = self.0.remove(index2);
                if index2 < index1 { index1 -= 1; }
                self.0[index1].extend(set2);
            }
            // both id's don't belong in some set
            else if set1_index.is_none() && set2_index.is_none() {
                self.0.push([id1, id2].into_iter().collect());
            }
            // one of the id's is in some set
            else {
                if let Some(index2) = set2_index { self.0[index2].insert(id1); }
                else { self.0[set1_index.unwrap()].insert(id2); }
            }
        }
    }
    pub struct BasinMap {
        pub basins: HashMap<BasinID, Basin>,
        pub map: HashMap<Point, BasinID>,
    }
    // data structure allowing us to query BasinID by Point and Basin(set of Points) by BasinID
    impl BasinMap {
        pub fn from_height_map(height_map: &[Vec<u8>]) -> Self {
            let num_of_rows = height_map.len() as u8;
            let mut basin_map = BasinMap {
                basins: HashMap::new(),
                map: HashMap::new()
            };
            let mut id_equality = IDEqualitySet::new();

            // connect basins horizontally
            let mut current_id = 0; // increment when the next point cannot belong to the same basin
            height_map.iter().enumerate().for_each(|(i, row)| {
                row.iter().enumerate().for_each(|(j, &height)| {
                    if height == 9 { current_id += 1; }
                    else { basin_map.add_basin_point(current_id, (i as u8, j as u8)); }
                });
                current_id += 1;
            });

            // connect basins vertically
            basin_map.basins.iter().for_each(|(&id, basin)| {
                // if the point below is a basin, merge by grouping their IDs using IDEqualitySet
                basin.iter().for_each(|&point| {
                    if point.0 == num_of_rows - 1 { return; }
                    match basin_map.map.get(&(point.0+1, point.1)) {
                        None => (),
                        Some(other_id) => id_equality.add_equality(id, *other_id)
                    }
                })
            });
            basin_map.update_with_equalities(&id_equality);

            basin_map
        }

        fn add_basin_point(&mut self, id: BasinID, point: Point) {
            self.basins.entry(id).or_insert_with(Basin::new).insert(point);
            self.map.insert(point, id);
        }

        fn update_with_equalities(&mut self, equalities: &IDEqualitySet) {
            equalities.0.iter().for_each(|set| {
                let mut id_list = set.iter();

                // remove and merge all hashsets in self.basins belonging to the same basin
                let base_id = id_list.next().unwrap();
                let mut base_basin = self.basins.remove(base_id).unwrap();
                for id in id_list {
                    base_basin.extend(self.basins.remove(id).unwrap());
                }

                // update all points in self.map to use the same id
                base_basin.iter().for_each(|point| {
                    *self.map.get_mut(point).unwrap() = *base_id;
                });

                // re-insert the basin into self.basins
                self.basins.insert(*base_id, base_basin);
            });
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_data =
        "2199943210
        3987894921
        9856789892
        8767896789
        9899965678";

        assert_eq!(day9_main(test_data), (15, 1134));
    }
}