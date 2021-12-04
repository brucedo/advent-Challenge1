use super::board::Board;

pub struct Bingo
{
    pub boards: Vec<Board>,
    pub called: Vec<i32>,
    pub winning_score: i32,
}

impl Bingo
{
    pub fn new() -> Bingo
    {
        Bingo
        {
            boards: Vec::new(),
            called: Vec::new(),
            winning_score: 0,
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
        for board in &mut self.boards
        {
            println!("Calling number {} on board {}", value, count);
            win = board.call_number(value);
            if win
            {
                let board_total = board.sum_uncalled();
                println!("Total for the winning board: {}", board_total);
                self.winning_score = board_total * value;
            }

            count += 1;
        }

        return win;
    }
}