use std::mem;
use std::cmp;
use std::iter;
use std::ops::{Sub, Add};
use std::error::Error;
use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::HashSet;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
enum ParsePositionError {
    InvalidDimension, ParseCoordinateError(ParseIntError)
}
impl From<ParseIntError> for ParsePositionError {
    fn from(e: ParseIntError) -> Self {
        ParsePositionError::ParseCoordinateError(e)
    }
}
impl Display for ParsePositionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            ParsePositionError::InvalidDimension => write!(f, "invalid dimension, expected dim = 3"),
            ParsePositionError::ParseCoordinateError(_) => write!(f, "error parsing coordinate"),
        }
    }
}
impl Error for ParsePositionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            ParsePositionError::ParseCoordinateError(e) => Some(e),
            _ => None
        }
    }
}


#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Position(i32, i32, i32);
impl FromStr for Position {
    type Err = ParsePositionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coordinates = s.split(',').map(str::trim);
        let x = coordinates.next()
            .ok_or(ParsePositionError::InvalidDimension)?
            .parse::<i32>()?;
        let y = coordinates.next()
            .ok_or(ParsePositionError::InvalidDimension)?
            .parse::<i32>()?;
        let z = coordinates.next()
            .ok_or(ParsePositionError::InvalidDimension)?
            .parse::<i32>()?;
        if coordinates.next().is_some() { return Err(ParsePositionError::InvalidDimension); }

        Ok(Position(x, y, z))
    }
}
impl Sub<Self> for Position {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Position(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}
impl Add<Self> for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl Position {
    // positive rotation of the reference frame about the x axis
    fn roll(&mut self) {
        mem::swap(&mut self.1, &mut self.2);
        self.2 *= -1;
    }

    // positive rotation of the reference frame about the z axis
    fn turn(&mut self) {
        mem::swap(&mut self.0, &mut self.1);
        self.1 *= -1;
    }

    // negative rotation of the reference frame about the z axis
    fn turn_rev(&mut self) {
        mem::swap(&mut self.0, &mut self.1);
        self.0 *= -1;
    }

    // translation of the reference frame
    fn translate(&mut self, offset: Position) {
        self.0 -= offset.0;
        self.1 -= offset.1;
        self.2 -= offset.2;
    }

    // return iterator of operations that cycle through all possible orientations
    fn get_orientation_walker() -> iter::Take<iter::Cycle<std::slice::Iter<'static, for<'r> fn(&'r mut Position)>>> {
        static OPS: [fn(&mut Position); 8] =
            [Position::roll, Position::turn, Position::turn, Position::turn,
                Position::roll, Position::turn_rev, Position::turn_rev, Position::turn_rev];

        OPS.iter().cycle().take(24)
    }

    fn distance_squared(&self, other: &Self) -> i32 {
        (self.0 - other.0).pow(2) + (self.1 - other.1).pow(2) + (self.2 - other.2).pow(2)
    }

    fn manhattan_distance(&self, other: &Self) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs() + (self.2 - other.2).abs()
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
struct Scanner {
    scanned_beacons: Vec<Position>,
    fingerprint: HashSet<i32>,
    dedup_count: usize
}
impl FromStr for Scanner {
    type Err = (ParsePositionError, usize);

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let scanned_beacons = s.lines().skip(1)
            .enumerate()
            .map(|(i, s)| {
                s.parse::<Position>().map_err(|e| (e, i+1))
            })
            .collect::<Result<Vec<Position>, Self::Err>>()?;

        let fingerprint = scanned_beacons.iter().enumerate()
            .map(|(i, pos)| {
                scanned_beacons.iter().skip(i+1).map(|other| {
                    pos.distance_squared(other)
                })
            })
            .flatten()
            .collect::<HashSet<i32>>();

        let dedup_count = (scanned_beacons.len()*(scanned_beacons.len()-1))/2 - fingerprint.len();

        Ok(Scanner { scanned_beacons, fingerprint, dedup_count })

    }
}
impl Scanner {
    // rotate the reference frame
    fn rotate(&mut self, rotation: for<'r> fn(&'r mut Position)) {
        for pos in self.scanned_beacons.iter_mut() {
            rotation(pos);
        }
    }

    // translates the reference frame
    fn translate(&mut self, offset: Position) {
        for pos in self.scanned_beacons.iter_mut() {
            pos.translate(offset);
        }
    }

