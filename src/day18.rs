use std::str;
use std::ops::Add;
use std::iter::Sum;
use std::error::Error;
use std::num::ParseIntError;
use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Eq, Clone, Copy)]
enum SnailfishToken {
    OpeningBrace, ClosingBrace, Comma, Literal(u8)
}
struct SnailfishTokenStream<'a>(&'a str, usize);
impl<'a> Iterator for SnailfishTokenStream<'a> {
    type Item = (Result<SnailfishToken, ParseIntError>, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.1;
        let symbol = match self.0.chars().next() {
            Some(c @ ('[' | ']' | ',')) => {
                self.0 = self.0.strip_prefix(c).unwrap().trim_start();
                self.1 += 1;
                match c {
                    '[' => Ok(SnailfishToken::OpeningBrace),
                    ']' => Ok(SnailfishToken::ClosingBrace),
                    ',' => Ok(SnailfishToken::Comma),
                    _ => unreachable!(),
                }
            },
            None => return None,
            _ => {
                let literal = {
                    if let Some(split_point) = self.0.find(&['[', ']', ','][..]) {
                        let (str, remaining) = self.0.split_at(split_point);
                        self.0 = remaining.trim_start();
                        self.1 += split_point;
                        str.trim_end()
                    }
                    else {
                        let str = self.0;
                        self.1 += self.0.len() - 1;
                        self.0 = &self.0[self.0.len()..];
                        str
                    }
                };
                literal.parse().map(SnailfishToken::Literal)
            },
        };
        Some((symbol, pos))
    }
}
impl<'a> SnailfishTokenStream<'a> {
    fn new(s: &'a str) -> Self {
        SnailfishTokenStream(s.trim(), 0)
    }
}

#[derive(Debug)]
enum ParseSnailfishNumberErrorKind {
    ParseRegularError(ParseIntError), UnresolvedTrailingChars, ExpectedComma, ExpectedClosingBrace, ExpectedNumber
}
impl Display for ParseSnailfishNumberErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            ParseSnailfishNumberErrorKind::ExpectedComma => write!(f, "expected ','"),
            ParseSnailfishNumberErrorKind::ExpectedNumber => write!(f, "expected number"),
            ParseSnailfishNumberErrorKind::ParseRegularError(_) => write!(f, "ParseIntError"),
            ParseSnailfishNumberErrorKind::ExpectedClosingBrace => write!(f, "expected ']'"),
            ParseSnailfishNumberErrorKind::UnresolvedTrailingChars => write!(f, "unresolved trailing characters"),
        }
    }
}
impl From<ParseIntError> for ParseSnailfishNumberErrorKind {
    fn from(e: ParseIntError) -> Self {
        ParseSnailfishNumberErrorKind::ParseRegularError(e)
    }
}

#[derive(Debug)]
struct ParseSnailfishNumberError {
    pos: usize,
    kind: ParseSnailfishNumberErrorKind,
}
impl Display for ParseSnailfishNumberError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} at index {}", self.kind, self.pos)
    }
}
impl Error for ParseSnailfishNumberError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            ParseSnailfishNumberErrorKind::ParseRegularError(e) => Some(e),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum SnailfishNumber {
    Zero,
    Regular(u8),
    Pair {
        left: Box<SnailfishNumber>,
        right: Box<SnailfishNumber>,
    },
}
impl SnailfishNumber {
    const ZERO: SnailfishNumber = SnailfishNumber::Zero;

    fn get_magnitude(&self) -> u16 {
        match self {
            SnailfishNumber::Regular(x) => *x as u16,
            SnailfishNumber::Pair { left, right } => {
                3*left.get_magnitude() + 2*right.get_magnitude()
            },
            SnailfishNumber::Zero => panic!("magnitude undefined for SnailfishNumber::Zero"),
        }
    }

    fn split(&mut self) -> bool {
        if let SnailfishNumber::Pair { left, right} = self {
            // perform in-order traversal
            let mut stack = vec![&mut **right, &mut **left];
            while let Some(num) = stack.pop() {
                match num {
                    &mut SnailfishNumber::Regular(val) if val >= 10 => {
                        *num = SnailfishNumber::Pair {
                            left: Box::new(SnailfishNumber::Regular(val / 2)),
                            right: Box::new(SnailfishNumber::Regular((val-1)/2 + 1)),
                        };
                        return true;
                    },
                    SnailfishNumber::Pair { left, right} => {
                        stack.push(right);
                        stack.push(left);
                    },
                    _ => (),
                }
            }
        }
        false
    }

