use std::fmt::{Display, self};

use super::board::Board;

pub struct Bingo
{
    pub boards: Vec<Board>,
    pub called: Vec<i32>,
    // pub winning_score: i32,
    pub won_boards: Vec<usize>,
    pub won_scores: Vec<i32>
}

impl Bingo
{
    pub fn new() -> Bingo
    {
        Bingo
        {
            boards: Vec::new(),
            called: Vec::new(),
            // winning_score: 0,
            won_boards: Vec::new(),
            won_scores: Vec::new()
        }
    }

    pub fn new_board(&mut self) -> usize
    {
        self.boards.push(Board::new());
        return self.boards.len() - 1;
    }

    pub fn load_board(&mut self, board: Board) -> usize
    {
        self.boards.push(board);
        return self.boards.len() - 1;
    }

    pub fn set_number_on_board(&mut self, board: usize, row: usize, col: usize, value: i32)
    {
        self.boards[board].insert_number(row, col, value);
    }

    pub fn call(&mut self, value: i32) -> bool
    {
        let mut win = false;
        let mut count = 0;
        
        while count < self.boards.len()
        // for board in &mut self.boards
        {
            let board = &mut self.boards[count];
            // if self.won_boards.contains(&count)
            // {
            //     println!("Board {} has already won.  Skipping further calls.", count);
            //     continue;
            // }
            println!("Calling number {} on board {}", value, count);
            win = board.call_number(value);
            if win
            {
                let board_total = board.sum_uncalled();
                println!("Total for the winning board: {}", board_total);
                // self.winning_score = board_total * value;
                // self.won_boards.push(count);
                self.won_scores.push(board_total * value);
                self.boards.remove(count);
                // break;
            }
            else 
            {
                count += 1;
            }
        }

        return win;
    }
}

impl Display for Bingo
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) ->fmt::Result
    {
        let mut buffer = String::new();

        for board in &self.boards
        {
            let mut row = 0;
            while row < 5
            {
                buffer.push_str
                (
                    &format!("{} {} {} {} {}\n", 
                        board.rows[row][0], 
                        board.rows[row][1],
                        board.rows[row][2],
                        board.rows[row][3],
                        board.rows[row][4]
                    ).to_string()
                );

                row += 1;
            }
            buffer.push_str("\n");
        }

        write!(f, "{}", buffer)
    }

    
}