    // Rotates and translates `self` to try to align the scanners and check if they overlap.
    // If they indeed overlap, the orientation and position of `selfr` upon alignment is kept upon return.
    // Otherwise, the original orientation and position of `self` is preserved.
    fn try_align_with(&mut self, other: &Self) -> Option<Position> {
        // compare fingerprints first
        let tolerance = cmp::min(self.dedup_count, other.dedup_count);
        if self.fingerprint.intersection(&other.fingerprint).count() < 66 - tolerance {
            return None;
        }

        let other_beacon_set = other.scanned_beacons.iter().cloned().collect::<HashSet<Position>>();
        for rotation in Position::get_orientation_walker() {
            self.rotate(*rotation);
            for point in other.scanned_beacons.iter() {
                for i in 0..self.scanned_beacons.len() {
                    // try matching each beacon `self` sees with each beacon `other` sees
                    // (matches if common_count >= 12)
                    let offset = self.scanned_beacons[i] - *point;
                    self.translate(offset);

                    let mut common_count = 0;
                    for beacon in self.scanned_beacons.iter() {
                        if other_beacon_set.contains(beacon) {
                            common_count += 1;
                            if common_count >= 12 {
                                return Some(offset);
                            }
                        }
                    }
                    // let common_count = self.scanned_beacons.iter().fold(0, |acc, x| {
                    //     if other_beacon_set.contains(x) {
                    //         acc + 1
                    //     }
                    //     else { acc }
                    // });
                    // if common_count >= 12 {
                    //     return Some(offset);
                    // }

                    // return back to original position
                    self.translate(Position(0, 0, 0) - offset);
                }
            }
        }
        None
    }
}