    fn explosion_from_left(&mut self, explosion: u8) {
        match self {
            SnailfishNumber::Regular(val) => *val += explosion,
            SnailfishNumber::Pair { left, .. } => left.explosion_from_left(explosion),
            SnailfishNumber::Zero => panic!("undefined case for explosion into SnailfishNumber::Zero"),
        }
    }
    fn explosion_from_right(&mut self, explosion: u8) {
        match self {
            SnailfishNumber::Regular(val) => *val += explosion,
            SnailfishNumber::Pair { right, .. } => right.explosion_from_right(explosion),
            SnailfishNumber::Zero => panic!("undefined case for explosion into SnailfishNumber::Zero"),
        }
    }

    fn explode(&mut self, depth: u8) -> ((Option<u8>, Option<u8>), bool) {
        // all unresolved explosions are returned to "propagate" them outwards
        let mut has_exploded = false;
        let propagated_explosion = match self {
            SnailfishNumber::Pair { left, right } => {
                // found a pair to explode
                if depth == 5 {
                    if let (SnailfishNumber::Regular(left), SnailfishNumber::Regular(right)) =
                        ((**left).clone(), (**right).clone()) {
                        has_exploded = true;
                        *self = SnailfishNumber::Regular(0);
                        (Some(left), Some(right))
                    }
                    else {
                        panic!("a snailfish number with more than 5 layers of brackets is invalid");
                    }
                }
                // continue traversing deeper to find possible a possible pair to explode
                else {
                    // prioritize the left side as we want to find the "leftmost such pair"
                    let (explosion, flag) = left.explode(depth+1);
                    if flag {
                        has_exploded = true;
                        match explosion {
                            (to_left, Some(to_right)) => {
                                right.explosion_from_left(to_right);
                                (to_left, None) // the unresolved explosion towards the left is propogated outwards
                            },

                            // unable to resolve any explosions at this level, so just propagate it outwards
                            unresolved_explosions => unresolved_explosions,
                        }
                    }
                    // no explosions on the left, so we can check the right
                    else {
                        let (explosion, flag) = right.explode(depth+1);
                        if flag {
                            has_exploded = true;
                            match explosion {
                                (Some(to_left), to_right) => {
                                    left.explosion_from_right(to_left);
                                    (None, to_right) // the unresolved explosion towards the right is propogated outwards
                                },

                                // unable to resolve any explosions at this level, so just propagate it outwards
                                unresolved_explosions => unresolved_explosions,
                            }
                        }
                        else { (None, None) }
                    }
                }
            },
            _ => (None, None),
        };
        (propagated_explosion, has_exploded)
    }

    fn reduce(&mut self) {
        loop {
            if self.explode(1).1 { continue; }
            if self.split() { continue; }
            break;
        }
    }
}

impl Display for SnailfishNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            SnailfishNumber::Zero => write!(f, "ZERO"),
            SnailfishNumber::Regular(x) => write!(f, "{}", x),
            SnailfishNumber::Pair { left, right } => {
                write!(f, "[{}, {}]", **left, **right)
            }
        }
    }
}
impl str::FromStr for SnailfishNumber {
    type Err = ParseSnailfishNumberError;
    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        #[derive(PartialEq, Eq, Clone, Copy)]
        enum Expected {
            Number, ClosingBrace, Comma
        }
        impl From<Expected> for ParseSnailfishNumberErrorKind {
            fn from(e: Expected) -> Self {
                match e {
                    Expected::Number => ParseSnailfishNumberErrorKind::ExpectedNumber,
                    Expected::Comma => ParseSnailfishNumberErrorKind::ExpectedComma,
                    Expected::ClosingBrace => ParseSnailfishNumberErrorKind::ExpectedClosingBrace,
                }
            }
        }
        impl SnailfishToken {
            fn matches_expectation(self, expected: Expected) -> bool {
                match expected {
                    Expected::Comma => self == SnailfishToken::Comma,
                    Expected::ClosingBrace => self == SnailfishToken::ClosingBrace,
                    Expected::Number => matches!(self, SnailfishToken::Literal(_) | SnailfishToken::OpeningBrace),
                }
            }
        }

