use std::str;
use std::fmt;
use core::ops;
use std::error::Error;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

const SIZE: usize = 100;
const MIN_RISK: u16 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point(usize, usize);
impl Point {
    fn get_neighbours(&self, grid_size: Point) -> [Option<Point>; 4] {
        // ignores over- or under- flow, passing the bounds checking responsibility to the caller

        let up = (self.0 > 0).then(|| Point(self.0-1, self.1));
        let down = (self.0 < grid_size.0-1).then(|| Point(self.0+1, self.1));
        let left = (self.1 > 0).then(|| Point(self.0, self.1-1));
        let right = (self.1 < grid_size.1-1).then(|| Point(self.0, self.1+1));
        [up, down, left, right]
    }
}
impl<T> ops::Index<Point> for Grid<T> {
    type Output = T;
    fn index(&self, index: Point) -> &Self::Output {
        &self.grid[index.0 * self.size.1 + index.1]
    }
}
impl<T> ops::IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.grid[index.0 * self.size.1 + index.1]
    }
}

#[derive(PartialEq, Eq)]
struct PQueueElem {
    hcost: u16, cost: u16, pos: Point
}
// flip comparison order so that BinaryHeap implemented as min-heap
impl PartialOrd for PQueueElem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match other.hcost.cmp(&self.hcost) {
            Ordering::Equal => Some(other.cost.cmp(&self.cost)),
            x => Some(x)
        }
    }
}
impl Ord for PQueueElem {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.hcost.cmp(&self.hcost) {
            Ordering::Equal => other.cost.cmp(&self.cost),
            x => x
        }
    }
}

#[derive(Debug)]
enum ParseCavernError {
    InvalidSize, InvalidRisk(Point)
}
impl fmt::Display for ParseCavernError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseCavernError::InvalidSize => write!(f, "cavern size and string length don't match"),
            ParseCavernError::InvalidRisk(p) =>
                write!(f, "invalid risk level at position ({}, {})", p.0+1, p.1+1),
        }
    }
}
impl Error for ParseCavernError {}

struct Grid<T> {
    grid: Vec<T>, size: Point
}
impl<T: Clone> Grid<T> {
    fn new(size: Point, init_val: T) -> Self {
        Grid { grid: vec![init_val; size.0*size.1], size }
    }
}
type CavernMap = Grid<u8>;
impl CavernMap {
    fn from_str(s: &str, size: Point) -> Result<Self, ParseCavernError> {
        let count = size.0 * size.1;
        let mut cavern = CavernMap::new(size, 0);
        let mut i = 0;
        let risk_chars = s.lines().flat_map(|line| line.trim().chars());
        for c in risk_chars {
            let risk = c.to_digit(10)
                .ok_or_else(|| ParseCavernError::InvalidRisk(Point((i/size.1)*size.1, i%size.1)))? as u8;
            if i > count-1 { return Err(ParseCavernError::InvalidSize); }
            cavern.grid[i] = risk;
            i += 1;
        }
        if i != count { return Err(ParseCavernError::InvalidSize); }
        Ok(cavern)
    }

    fn expand_map(&self) -> Self {
        let mut new_cavern = CavernMap::new(Point(5*self.size.0, 5*self.size.1), 0);
        for i in 0..new_cavern.size.0 {
            for j in 0..new_cavern.size.1 {
                let base_index = Point(i % self.size.0, j % self.size.1);
                let risk_offset = ((i-base_index.0) + (j-base_index.1)) / SIZE;
                let mut risk = self[base_index] + risk_offset as u8;
                while risk > 9 {
                    risk -= 9;
                }
                new_cavern[Point(i, j)] = risk;
            }
        }
        new_cavern
    }

    // find minimum cost path using A* search algorithm
    fn find_min_risk(&self, start: Point, end: Point) -> Result<Option<u16>, ()> {
        let is_in_bounds = |p: Point| -> bool {
            p.0 < self.size.0 && p.1 < self.size.1
        };
        if !is_in_bounds(start) || !is_in_bounds(end) { return Err(()); }

        let estimate_cost = |p: Point| -> u16 {
            // use manhattan distance (modified to prevent overestimation) as heuristic cost estimate
            MIN_RISK * (my_abs_diff(end.0, p.0) + my_abs_diff(end.1, p.1)) as u16
        };
        let mut min_costs = Grid::new(self.size, None);
        min_costs[start] = Some(0);

        let mut pqueue = BinaryHeap::new();
        pqueue.push(PQueueElem {
            hcost: estimate_cost(start), cost: 0, pos: start
        });
        let mut run_count = 0;
        while let Some(elem) = pqueue.pop() {
            run_count += 1;
            if elem.pos == end { println!("run_count = {}", run_count); return Ok(min_costs[elem.pos]); }

            // don't bother anymore if an even cheaper path to elem.pos has already been found
            if min_costs[elem.pos].is_none() || min_costs[elem.pos].unwrap() < elem.cost { continue; }

            for p in elem.pos.get_neighbours(self.size) {
                if p.is_none() { continue; }
                let p = p.unwrap();
                let new_cost = elem.cost + self[p] as u16;
                if min_costs[p].is_none() || new_cost < min_costs[p].unwrap() {
                    min_costs[p] = Some(new_cost);
                    pqueue.push(PQueueElem {
                        hcost: new_cost + estimate_cost(p), cost: new_cost, pos: p
                    });
                }
            }
        }
        Ok(None)
    }