pub fn day19_main(file_data: &str) -> (usize, i32) {
    let block_delimiter = file_data.chars()
        .nth(file_data.lines().next().unwrap().len());
    let block_delimiter = match block_delimiter {
        Some('\r') => "\r\n\r\n",
        Some('\n') => "\n\n",
        _ => unreachable!()
    };

    let mut scanners = file_data.split(block_delimiter)
        .enumerate()
        .map(|(i, s)| {
            s.parse::<Scanner>().map_err(|e| (e, i))
        })
        .collect::<Result<Vec<Scanner>, _>>()
        .unwrap_or_else(|((e, beacon_number), scanner_number)| {
            panic!("unable to parse scanner {}, caused by ParsePositionError({}) when parsing beacon {}",
                scanner_number, e, beacon_number);
        });

    // The first scanner in `verified_scanners` is the reference scanner. All positions in `all_beacons` are relative
    // to this scanner, and all verified scanners are rotated to match its orientation as well.
    // The position stored with each verified scanner is the offset required to shift it to the reference scanner's position.
    let mut verified_scanners: Vec<(Position, Scanner)> = vec![(Position(0, 0, 0), scanners.pop().unwrap())];
    let mut all_beacons = verified_scanners[0].1.scanned_beacons
        .iter().cloned().collect::<HashSet<Position>>();

    let mut i = 0;
    let mut scanners_to_move = Vec::new();
    while !scanners.is_empty() {
        for (scanner_index, scanner) in scanners.iter_mut().enumerate() {
            let result = scanner.try_align_with(&verified_scanners[i].1);
            if let Some(offset) = result {
                // `scanner` now has the same orientation and position as verified scanner `i`, thus we only
                // need to translate it to the reference scanner's position
                scanner.translate(verified_scanners[i].0);
                all_beacons.extend(scanner.scanned_beacons.iter());

                // translate `scanner` back to its original position, but keep its current orientation
                let reference_offset = verified_scanners[i].0 + offset;
                scanner.translate(Position(0, 0, 0) - reference_offset);

                scanners_to_move.push((reference_offset, scanner_index));
            }
        }

        while let Some((offset, index)) = scanners_to_move.pop() {
            // swap_remove can be used because the higher indices will be removed first
            let scanner = scanners.swap_remove(index);
            verified_scanners.push((offset, scanner));
        }
        i += 1;
    }

    let part1_ans = all_beacons.len();
    println!("[Part 1] There are {} beacons in total.", part1_ans);

    let part2_ans = verified_scanners.iter().enumerate()
        .map(|(i, (pos, _))| {
            verified_scanners.iter().skip(i+1).map(|(other, _)| {
                pos.manhattan_distance(other)
            })
        })
        .flatten()
        .max()
        .unwrap();
    println!("[Part 2] The largest manhattan distance between any two scanners is {}.", part2_ans);

    (part1_ans, part2_ans)
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_position_transformation() {
        let mut i_hat = Position(1, 0, 0);
        i_hat.turn();
        assert_eq!(i_hat, Position(0, -1, 0));

        let mut j_hat = Position(0, 1, 0);
        j_hat.roll();
        assert_eq!(j_hat, Position(0, 0, -1));

        let mut k_hat = Position(0, 0, 1);
        k_hat.turn_rev();
        assert_eq!(k_hat, Position(0, 0, 1));

        let mut pos = Position(1, 0, 0);
        pos.translate(Position(0, 1, 1));
        assert_eq!(pos, Position(1, -1, -1))
    }

    #[test]
    fn test_try_align_with() {
        let mut scanner_0 =
            "--- scanner 0 ---
            404,-588,-901
            528,-643,409
            -838,591,734
            390,-675,-793
            -537,-823,-458
            -485,-357,347
            -345,-311,381
            -661,-816,-575
            -876,649,763
            -618,-824,-621
            553,345,-567
            474,580,667
            -447,-329,318
            -584,868,-557
            544,-627,-890
            564,392,-477
            455,729,728
            -892,524,684
            -689,845,-530
            423,-701,434
            7,-33,-71
            630,319,-379
            443,580,662
            -789,900,-551
            459,-707,401".parse::<Scanner>().unwrap();

        let scanner_1 =
            "--- scanner 1 ---
            686,422,578
            605,423,415
            515,917,-361
            -336,658,858
            95,138,22
            -476,619,847
            -340,-569,-846
            567,-361,727
            -460,603,-452
            669,-402,600
            729,430,532
            -500,-761,534
            -322,571,750
            -466,-666,-811
            -429,-592,574
            -355,545,-477
            703,-491,-529
            -328,-685,520
            413,935,-424
            -391,539,-444
            586,-435,557
            -364,-763,-893
            807,-499,-711
            755,-354,-619
            553,889,-390".parse::<Scanner>().unwrap();

        assert!(scanner_0.try_align_with(&scanner_1).is_some());
    }

    #[test]
    fn it_works() {
        let test_data =
            "--- scanner 0 ---
            404,-588,-901
            528,-643,409
            -838,591,734
            390,-675,-793
            -537,-823,-458
            -485,-357,347
            -345,-311,381
            -661,-816,-575
            -876,649,763
            -618,-824,-621
            553,345,-567
            474,580,667
            -447,-329,318
            -584,868,-557
            544,-627,-890
            564,392,-477
            455,729,728
            -892,524,684
            -689,845,-530
            423,-701,434
            7,-33,-71
            630,319,-379
            443,580,662
            -789,900,-551
            459,-707,401

            --- scanner 1 ---
            686,422,578
            605,423,415
            515,917,-361
            -336,658,858
            95,138,22
            -476,619,847
            -340,-569,-846
            567,-361,727
            -460,603,-452
            669,-402,600
            729,430,532
            -500,-761,534
            -322,571,750
            -466,-666,-811
            -429,-592,574
            -355,545,-477
            703,-491,-529
            -328,-685,520
            413,935,-424
            -391,539,-444
            586,-435,557
            -364,-763,-893
            807,-499,-711
            755,-354,-619
            553,889,-390

            --- scanner 2 ---
            649,640,665
            682,-795,504
            -784,533,-524
            -644,584,-595
            -588,-843,648
            -30,6,44
            -674,560,763
            500,723,-460
            609,671,-379
            -555,-800,653
            -675,-892,-343
            697,-426,-610
            578,704,681
            493,664,-388
            -671,-858,530
            -667,343,800
            571,-461,-707
            -138,-166,112
            -889,563,-600
            646,-828,498
            640,759,510
            -630,509,768
            -681,-892,-333
            673,-379,-804
            -742,-814,-386
            577,-820,562

            --- scanner 3 ---
            -589,542,597
            605,-692,669
            -500,565,-823
            -660,373,557
            -458,-679,-417
            -488,449,543
            -626,468,-788
            338,-750,-386
            528,-832,-391
            562,-778,733
            -938,-730,414
            543,643,-506
            -524,371,-870
            407,773,750
            -104,29,83
            378,-903,-323
            -778,-728,485
            426,699,580
            -438,-605,-362
            -469,-447,-387
            509,732,623
            647,635,-688
            -868,-804,481
            614,-800,639
            595,780,-596

            --- scanner 4 ---
            727,592,562
            -293,-554,779
            441,611,-461
            -714,465,-776
            -743,427,-804
            -660,-479,-426
            832,-632,460
            927,-485,-438
            408,393,-506
            466,436,-512
            110,16,151
            -258,-428,682
            -393,719,612
            -211,-452,876
            808,-476,-593
            -575,615,604
            -485,667,467
            -680,325,-822
            -627,-443,-432
            872,-547,-609
            833,512,582
            807,604,487
            839,-516,451
            891,-625,532
            -652,-548,-490
            30,-46,-14";

        assert_eq!(day19_main(test_data), (79, 3621));
    }
}