        s = s.trim();
        let mut stream = SnailfishTokenStream::new(s);
        let mut open_pairs = Vec::new();
        let mut error_pos = 0;
        let mut expected = Expected::Number;

        while let Some((symbol, pos)) = stream.next() {
            let symbol = symbol
                .map_err(|e| ParseSnailfishNumberError { pos, kind: e.into() })?;
            error_pos = pos;
            if !symbol.matches_expectation(expected) { break; }

            match symbol {
                SnailfishToken::OpeningBrace => open_pairs.push((None, None)),

                SnailfishToken::Comma => expected = Expected::Number,

                SnailfishToken::Literal(literal) => {
                    let num = SnailfishNumber::Regular(literal);
                    let current_pair = open_pairs.iter_mut().rev().next();

                    if let Some(current_pair) = current_pair {
                        if current_pair.0.is_none() {
                            current_pair.0 = Some(Box::new(num));
                            expected = Expected::Comma;
                        }
                        else {
                            current_pair.1 = Some(Box::new(num));
                            expected = Expected::ClosingBrace;
                        }
                    }
                    else {
                        if stream.next().is_none() {
                            return Ok(SnailfishNumber::Regular(literal));
                        }
                        return Err(ParseSnailfishNumberError {
                            pos,
                            kind: ParseSnailfishNumberErrorKind::UnresolvedTrailingChars,
                        });
                    }
                },

                SnailfishToken::ClosingBrace => {
                    let resolved_pair = open_pairs.pop().unwrap();
                    let resolved_pair = SnailfishNumber::Pair {
                        left: resolved_pair.0.unwrap(),
                        right: resolved_pair.1.unwrap(),
                    };
                    let next_pair = open_pairs.iter_mut().rev().next();

                    if let Some(next_pair) = next_pair {
                        if next_pair.0.is_none() {
                            next_pair.0 = Some(Box::new(resolved_pair));
                            expected = Expected::Comma;
                        }
                        else {
                            next_pair.1 = Some(Box::new(resolved_pair));
                            expected = Expected::ClosingBrace;
                        }
                    }
                    else {
                        if stream.next().is_none() {
                            return Ok(resolved_pair);
                        }
                        return Err(ParseSnailfishNumberError {
                            pos,
                            kind: ParseSnailfishNumberErrorKind::UnresolvedTrailingChars
                        });
                    }
                },
            }
        }

        Err(ParseSnailfishNumberError {
            pos: error_pos, kind: expected.into()
        })
    }
}
impl Add<Self> for SnailfishNumber {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if self == SnailfishNumber::Zero { return rhs; }
        if rhs == SnailfishNumber::Zero { return self; }
        let mut result = SnailfishNumber::Pair {
            left: Box::new(self),
            right: Box::new(rhs),
        };
        result.reduce();
        result
    }
}
impl Sum<Self> for SnailfishNumber {
    fn sum<I>(iter: I) -> Self where I: Iterator<Item = Self> {
        iter.fold(SnailfishNumber::ZERO, |acc, x| acc + x)
    }
}



