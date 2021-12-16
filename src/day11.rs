use std::iter;
use std::str;
use std::num::Wrapping;
use core::ops::{Index, IndexMut};
use std::collections::HashSet;

const GRID_SIZE: usize = 10;
type Point = (usize, usize);

// allows overflow and points outside the grid
fn get_neighbours(p: Point) -> [Point; 8] {
    let mut neighbours = [(0, 0); 8];
    let u = (Wrapping(p.0)-Wrapping(1)).0;
    let d = (Wrapping(p.0)+Wrapping(1)).0;
    let l = (Wrapping(p.1)-Wrapping(1)).0;
    let r = (Wrapping(p.1)+Wrapping(1)).0;
    neighbours[0] = (u, l);
    neighbours[1] = (u, p.1);
    neighbours[2] = (u, r);
    neighbours[3] = (p.0, r);
    neighbours[4] = (d, r);
    neighbours[5] = (d, p.1);
    neighbours[6] = (d, l);
    neighbours[7] = (p.0, l);
    neighbours
}

#[derive(Clone, PartialEq, Debug)]
struct OctopusGrid(Box<[[u8; GRID_SIZE]; GRID_SIZE]>);
impl OctopusGrid {
    fn trigger_point(&mut self, point: Point) -> bool {
        if point.0 > GRID_SIZE-1 || point.1 > GRID_SIZE-1 { return false; }
        self[point] += 1;
        self[point] > 9
    }
}
impl str::FromStr for OctopusGrid {
    type Err = Point;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines().enumerate().flat_map(|(i, line)| {
            line.trim().chars().enumerate().map(move |(j, c)| {
                c.to_digit(10).map(|x| x as u8).ok_or((i, j))
            })
        })
        .collect()
    }
}
impl iter::FromIterator<u8> for OctopusGrid {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        let mut octopus_grid = OctopusGrid(Box::new([[0; GRID_SIZE]; GRID_SIZE]));
        let mut p: Point = (0, 0);
        for energy in iter {
            octopus_grid[p] = energy;
            if p == (GRID_SIZE-1, GRID_SIZE-1) { break; }
            else if p.1 == GRID_SIZE-1 { p.0 += 1; p.1 = 0; }
            else { p.1 += 1; }
        }
        octopus_grid
    }
}
impl iter::IntoIterator for OctopusGrid {
    type Item = u8;
    type IntoIter = OctopusGridIntoIter;
    fn into_iter(self) -> Self::IntoIter {
        OctopusGridIntoIter {
            grid: self,
            stack: Vec::new(),
            flashers: HashSet::new(),
        }
    }
}
impl Index<Point> for OctopusGrid {
    type Output = u8;
    fn index(&self, index: Point) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}
impl IndexMut<Point> for OctopusGrid {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.0[index.0][index.1]
    }
}

#[derive(Clone)]
struct OctopusGridIntoIter {
    grid: OctopusGrid,
    stack: Vec<Point>,
    flashers: HashSet<Point>
}
impl Iterator for OctopusGridIntoIter {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        // increment energy levels
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                if self.grid.trigger_point((i, j)) {
                    self.stack.push((i, j));
                    self.flashers.insert((i, j));
                }
            }
        }

        // trigger neighbours of flashers
        while let Some(point) = self.stack.pop() {
            for &p in get_neighbours(point).iter() {
                if !self.flashers.contains(&p) && self.grid.trigger_point(p) {
                    self.stack.push(p);
                    self.flashers.insert(p);
                }
            }
        }

        // set energy level of flashers to 0
        for &p in self.flashers.iter() {
            self.grid[p] = 0;
        }

        let ret_val = self.flashers.len() as u8;
        self.flashers.clear();
        Some(ret_val)
    }
}




pub fn day11_main(file_data: &str) -> (u16, usize) {
    let mut octopus_grid_iter = file_data.parse::<OctopusGrid>()
        .unwrap_or_else(|(i, j)| {
            panic!("Error parsing energy level of octopus in position ({}, {}).", i, j);
        })
        .into_iter();

    let flash_count: u16 = octopus_grid_iter.clone().take(100).map(|x| x as u16).sum();
    println!("[Part 1] After 100 steps, {} flashes occured!", flash_count);
    let step_count = octopus_grid_iter.position(|x| x == (GRID_SIZE*GRID_SIZE) as u8).unwrap() + 1;
    println!("[Part 2] First synchronization happens after {} steps!", step_count);

    (flash_count, step_count)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let str_to_grid = |s: &str| {
            s.chars().filter_map(|c| c.to_digit(10).map(|x| x as u8)).collect::<OctopusGrid>()
        };

        let test_data =
            "5483143223
            2745854711
            5264556173
            6141336146
            6357385478
            4167524645
            2176841721
            6882881134
            4846848554
            5283751526";

        let day100_grid =
            "0397666866
            0749766918
            0053976933
            0004297822
            0004229892
            0053222877
            0532222966
            9322228966
            7922286866
            6789998766";

        assert_eq!(day11_main(test_data), (1656, 195));

        let mut calculated_day100_grid = str_to_grid(test_data).into_iter();
        for _ in 1..=100 { calculated_day100_grid.next(); }
        let calculated_day100_grid = calculated_day100_grid.grid;
        let day100_grid = str_to_grid(day100_grid);
        assert_eq!(calculated_day100_grid, day100_grid);
    }

}