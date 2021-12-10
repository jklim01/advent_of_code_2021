use std::process;

const FLOOR_SIZE: usize = 1000;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Point(i16, i16);
impl Point {
    fn from_str(s: &str) -> Result<Self, ()> {
        let coordinates = s.split(",")
            .map(|slice| slice.trim().parse::<i16>().map_err(|_| ()))
            .collect::<Result<Vec<_>, ()>>()?;
        if coordinates.len() != 2 { return Err(()); }
        Ok(Point(coordinates[0], coordinates[1]))
    }
    fn flip(&mut self) {
        std::mem::swap(&mut self.0, &mut self.1);
    }
}
impl std::ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl std::ops::Sub<Point> for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

fn rasterize_line_seg(mut start: Point, mut end: Point) -> Vec<Point> {
    let mut delta = end - start;
    let mut flipped = false;

    // if abs(slope) is larger than 1, flip axis to make it <= 1
    if delta.1.abs() > delta.0.abs() {
        flipped = true;
        delta.flip();
        start.flip();
        end.flip();
    }

    // ensure first coordinate ascending from start to end (iterator must be ascending)
    if start.0 > end.0 {
        delta.0 *= -1;
        delta.1 *= -1;
        std::mem::swap(&mut start, &mut end);
    }
    let points = start.0..=end.0;

    // Bresenham's Line Algorithm
    let mut points = points
        .scan((-delta.0, start.1), |(diff, y), x| {
            if *diff >= 0 {
                *y += delta.1.signum();
                *diff -= 2*delta.0;
            }
            *diff += 2*delta.1.abs();   // calculate diff for next point
            Some(Point(x, *y))
        })
        .collect::<Vec<Point>>();

    if flipped { points.iter_mut().for_each(Point::flip); } // if axis flipped, revert to normal
    points
}

#[derive(Debug)]
struct FloorMap(Box<[[u8; FLOOR_SIZE]; FLOOR_SIZE]>);
impl FloorMap {
    fn new() -> Self {
        FloorMap(Box::new([[0; FLOOR_SIZE]; FLOOR_SIZE]))
    }

    fn add_line_part1(&mut self, start: Point, end: Point) -> bool {
        // horizontal line
        if start.1 == end.1 {
            let x_iter = match end.0 > start.0 {
                true => start.0..=end.0,
                false => end.0..=start.0
            };
            for x in x_iter {
                self.0[start.1 as usize][x as usize] += 1;
            }
            return true;
        }
        // vertical line
        if start.0 == end.0 {
            let y_iter = match end.1 > start.1 {
                true => start.1..=end.1,
                false => end.1..=start.1
            };
            for y in y_iter {
                self.0[y as usize][start.0 as usize] += 1;
            }
            return true;
        }
        false
    }

    fn add_line_part2(&mut self, start: Point, end: Point) -> bool {
        // let line_coordinates = match LineCoordIterator::new(start, end) {
        //     Err(_) => return false,
        //     Ok(x) => x
        // };
        let line_coordinates = rasterize_line_seg(start, end);
        for Point(x, y) in line_coordinates {
            self.0[y as usize][x as usize] += 1;
        }
        true
    }

    fn count_intersections(&self) -> u16 {
        self.0.iter().flatten().fold(0, |acc, x| {
            if *x > 1 { return acc + 1; }
            acc
        })
    }
}

pub fn day5_main(file_data: &str) -> (u16, u16) {
    // Part 1
    let mut floor_map = FloorMap::new();
    for (i, line) in file_data.lines().enumerate() {
        let coordinates = line.split(" -> ")
            .map(Point::from_str)
            .collect::<Result<Vec<Point>, ()>>()
            .unwrap_or_else(|_| {
                eprintln!("Error parsing coordinates on line {}!", i+1);
                process::exit(1);
            });
        if coordinates.len() != 2 {
            eprintln!("Invalid number of tokens in line {}!", i+1);
            process::exit(1);
        }
        if !floor_map.add_line_part1(coordinates[0], coordinates[1]) {
            println!("[Part 1] The vent coordinates on line {} was ignored!", i+1);
        }
    }
    let intersection_count_1 = floor_map.count_intersections();
    println!("There are {} points where the lines intersect!", intersection_count_1);


    // Part 2
    let mut floor_map = FloorMap::new();
    for (i, line) in file_data.lines().enumerate() {
        let coordinates = line.split(" -> ")
            .map(Point::from_str)
            .collect::<Result<Vec<Point>, ()>>()
            .unwrap_or_else(|_| {
                eprintln!("Error parsing coordinates on line {}!", i+1);
                process::exit(1);
            });
        if coordinates.len() != 2 {
            eprintln!("Invalid number of tokens in line {}!", i+1);
            process::exit(1);
        }
        if !floor_map.add_line_part2(coordinates[0], coordinates[1]) {
            eprintln!("Unable to map the vent coordinates on line {}!", i+1);
            eprintln!("The coordinates must form a horizontal, vertical or 45 degree line.");
            process::exit(1);
        }
    }
    let intersection_count_2 = floor_map.count_intersections();
    println!("There are {} points where the lines intersect!", intersection_count_2);

    (intersection_count_1, intersection_count_2)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_data =
            "0,9 -> 5,9
            8,0 -> 0,8
            9,4 -> 3,4
            2,2 -> 2,1
            7,0 -> 7,4
            6,4 -> 2,0
            0,9 -> 2,9
            3,4 -> 1,4
            0,0 -> 8,8
            5,5 -> 8,2";
        assert_eq!(day5_main(test_data), (5, 12)); // part 2
    }
}