    fn bidirectional_astar(&self, start: Point, end: Point) -> Result<Option<u16>, ()> {
        let is_in_bounds = |p: Point| -> bool {
            p.0 < self.size.0 && p.1 < self.size.1
        };
        if !is_in_bounds(start) || !is_in_bounds(end) { return Err(()); }

        let estimate_cost = |p1: Point, p2:Point| -> u16 {
            // use manhattan distance (modified to prevent overestimation) as heuristic cost estimate
            MIN_RISK * (my_abs_diff(p1.0, p2.0) + my_abs_diff(p1.1, p2.1)) as u16
        };

        let mut min_costs = Grid::new(self.size, (None, None));
        min_costs[start] = (Some(0), None);
        min_costs[end] = (None, Some(self[end] as u16));

        let mut mu = None;
        let mut forward_q = BinaryHeap::new();
        let mut backward_q = BinaryHeap::new();
        forward_q.push(PQueueElem {
            hcost: estimate_cost(start, end), cost: 0, pos: start
        });
        backward_q.push(PQueueElem {
            hcost: (self[end] as u16) + estimate_cost(end, start), cost: (self[end] as u16), pos: end
        });

        while let (Some(u), Some(v)) = (forward_q.pop(), backward_q.pop()) {
            if let Some(d) = mu {
                if u.cost + v.cost > d { return Ok(mu); }
            }

            if !(min_costs[u.pos].0.unwrap() < u.cost) {
                for p in u.pos.get_neighbours(self.size) {
                    if p.is_none() { continue; }
                    let p = p.unwrap();
                    let new_cost = u.cost + self[p] as u16;
                    if min_costs[p].0.is_none() || new_cost < min_costs[p].0.unwrap() {
                        min_costs[p].0 = Some(new_cost);
                        forward_q.push(PQueueElem {
                            hcost: new_cost + estimate_cost(p, end), cost: new_cost, pos: p
                        });

                        // update mu as required
                        if let (Some(x), Some(y)) = min_costs[p] {
                            let new_mu = x + y - (self[p] as u16);
                            if mu.is_none() || new_mu < mu.unwrap() {
                                mu = Some(new_mu)
                            }
                        }
                    }
                }
            }

            if !(min_costs[v.pos].1.unwrap() < v.cost) {
                for p in v.pos.get_neighbours(self.size) {
                    if p.is_none() { continue; }
                    let p = p.unwrap();
                    let new_cost = v.cost + self[p] as u16;
                    if min_costs[p].1.is_none() || new_cost < min_costs[p].1.unwrap() {
                        min_costs[p].1 = Some(new_cost);
                        backward_q.push(PQueueElem {
                            hcost: new_cost + estimate_cost(start, p), cost: new_cost, pos: p
                        });

                        // update mu as required
                        if let (Some(x), Some(y)) = min_costs[p] {
                            let new_mu = x + y - (self[p] as u16);
                            if mu.is_none() || new_mu < mu.unwrap() {
                                mu = Some(new_mu)
                            }
                        }
                    }
                }
            }
        }
        Ok(mu)
    }

