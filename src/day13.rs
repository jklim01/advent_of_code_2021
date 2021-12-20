use std::str;
use std::fmt;
use std::iter;
use std::cmp::Ordering;
use std::error::Error;
use std::num::ParseIntError;
use std::collections::HashSet;
use std::collections::BTreeSet;

// ParsePointError
#[derive(Debug)]
enum ParsePointError {
    InvalidFormat, InvalidNum(ParseIntError)
}
impl From<ParseIntError> for ParsePointError {
    fn from(e: ParseIntError) -> Self {
        ParsePointError::InvalidNum(e)
    }
}
impl fmt::Display for ParsePointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsePointError::InvalidFormat => write!(f, "points must be a pair of comma-separated numbers"),
            ParsePointError::InvalidNum(_) => write!(f, "coordinates must be u16's"),
        }
    }
}
impl Error for ParsePointError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParsePointError::InvalidFormat => None,
            ParsePointError::InvalidNum(e) => Some(e),
        }
    }
}

// Point
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point(u16, u16);
impl str::FromStr for Point {
    type Err = ParsePointError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coordinates = s.split(',');
        if let (Some(x), Some(y), None) = (coordinates.next(), coordinates.next(), coordinates.next()) {
            let x = x.trim().parse()?;
            let y = y.trim().parse()?;
            Ok(Point(x, y))
        }
        else { Err(ParsePointError::InvalidFormat) }
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.1.cmp(&other.1) {
            Ordering::Equal => Some(self.0.cmp(&other.0)),
            x => Some(x),
        }
    }
}
impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.1.cmp(&other.1) {
            Ordering::Equal => self.0.cmp(&other.0),
            x => x,
        }
    }
}

// ParseFoldError
#[derive(Debug)]
enum ParseFoldError {
    InvalidFormat, InvalidFoldAxis, InvalidNum(ParseIntError)
}
impl From<ParseIntError> for ParseFoldError {
    fn from(e: ParseIntError) -> Self {
        ParseFoldError::InvalidNum(e)
    }
}
impl fmt::Display for ParseFoldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseFoldError::InvalidFormat => write!(f, "invalid fold representation format"),
            ParseFoldError::InvalidFoldAxis => write!(f, "folds can only be along the x or y axis"),
            ParseFoldError::InvalidNum(_) => write!(f, "fold location must be a u16"),
        }
    }
}
impl Error for ParseFoldError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseFoldError::InvalidFormat => None,
            ParseFoldError::InvalidFoldAxis => None,
            ParseFoldError::InvalidNum(e) => Some(e),
        }
    }
}

// Fold
enum Fold {
    X(u16), Y(u16)
}
impl str::FromStr for Fold {
    type Err = ParseFoldError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.trim().split_whitespace();
        if (Some("fold"), Some("along")) != (tokens.next(), tokens.next()) {
            return Err(ParseFoldError::InvalidFormat);
        }

        let mut fold_data = tokens.next().ok_or(ParseFoldError::InvalidFormat)?.split('=');
        if let (Some(axis), Some(location), None) = (fold_data.next(), fold_data.next(), fold_data.next()) {
            let location = location.trim().parse()?;
            match axis {
                "x" => Ok(Fold::X(location)),
                "y" => Ok(Fold::Y(location)),
                _ => Err(ParseFoldError::InvalidFoldAxis),
            }
        }
        else { Err(ParseFoldError::InvalidFormat) }
    }
}
impl fmt::Display for Fold {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fold::X(x) => write!(f, "fold along x={}", x),
            Fold::Y(y) => write!(f, "fold along y={}", y),
        }
    }
}


// Paper
struct Paper {
    points: BTreeSet<Point>,
    size: Point
}
impl iter::FromIterator<Point> for Paper {
    fn from_iter<T: IntoIterator<Item = Point>>(iter: T) -> Self {
        let mut paper = Paper {
            points: BTreeSet::new(), size: Point(0, 0),
        };
        for point in iter {
            paper.points.insert(point);
            if point.0 > paper.size.0 { paper.size.0 = point.0; }
            if point.1 > paper.size.1 { paper.size.1 = point.1; }
        }
        paper
    }
}
impl Paper {
    fn count_points(&self) -> u16 {
        self.points.len() as u16
    }