pub fn day18_main(file_data: &str) -> (u16, u16) {
    let nums = file_data.lines().enumerate()
        .map(|(i, num)| num.parse::<SnailfishNumber>().unwrap_or_else(|e| {
            if let Some(src) = e.source() {
                panic!("Error parsing SnailfishNumber on line {}: \n {}, caused by {}", i+1, e, src);
            }
            else { panic!("Error parsing SnailfishNumber on line {}: \n {}", i+1, e); }
        }))
        .collect::<Vec<_>>();

    // Part 1
    let result = nums.iter().cloned().sum::<SnailfishNumber>().get_magnitude();
    println!("[Part 1] The magnitude of the sum is {}.", result);

    // Part 2
    let largest_magnitude = nums.iter()
        .enumerate()
        .map(|(i, num)| {
            nums.iter().cloned().skip(i+1).map(|other| {
                [(num.clone() + other.clone()).get_magnitude(), (other + num.clone()).get_magnitude()]
            })
            .flatten()
        })
        .flatten()
        .max()
        .expect("there are no numbers to sum");

    println!("[Part 2] The possible largest magnitude is {}.", largest_magnitude);

    (result, largest_magnitude)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_regular() {
        let test_data = "0";
        let expected = SnailfishNumber::Regular(0);
        assert_eq!(test_data.parse::<SnailfishNumber>().unwrap(), expected);
    }

    #[test]
    fn test_parsing_pair() {
        let test_data = "[1, 2]";
        let expected = SnailfishNumber::Pair {
            left: Box::new(SnailfishNumber::Regular(1)),
            right: Box::new(SnailfishNumber::Regular(2)),
        };
        assert_eq!(test_data.parse::<SnailfishNumber>().unwrap(), expected);
    }

    #[test]
    fn test_parsing_nested_pair() {
        let test_data = "[1, [2, 3]]";
        let expected = SnailfishNumber::Pair {
            left: Box::new(SnailfishNumber::Regular(1)),
            right: Box::new(SnailfishNumber::Pair {
                left: Box::new(SnailfishNumber::Regular(2)),
                right: Box::new(SnailfishNumber::Regular(3)),
            }),
        };
        assert_eq!(test_data.parse::<SnailfishNumber>().unwrap(), expected);
    }

    #[test]
    fn test_addition() {
        let num1 = "[[[[4,3],4],4],[7,[[8,4],9]]]".parse::<SnailfishNumber>().unwrap();
        let num2 = "[1,1]".parse().unwrap();
        let result = num1 + num2;
        let expected = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse().unwrap();

        println!("result \t\t= {}", result);
        println!("expected \t= {}", expected);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sum_basic() {
        let nums =
            "[1,1]
            [2,2]
            [3,3]
            [4,4]
            [5,5]";
        let nums = nums.lines()
            .map(|line| line.parse::<SnailfishNumber>().unwrap());
        let sum = nums.sum::<SnailfishNumber>();
        let expected = "[[[[3,0],[5,3]],[4,4]],[5,5]]".parse().unwrap();

        println!("sum \t\t= {}", sum);
        println!("expected \t= {}", expected);
        assert_eq!(sum, expected);
    }

    #[test]
    fn test_sum() {
        let nums =
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
            [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
            [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
            [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
            [7,[5,[[3,8],[1,4]]]]
            [[2,[2,2]],[8,[8,1]]]
            [2,9]
            [1,[[[9,3],9],[[9,0],[0,7]]]]
            [[[5,[7,4]],7],1]
            [[[[4,2],2],6],[8,7]]";
        let nums = nums.lines()
            .map(|line| line.parse::<SnailfishNumber>().unwrap());
        let sum = nums.sum::<SnailfishNumber>();
        let expected = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".parse().unwrap();

        println!("sum \t\t= {}", sum);
        println!("expected \t= {}", expected);
        assert_eq!(sum, expected);
    }

    #[test]
    fn test_magnitude() {
        assert_eq!("[[1,2],[[3,4],5]]".parse::<SnailfishNumber>().unwrap().get_magnitude(),
            143);
        assert_eq!("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse::<SnailfishNumber>().unwrap().get_magnitude(),
            1384);
        assert_eq!("[[[[1,1],[2,2]],[3,3]],[4,4]]".parse::<SnailfishNumber>().unwrap().get_magnitude(),
            445);
        assert_eq!("[[[[3,0],[5,3]],[4,4]],[5,5]]".parse::<SnailfishNumber>().unwrap().get_magnitude(),
            791);
        assert_eq!("[[[[5,0],[7,4]],[5,5]],[6,6]]".parse::<SnailfishNumber>().unwrap().get_magnitude(),
            1137);
        assert_eq!("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".parse::<SnailfishNumber>().unwrap().get_magnitude(),
            3488);
    }

    #[test]
    fn test_example() {
        let test_data =
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
            [[[5,[2,8]],4],[5,[[9,9],0]]]
            [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
            [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
            [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
            [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
            [[[[5,4],[7,7]],8],[[8,3],8]]
            [[9,3],[[9,9],[6,[4,9]]]]
            [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
            [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        let nums = test_data.lines()
            .map(|line| line.parse::<SnailfishNumber>().unwrap());

        let sum = nums.sum::<SnailfishNumber>();
        let expected = "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]".parse().unwrap();
        assert_eq!(sum, expected);

        let (magnitude, largest_pair_magnitude) = day18_main(test_data);
        assert_eq!(magnitude, 4140);
        assert_eq!(largest_pair_magnitude, 3993);
    }
}