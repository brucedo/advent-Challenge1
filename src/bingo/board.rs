pub struct Board 
{
    pub rows: [[i32;5]; 5],
    pub called: Vec<i32>,
}

impl Board
{
    pub fn new() -> Board
    {
        Board
        {
            rows:[[-1;5];5],
            called: Vec::new()
        }
    }

    pub fn insert_number(&mut self, row:usize, column:usize, value:i32)
    {
        self.rows[row][column] = value;
    }

    pub fn insert_row(&mut self, row:usize, value:[i32;5])
    {
        self.rows[row] = value;
    }

    pub fn sum_uncalled(&self) -> i32
    {
        let mut row = 0;
        let mut col = 0;
        let mut total = 0;

        println!("Called numbers for this board: {:?}", self.called);

        while row < 5
        {
            while col < 5
            {
                print!("{} ", self.rows[row][col]);
                if !self.called.contains(&self.rows[row][col])
                {
                    total += self.rows[row][col];
                }
                col += 1
            }
            print!("\n");
            row += 1;
        }

        return total;
    }

    pub fn call_number(&mut self, value:i32) -> bool
    {
        let value_pos = self.is_at(value);
        match value_pos
        {
            Ok((row_pos, col_pos)) =>
            {
                println!("Called value is at position row {}, column {}.", row_pos, col_pos);
                // If the board contains the number, push it onto our list of called numbers.
                self.called.push(value);
                // Check if this new number causes a row or column win.
                return self.row_or_col_win(row_pos, col_pos);
            },
            Err(err) => 
            {
                return false;
            }
            
        }
    }

    pub fn is_at(&mut self, value: i32) -> Result<(usize, usize), String>
    {
        let mut row_index = 0;
        let mut col_index = 0;

        while row_index < 5
        {
            while col_index < 5
            {
                if value == self.rows[row_index][col_index]
                {
                    return Ok((row_index, col_index));
                }
                col_index += 1;
            }
            row_index += 1;
        }

        return Err("Not found.".to_string());
    }

    pub fn row_or_col_win(&mut self, row: usize, column: usize) -> bool
    {
        // check row, then column
        let mut column_win = true;
        let mut row_win:bool;
        let mut col_index = 0;
        println!("Testing columns of row {}", row);
        println!("Contents of called set: {:?}", self.called);
        while col_index < 5
        {
            println!("What's at {}, {}? {}", row, col_index, &self.rows[row][col_index]);
            column_win &= self.called.contains(&self.rows[row][col_index]);
            println!("Column position {} in the called set? {}", col_index, column_win);
            col_index += 1;
        }

        row_win = true;
        let mut row_index = 0;
        println!("Now testing rows of column {}", column);
        while row_index < 5
        {
            row_win &= self.called.contains(&self.rows[row_index][column]);
            println!("Row position {} in the called set? {}", row_index, row_win);
            row_index += 1;
        }

        return row_win || column_win;
    }

    pub fn is_win(&mut self ) -> bool
    {
        // search rows
        let mut row = 0;
        let mut column = 0;
        let mut win = true;
        
        while row < 5
        {
            column = 0;
            while column < 5
            {
                win &= self.called.contains(&self.rows[row][column]);
                column += 1;
            }

            // if every element in a row is a win, this is a victory.
            if win == true
            {
                return win;
            }
            else {
                win = true;
                column = 0;
                row += 1;
            }
        }

        while column < 5
        {
            while row < 5
            {
                win &= self.called.contains(&self.rows[row][column]);
                row += 1
            }

            if win == true
            {
                return win;
            }
            else
            {
                column += 1;
            }
        }

        return win;
    }
}