    fn fold_paper(&mut self, fold: &Fold) -> bool {
        let mut new_points = HashSet::new();
        match fold {
            Fold::X(x) => {
                if *x == 0 || *x >= self.size.0 { return false; }
                self.size.0 = x-1;
                self.points.retain(|point| {
                    match point.0.cmp(&(self.size.0+1)) {
                        Ordering::Less => true,     // leave interior points untouched
                        Ordering::Equal => false,   // remove points on folding line
                        Ordering::Greater => {
                            new_points.insert(Point(2*(self.size.0+1) - point.0, point.1));
                            false
                        }
                    }
                });
            },
            Fold::Y(y) => {
                if *y == 0 || *y >= self.size.1 { return false; }
                self.size.1 = y-1;
                self.points.retain(|point| {
                    match point.1.cmp(&(self.size.1+1)) {
                        Ordering::Less => true,
                        Ordering::Equal => false,
                        Ordering::Greater => {
                            new_points.insert(Point(point.0, 2*(self.size.1+1) - point.1));
                            false
                        }
                    }
                });
            },
        };
        self.points.extend(new_points);
        true
    }
}
impl fmt::Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut cursor = Point(0, 0);
        let mut buf = String::with_capacity(((self.size.0+2) as usize)*((self.size.1+1) as usize));
        let fill_blanks =
            |buf: &mut String, cursor: &mut Point, dest: &Point| {
                match (*cursor).cmp(dest) {
                    Ordering::Greater | Ordering::Equal  => return,
                    Ordering::Less => (),
                }
                // adjust y position
                for _ in 1..=(dest.1-cursor.1) {
                    for _ in 1..=(self.size.0-cursor.0+1) { buf.push('.'); }
                    buf.push('\n');
                    cursor.0 = 0;
                }
                // adjust x position
                for _ in 1..=(dest.0-cursor.0) { buf.push('.'); }
                *cursor = *dest;
            };
        self.points.iter().for_each(|point| {
            fill_blanks(&mut buf, &mut cursor, point);
            buf.push('#');
            if cursor.0 == self.size.0 {
                buf.push('\n');
                cursor.0 = 0;
                cursor.1 += 1;
            }
            else { cursor.0 += 1; }
        });
        fill_blanks(&mut buf, &mut cursor, &Point(self.size.0+1, self.size.1));
            // Point(self.size.0+1, self.size.1)) is a hack to ensure the last point is filled in
        write!(f, "{}", buf)
    }
}



pub fn day13_main(file_data: &str) -> u16 {
    let block_delimiter;
    if file_data.chars().nth(file_data.lines().next()
        .unwrap_or_else(|| panic!("File is empty!")).len()).unwrap() == '\r' {
            block_delimiter = "\r\n\r\n";
        }
    else { block_delimiter = "\n\n"; }

    let mut file_blocks = file_data.split(block_delimiter);
    let mut paper = file_blocks.next().unwrap_or("").lines().enumerate()
        .map(|(i, line)| {
            line.parse::<Point>().unwrap_or_else(|e| {
                match e.source() {
                    None => panic!("Error parsing point on line {}! :\nParsePointError{{ {} }}", i+1, e),
                    Some(src) =>
                        panic!("Error parsing point on line {}! :\nParsePointError{{ {} }}, \
                        caused by ParseIntError{{ {} }}", i+1, e, src),
                }
            })
        })
        .collect::<Paper>();
    let mut folds = file_blocks.next().unwrap_or("").lines().map(str::parse::<Fold>);

    // Part 1
    let first_fold = folds.next().unwrap_or_else(|| panic!("No folds to do!"));
    match first_fold {
        Ok(fold) => {
            if !paper.fold_paper(&fold) {
                panic!("Error: Fold 1 {{ {} }} is invalid!", fold);
            }
        },
        Err(e) => {
            if let Some(src) = e.source() {
                panic!("Error parsing fold 1! : {}, caused by ParseIntError{{ {} }}", e, src);
            }
            else { panic!("Error parsing fold 1! : {}", e); }
        },
    }
    let part1_point_count = paper.count_points();
    println!("[Part 1] The number of points after 1 fold is {}.", part1_point_count);

    // Part 2
    for (i, res) in folds.enumerate() {
        let i = i+2;
        match res {
            Ok(fold) => {
                if !paper.fold_paper(&fold) {
                    panic!("Error: Fold {} {{ {} }} is invalid!", i, fold);
                }
            },
            Err(e) => {
                if let Some(src) = e.source() {
                    panic!("Error parsing fold {}! : {}, caused by ParseIntError{{ {} }}", i, e, src);
                }
                else { panic!("Error parsing fold {}! : {}", i, e); }
            },
        }
    }
    println!("After completing all folds, paper = \n{}", paper);

    part1_point_count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_data =
            "6,10
            0,14
            9,10
            0,3
            10,4
            4,11
            6,0
            6,12
            4,1
            0,13
            10,12
            3,4
            3,0
            8,4
            1,10
            2,14
            8,10
            9,0

            fold along y=7
            fold along x=5";

        assert_eq!(day13_main(test_data), 17);
    }
}