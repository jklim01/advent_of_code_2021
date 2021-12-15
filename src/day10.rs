use std::cmp::Ordering;
use std::fmt;
use std::ops;

#[derive(PartialEq, Clone, Copy)]
enum BracketType {
    Round, Square, Curly, Angle
}
#[derive(PartialEq, Clone, Copy)]
enum BracketMode {
    Open, Close
}
#[derive(Clone, Copy)]
struct Bracket(BracketType, BracketMode);
impl Bracket {
    fn parse_bracket(c: char) -> Option<Self> {
        match c {
            '(' => Some(Bracket(BracketType::Round, BracketMode::Open)),
            ')' => Some(Bracket(BracketType::Round, BracketMode::Close)),
            '[' => Some(Bracket(BracketType::Square, BracketMode::Open)),
            ']' => Some(Bracket(BracketType::Square, BracketMode::Close)),
            '{' => Some(Bracket(BracketType::Curly, BracketMode::Open)),
            '}' => Some(Bracket(BracketType::Curly, BracketMode::Close)),
            '<' => Some(Bracket(BracketType::Angle, BracketMode::Open)),
            '>' => Some(Bracket(BracketType::Angle, BracketMode::Close)),
            _ => None
        }
    }

    #[inline]
    fn get_score_val(&self, m_type: ScoreType) -> u64 {
        match m_type {
            ScoreType::Error => {
                match self.0 {
                    BracketType::Round => 3,
                    BracketType::Square => 57,
                    BracketType::Curly => 1197,
                    BracketType::Angle => 25137,
                }
            }
            ScoreType::Completion => {
                match self.0 {
                    BracketType::Round => 1,
                    BracketType::Square => 2,
                    BracketType::Curly => 3,
                    BracketType::Angle => 4,
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ScoreType {
    Error, Completion
}
impl fmt::Display for ScoreType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScoreType::Error => write!(f, "SyntaxErrorScore"),
            ScoreType::Completion => write!(f, "CompletionScore"),
        }
    }
}
#[derive(Debug, Eq, Clone, Copy)]
pub struct Score {
    val: u64, m_type: ScoreType
}
impl Score {
    fn new(val: u64, m_type: ScoreType) -> Self {
        Score { val, m_type }
    }
}
impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}
impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.val.partial_cmp(&other.val)
    }
}
impl PartialEq for Score {
    fn eq(&self, other: &Self) -> bool {
        self.val.eq(&other.val)
    }
}
impl ops::AddAssign for Score {
    fn add_assign(&mut self, other: Self) {
        self.val = self.val + other.val;
    }
}
impl ops::Add<u64> for Score {
    type Output = Score;
    fn add(self, rhs: u64) -> Self {
        Score {
            val: self.val + rhs,
            m_type: self.m_type,
        }
    }
}
impl ops::Mul<u64> for Score {
    type Output = Score;
    fn mul(self, rhs: u64) -> Self {
        Score {
            val: self.val * rhs,
            m_type: self.m_type,
        }
    }
}
impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.m_type, self.val)
    }
}

struct BracketStack(Vec<Bracket>);
impl BracketStack {
    fn new() -> Self {
        BracketStack(Vec::new())
    }

    fn add_bracket(&mut self, bracket: Bracket) -> bool {
        if bracket.1 == BracketMode::Open {
            self.0.push(bracket);
            return true;
        }
        if let Some(last) = self.0.last() {
            if last.0 == bracket.0 {
                self.0.pop();
                return true;
            }
        }
        false
    }
}

fn get_score(line: &str) -> Result<Score, usize> {
    let mut stack = BracketStack::new();
    for (j, c) in line.trim().chars().enumerate() {
        let bracket = Bracket::parse_bracket(c).ok_or(j+1)?;
        if !stack.add_bracket(bracket) {
            return Ok(Score::new(bracket.get_score_val(ScoreType::Error), ScoreType::Error));
        }
    }
    Ok(stack.0.iter().rev().fold(Score{ val: 0, m_type: ScoreType::Completion }, |acc, bracket| {
        (acc * 5) + bracket.get_score_val(ScoreType::Completion)
    }))
}


pub fn day10_main(file_data: &str) -> (Score, Score) {
    let mut part1_score = Score::new(0, ScoreType::Error);
    let mut completion_scores = file_data
        .lines().enumerate()
        .flat_map(|(i, line)| {
            let score = get_score(line).unwrap_or_else(|j|
                panic!("Error parsing: invalid character at line {} ({})", i, j)
            );
            if matches!(score.m_type, ScoreType::Completion) { return Some(score); }
            part1_score += score;
            None
        })
        .collect::<Vec<_>>();

    completion_scores.sort_unstable();
    let part2_score = completion_scores[completion_scores.len() / 2];

    println!("[Part 1] The total syntax error score is {}.", part1_score);
    println!("[Part 2] The autocomplete score is {}.", part2_score);

    (part1_score, part2_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_data =
            "[({(<(())[]>[[{[]{<()<>>
            [(()[<>])]({[<{<<[]>>(
            {([(<{}[<>[]}>{[]{[(<()>
            (((({<>}<{<{<>}{[]{[]{}
            [[<[([]))<([[{}[[()]]]
            [{[{({}]{}}([{[{{{}}([]
            {<[[]]>}<{[{[{[]{()[[[]
            [<(<(<(<{}))><([]([]()
            <{([([[(<>()){}]>(<<{{
            <{([{{}}[<[[[<>{}]]]>[]]";

        assert_eq!(day10_main(test_data),
            (Score::new(26397, ScoreType::Error), Score::new(288957, ScoreType::Completion)));
    }
}