    fn bidirectional_dijkstra(&self, start: Point, end: Point) -> Result<Option<u16>, ()> {
        let is_in_bounds = |p: Point| -> bool {
            p.0 < self.size.0 && p.1 < self.size.1
        };
        if !is_in_bounds(start) || !is_in_bounds(end) { return Err(()); }

        let estimate_cost = |p1: Point, p2:Point| -> u16 {
            // use manhattan distance (modified to prevent overestimation) as heuristic cost estimate
            MIN_RISK * (my_abs_diff(p1.0, p2.0) + my_abs_diff(p1.1, p2.1)) as u16
            // 0 as u16
        };

        let mut min_costs = Grid::new(self.size, (None, None, false, false));
        min_costs[start] = (Some(0), None, true, false);
        min_costs[end] = (None, Some(self[end] as u16), false, true);

        let mut mu = None;
        let mut forward_q = BinaryHeap::new();
        let mut backward_q = BinaryHeap::new();
        forward_q.push(PQueueElem {
            hcost: estimate_cost(start, end), cost: 0, pos: start
        });
        backward_q.push(PQueueElem {
            hcost: (self[end] as u16) + estimate_cost(end, start), cost: (self[end] as u16), pos: end
        });
        let mut flag = false;
        let mut run_count = 0;

        while let (Some(u), Some(v)) = (forward_q.pop(), backward_q.pop()) {
            run_count += 2;
            if let Some(d) = mu {
                if u.cost + v.cost > d { println!("run_count = {}", run_count); return Ok(mu); }
            }
            min_costs[u.pos].2 = true;
            min_costs[v.pos].3 = true;
            if flag == true { println!("Continues processing even after a double processed node was found!"); }
            if min_costs[u.pos].3 == true { println!("Found a double processed node!"); flag = true; }
            if min_costs[v.pos].2 == true { println!("Found a double processed node!"); flag = true; }

            if !(min_costs[u.pos].0.unwrap() < u.cost) {
                for p in u.pos.get_neighbours(self.size) {
                    if p.is_none() { continue; }
                    let p = p.unwrap();
                    let new_cost = u.cost + self[p] as u16;
                    if min_costs[p].0.is_none() || new_cost < min_costs[p].0.unwrap() {
                        min_costs[p].0 = Some(new_cost);
                        forward_q.push(PQueueElem {
                            hcost: new_cost + estimate_cost(p, end), cost: new_cost, pos: p
                        });

                        // update mu as required
                        if let (Some(x), Some(y), _, _) = min_costs[p] {
                            let new_mu = x + y - (self[p] as u16);
                            if mu.is_none() || new_mu < mu.unwrap() {
                                mu = Some(new_mu)
                            }
                        }
                    }
                }
            }

            if !(min_costs[v.pos].1.unwrap() < v.cost) {
                for p in v.pos.get_neighbours(self.size) {
                    if p.is_none() { continue; }
                    let p = p.unwrap();
                    let new_cost = v.cost + self[p] as u16;
                    if min_costs[p].1.is_none() || new_cost < min_costs[p].1.unwrap() {
                        min_costs[p].1 = Some(new_cost);
                        backward_q.push(PQueueElem {
                            hcost: new_cost + estimate_cost(start, p), cost: new_cost, pos: p
                        });

                        // update mu as required
                        if let (Some(x), Some(y), _, _) = min_costs[p] {
                            let new_mu = x + y - (self[p] as u16);
                            if mu.is_none() || new_mu < mu.unwrap() {
                                mu = Some(new_mu)
                            }
                        }
                    }
                }
            }
        }
        Ok(mu)
    }
}

fn my_abs_diff<T: std::ops::Sub<Output = T> + Ord>(a: T, b: T) -> T {
    if a > b { a-b }
    else { b-a }
}


pub fn day15_main(file_data: &str) -> (Option<u16>, Option<u16>) {
    // Part 1
    let cavern = CavernMap::from_str(file_data, Point(SIZE, SIZE))
        .unwrap_or_else(|e| {
            panic!("Error parsing cavern! : {}", e);
        });
    let (start, end) = (Point(0, 0), Point(SIZE-1, SIZE-1));
    let part1_answer = cavern.find_min_risk(start, end)
        .unwrap_or_else(|_| panic!("[Part 1] Error: One or more endpoints not within cavern!"));
    match part1_answer {
        Some(x) => println!("[Part 1] The lowest total risk possible is {}.", x),
        None => println!("[Part 1] Could not find any path connecting the endpoints."),
    }

    // Part 2
    let new_cavern = cavern.expand_map();
    let (start, end) = (Point(0, 0), Point(5*SIZE-1, 5*SIZE-1));
    let part2_answer = new_cavern.bidirectional_astar(start, end)
        .unwrap_or_else(|_| panic!("[Part 2] Error: One or more endpoints not within cavern!"));
    match part2_answer {
        Some(x) => println!("[Part 2] The lowest total risk possible is {}.", x),
        None => println!("[Part 2] Could not find any path connecting the endpoints."),
    }

    // Test
    let test1 = cavern;
    println!("--- Starting test.....................................................");
    let test1_ans1 = test1.find_min_risk(Point(71, 29), Point(79, 51)).unwrap().unwrap();
    println!("--- Completed test 1.1 with {}", test1_ans1);

    println!("--- Starting test..........");
    let test1_ans2 = test1.find_min_risk(Point(53, 29), Point(20, 75)).unwrap().unwrap();
    println!("--- Completed test 1.2 with {}", test1_ans2);


    let test2 = test1.expand_map();
    println!("--- Starting test..........");
    let test2_ans1 = test2.find_min_risk(Point(233, 366), Point(250, 287)).unwrap().unwrap();
    println!("--- Completed test 2.1 with {}", test2_ans1);

    println!("--- Starting test..........");
    let test2_ans2 = test2.find_min_risk(Point(332, 419), Point(109, 451)).unwrap().unwrap();
    println!("--- Completed test 2.2 with {}", test2_ans2);


    let test3 = test2.expand_map();
    println!("--- Starting test..........");
    let test3_ans1 = test3.find_min_risk(Point(1738, 1251), Point(1993, 1800)).unwrap().unwrap();
    println!("--- Completed test 3.1 with {}", test3_ans1);

    println!("--- Starting test..........");
    let test3_ans2 = test3.find_min_risk(Point(1093, 2152), Point(2460, 1648)).unwrap().unwrap();
    println!("--- Completed test 3.2 with {}", test3_ans2);

    (part1_answer, part2_answer)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_data =
            "1163751742
            1381373672
            2136511328
            3694931569
            7463417111
            1319128137
            1359912421
            3125421639
            1293138521
            2311944581";
        assert_eq!(day15_main(test_data), (Some(40), Some(315)));
    }
}