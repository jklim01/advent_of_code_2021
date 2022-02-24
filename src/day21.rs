use std::mem;
use std::cmp::Ordering;
use std::str::FromStr;
use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug)]
enum ParsePlayerError {
    InvalidPosition, InvalidId, InvalidFormat
}
impl Display for ParsePlayerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsePlayerError::InvalidPosition => write!(f, "position must be a number within 1..=10"),
            ParsePlayerError::InvalidId => write!(f, "player id must be a number"),
            ParsePlayerError::InvalidFormat => write!(f, "invalid format"),
        }
    }
}
impl Error for ParsePlayerError {}

#[derive(Debug, Clone)]
struct Player {
    id: u8,
    pos: u8,    // all positions are shifted by -1 so that the range is 0..=9, making modulus operations more convenient
    score: usize,
}
impl FromStr for Player {
    type Err = ParsePlayerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(' ');
        if let (Some("Player"), Some(id), Some("starting"), Some("position:"), Some(pos)) =
            (tokens.next(), tokens.next(), tokens.next(), tokens.next(), tokens.next()) {
            let id = id.parse().map_err(|_| ParsePlayerError::InvalidId)?;
            let pos = pos.parse::<u8>().map_err(|_| ParsePlayerError::InvalidPosition)?;
            if pos == 0 || pos > 10 {
                return Err(ParsePlayerError::InvalidPosition);
            }
            return Ok(Player {
                id, pos: pos - 1, score: 0
            });
        }
        Err(ParsePlayerError::InvalidFormat)
    }
}
impl Player {
    fn roll(&mut self, roll_val: u16) {
        self.pos = ((self.pos as u16 + roll_val) % 10) as u8;
        self.score += self.pos as usize + 1;
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Roller {
    Player1, Player2
}

#[derive(Debug)]
enum ParseGameStateError {
    IncorrectNumberOfPlayers, ParsePlayerError(ParsePlayerError), RepeatedId
}
impl Display for ParseGameStateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseGameStateError::IncorrectNumberOfPlayers => write!(f, "there must be exactly 2 players"),
            ParseGameStateError::ParsePlayerError(e) => write!(f, "ParsePlayerError ({})", e),
            ParseGameStateError::RepeatedId => write!(f, "a player id was repeated"),
        }
    }
}
impl From<ParsePlayerError> for ParseGameStateError {
    fn from(e: ParsePlayerError) -> Self {
        ParseGameStateError::ParsePlayerError(e)
    }
}
impl Error for ParseGameStateError {}

#[derive(Debug, Clone)]
struct GameState {
    player1: Player,
    player2: Player,
}
impl FromStr for GameState {
    type Err = ParseGameStateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        if let (Some(p1), Some(p2), None) = (lines.next(), lines.next(), lines.next()) {
            let player1 = p1.trim().parse::<Player>()?;
            let player2 = p2.trim().parse::<Player>()?;

            if player1.id == player2.id {
                return Err(ParseGameStateError::RepeatedId);
            }

            return Ok(GameState {
                player1, player2
            })
        }
        Err(ParseGameStateError::IncorrectNumberOfPlayers)
    }
}
impl GameState {
    fn play_deterministic(&mut self) -> usize {
        let mut iter = (1..=100_u16).cycle().step_by(3).enumerate();
        let mut current_player = &mut self.player1;
        let mut other_player = &mut self.player2;

        loop {
            let (i, first_roll_value) = iter.next().unwrap();
            let i = i + 1;

            current_player.roll(3*first_roll_value + 3);
            if current_player.score >= 1000 {
                return 3*i * other_player.score;
            }

            mem::swap(&mut current_player, &mut other_player);
        }
    }

    fn play_dirac(&self) -> Option<(u8, usize)> {
        let mut player1_wins = 0;
        let mut player2_wins = 0;
        let mut stack =  Vec::new();

        stack.push((self.clone(), Roller::Player1, 1));
        let roll_values = [
            (3, 1),     // 1 + 1 + 1
            (4, 3),     // 1 + 1 + 2
            (5, 3+3),   // 1 + 1 + 3  /  1 + 2 + 2
            (6, 6+1),   // 1 + 2 + 3  /  2 + 2 + 2
            (7, 3+3),   // 1 + 3 + 3  /  2 + 2 + 3
            (8, 3),     // 2 + 3 + 3
            (9, 1),     // 3 + 3 + 3
        ];

        while let Some((game, roller, occurences)) = stack.pop() {
            match roller {
                Roller::Player1 => {
                    for (roll_val, scaling_factor) in roll_values.iter() {
                        let occurences = occurences * scaling_factor;
                        let mut next_game = game.clone();
                        next_game.player1.roll(*roll_val);
                        if next_game.player1.score >= 21 {
                            player1_wins += occurences;
                            continue;
                        }
                        stack.push((next_game, Roller::Player2, occurences));
                    }
                },
                Roller::Player2 => {
                    for (roll_val, scaling_factor) in roll_values.iter() {
                        let occurences = occurences * scaling_factor;
                        let mut next_game = game.clone();
                        next_game.player2.roll(*roll_val);
                        if next_game.player2.score >= 21 {
                            player2_wins += occurences;
                            continue;
                        }
                        stack.push((next_game, Roller::Player1, occurences));
                    }
                },
            }
        }

        match player1_wins.cmp(&player2_wins) {
            Ordering::Equal => None,
            Ordering::Greater => Some((self.player1.id, player1_wins)),
            Ordering::Less => Some((self.player2.id, player2_wins)),
        }
    }
}

pub fn day21_main(file_data: &str) -> (usize, Option<(u8, usize)>) {
    let game = file_data.parse::<GameState>()
        .unwrap_or_else(|e| {
            panic!("error parsing the game state: {}", e);
        });

    // Part 1
    let part1_ans = game.clone().play_deterministic();
    println!("[Part 1] The product is {}.", part1_ans);

    let part2_ans = game.play_dirac();
    if let Some((winner_id, win_count)) = part2_ans {
        println!("[Part 2] Player {} takes the lead with {} wins.", winner_id, win_count);
    }
    else {
        println!("[Part 2] Both players win the same number of times.");
    }

    (part1_ans, part2_ans)
}


#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn it_works() {
        let test_data =
            "Player 1 starting position: 4
            Player 2 starting position: 8";

        assert_eq!(day21_main(test_data), (739785, Some((1, 444356092776315))));
    }
}