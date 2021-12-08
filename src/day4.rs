use std::process;

#[derive(Clone)]
struct BingoBoard(Vec<u8>);
impl BingoBoard {
    fn new(board_str: &str) -> Result<Self, usize> {
        let board = board_str.split_whitespace().enumerate()
        .map(|(i, s)| {
            s.parse::<u8>().map_err(|_| i)
        })
        .collect::<Result<Vec<u8>, usize>>()?;
        if board.len() != 25 { return Err(25); }
        Ok(BingoBoard(board))
    }

    fn mark(&mut self, num: u8) -> bool {
        let mut is_marked = false;
        for i in 0..25 {
            if self.0[i] == num {
                self.0[i] = u8::MAX;
                is_marked = true;
            }
        }
        is_marked
    }

    fn check(&self) -> bool {
        for i in 0..5 {
            let mut row_i = self.0.iter().skip(5*i).take(5);
            if row_i.all(|x| *x == u8::MAX) { return true; }

            let mut col_i = self.0.iter().skip(i).step_by(5);
            if col_i.all(|x| *x == u8::MAX) { return true; }
        }
        false
    }

    fn mark_and_check(&mut self, num: u8) -> bool {
        if !self.mark(num) { return false; }
        self.check()
    }

    fn sum_unmarked(&self) -> u16 {
        self.0.iter().fold(0, |acc, x| {
            if *x == u8::MAX { return acc; }
            acc + (*x as u16)
        })
    }
}

pub fn day4_main(file_data: &str) -> (u16, u16) {
    // split file into blocks using 2 new lines
    let block_delimiter;
    if file_data.chars().nth(file_data.lines().nth(0).unwrap_or_else(|| {
        eprintln!("File is empty!");
        process::exit(1);
    }).len()).unwrap() == '\r' { block_delimiter = "\r\n\r\n"; }
    else { block_delimiter = "\n\n"; }
    let mut file_blocks = file_data.split(block_delimiter);

    // get list of bingo numbers
    let mut nums = file_blocks.next().unwrap()
        .split(",")
        .enumerate()
        .map(|(i, s)| {
            s.parse::<u8>().unwrap_or_else(|e| {
                eprintln!("Error parsing bingo number {}: ParseIntError {{ {} }}", i, e);
                eprintln!("Instead of u8, \"{}\" was encountered!", s);
                process::exit(1);
            })
        });

    // create vector of BingoBoard
    let mut all_boards = file_blocks
        .enumerate()
        .map(|(i, block)| {
            BingoBoard::new(block).unwrap_or_else(|e| {
                eprintln!("Error parsing bingo board {} (line {}~{}):", i, 3+6*i, 7+6*i);
                if e == 25 { eprintln!("each board can only have {} numbers", 25); }
                else { eprintln!("invalid value in the position {} of the board", e+1); }
                process::exit(1);
            })
        })
        .collect::<Vec<BingoBoard>>();


    // Part 1
    let mut num;
    let mut first_is_found = false;
    let score1 = loop {
        num = nums.next().unwrap();
        let mut score1 = 0;

        // mark and check all boards with current bingo number, break with score if found
        all_boards.iter_mut().enumerate().for_each(|(i, board)| {
            if board.mark_and_check(num) && !first_is_found {
                println!("Board {} is the first to win!", i+1);
                score1 = (num as u16) * board.sum_unmarked();
                first_is_found = true;
            }
        });
        if first_is_found { break score1; }
    };
    println!("The score of this board is {}.", score1);


    // Part 2
    let last_board_completed = loop {
        num = nums.next().unwrap();

        // mark all boards, then remove all winning boards
        all_boards.iter_mut().for_each(|board| { board.mark(num); });
        let last_in_list_copy = (*all_boards.last().unwrap()).clone();
        all_boards.retain(|board| !board.check());

        // play the last remaning board till bingo and get score
        if all_boards.len() ==  1 {
            let mut last_board = all_boards.pop().unwrap();
            while {
                num = nums.next().unwrap();
                !last_board.mark_and_check(num)
            } {}
            break last_board
        }

        // if no boards remain after this pass, the last board is taken to be the last to win
        if all_boards.len() == 0 { break last_in_list_copy; }
    };
    let score2 = (num as u16) * last_board_completed.sum_unmarked();
    println!("The score of the last board to win is {}.", score2);

    (score1, score2)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_data =
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

            22 13 17 11  0
             8  2 23  4 24
            21  9 14 16  7
             6 10  3 18  5
             1 12 20 15 19

             3 15  0  2 22
             9 18 13 17  5
            19  8  7 25 23
            20 11 10 24  4
            14 21 16 12  6

            14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
             2  0 12  3  7";
        assert_eq!(day4_main(test_data), (4512, 1924));
    }
}