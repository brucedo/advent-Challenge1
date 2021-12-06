use std::num;

use log::{debug, trace};

use super::line::Line;
pub struct Grid
{
    grid: Vec<Vec<i16>>,
}

impl Grid
{
    pub fn new() -> Grid
    {
        Grid
        {
            grid: Vec::new()
        }
    }

    pub fn init(&mut self, x: usize, y: usize)
    {
        for x_index in 0..x
        {
            debug!("Creating vec {} out of {}", x_index, x);
            self.grid.push(Vec::new());
            for _y_index in 0..y
            {
                trace!("Creating nested vec {} out of {}", _y_index, y);
                self.grid[x_index].push(0);
            }
        }
    }

    pub fn get_width(&self) -> usize
    {
        self.grid.len()
    }

    pub fn get_height_at(&self, x: usize) -> usize
    {
        self.grid[x].len()
    }

    pub fn get_value_at(&self, x: usize, y:usize) -> i16
    {
        self.grid[x][y]
    }

    pub fn insert_diagonal_line(&mut self, line: &Line)
    {
        // Again, we need to know which way our x and y steps are going.  Even on a diagonla, the rules for x_step and
        // y_step for flat lines should work here.
        let x_step:i16 = if line.x1 > line.x2 {-1} else {1};
        let y_step:i16 = if line.y1 > line.y2 {-1} else {1};

        debug!("Steps set - x: {}, y: {}", x_step, y_step);

        debug!("Starting at {}, {} and traversing to {}, {}", line.x1, line.y1, line.x2, line.y2);
        let mut x_index = line.x1;
        let mut y_index = line.y1;

        // This time, we are going to be incrementing and decrementing x and y with _every_ update - not just one.
        // a more general purpose algo would see us updating our x/y index in accordance with a slop calculation, but hey.
        
        // We can be assured from the problem description that every diagonal line will have exactly as many x updates
        // as y.  So we just need to iterate over the two coordinates one time.
        loop
        {
            self.grid[x_index as usize][y_index as usize] += 1;

            x_index += x_step;
            y_index += y_step;

            if (x_step == -1 && x_index < line.x2) || (x_step == 1 && x_index > line.x2)
            {
                break;
            }
        }
    }

    // horizontal or vertical only.
    pub fn insert_flat_line(&mut self, line: &Line)
    {
        // increment all of the points in the graph that lie along the line.
        // First thing - determine the directionality of the x and y coords.  We don't know for sure that x1 < x2 or y1 < y2.
        debug!("Inserting horizontal line {}, {} -> {}, {}", line.x1, line.y1, line.x2, line.y2);
        let x_step:i16 = if line.x1 > line.x2 {-1} else {1};
        let y_step:i16 = if line.y1 > line.y2 {-1} else {1};

        debug!("Steps set - x: {}, y: {}", x_step, y_step);
        
        // Next, I want to skip across each x,y coordinate and increment the number.  The trick to remember is that range operators
        // are only left-inclusive; the right-side number will not be counted.  So we actually have to go from x1 to x2 + 1.
        // This is something to keep in mind when calculating the grid size: mental note that.
        
        debug!("Starting at {}, {} and traversing to {}, {}", line.x1, line.y1, line.x2, line.y2);
        let mut x_index = line.x1;
        let mut y_index = line.y1;
        loop
        {

            loop
            {

                self.grid[x_index as usize][y_index as usize] += 1;

                y_index += y_step;
                if (y_step == -1 && y_index < line.y2) || (y_step == 1 && y_index > line.y2)
                {
                    trace!("finished traversing y");
                    break;
                }
                trace!("Next {}, {}", x_index, y_index);
            }

            x_index += x_step;
            y_index = line.y1;
            if (x_step == -1 && x_index < line.x2) || (x_step == 1 && x_index > line.x2)
            {
                break;
            }
            trace!("Next {}, {}", x_index, y_index);
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::Grid;
    use super::Line;

    fn init()
    {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_init()
    {
        init();

        let mut test_grid = Grid::new();

        test_grid.init(5, 5);
        
        assert_eq!(test_grid.get_width(), 5);

        for i in 0..5
        {
            assert_eq!(test_grid.get_height_at(i), 5);
        }
    }

    #[test]
    fn test_init_uneven()
    {
        init();
        let mut test_grid = Grid::new();

        test_grid.init(1010, 22);

        assert_eq!(test_grid.get_width(), 1010);

        for i in 0..1010
        {
            assert_eq!(test_grid.get_height_at(i), 22);
        }
    }

    #[test]
    fn test_insert_line()
    {
        init();
        let mut test_grid = Grid::new();
        let mut line = Line::new();

        test_grid.init(20, 20);

        line.x1 = 0;
        line.y1 = 5;
        line.x2 = 0;
        line.y2 = 10;

        test_grid.insert_flat_line(&line);

        for i in 5..11
        {
            assert_eq!(test_grid.get_value_at(0, i), 1);
        }
        
    }

    #[test]
    fn test_insert_line_reverse_x()
    {
        init();
        let mut test_grid = Grid::new();
        let mut line = Line::new();

        test_grid.init(20, 20);
        
        line.x1 = 10;
        line.y1 = 0;
        line.x2 = 5;
        line.y2 = 0;

        test_grid.insert_flat_line(&line);

    
        assert_eq!(test_grid.get_value_at(10, 0), 1);
        assert_eq!(test_grid.get_value_at(9, 0), 1);
        assert_eq!(test_grid.get_value_at(8, 0), 1);
        assert_eq!(test_grid.get_value_at(7, 0), 1);
        assert_eq!(test_grid.get_value_at(6, 0), 1);
        assert_eq!(test_grid.get_value_at(5, 0), 1);
    }

    #[test]
    fn test_insert_line_reverse_y()
    {
        init();
        let mut test_grid = Grid::new();
        let mut line = Line::new();

        test_grid.init(20, 20);
        
        line.x1 = 0;
        line.y1 = 10;
        line.x2 = 0;
        line.y2 = 5;

        test_grid.insert_flat_line(&line);

    
        assert_eq!(test_grid.get_value_at(0, 10), 1);
        assert_eq!(test_grid.get_value_at(0, 9), 1);
        assert_eq!(test_grid.get_value_at(0, 8), 1);
        assert_eq!(test_grid.get_value_at(0, 7), 1);
        assert_eq!(test_grid.get_value_at(0, 6), 1);
        assert_eq!(test_grid.get_value_at(0, 5), 1);
    }

    #[test]
    fn test_insert_line_forward_x()
    {
        init();
        let mut test_grid = Grid::new();
        let mut line = Line::new();

        test_grid.init(20, 20);
        
        line.x1 = 5;
        line.y1 = 0;
        line.x2 = 10;
        line.y2 = 0;

        test_grid.insert_flat_line(&line);

    
        assert_eq!(test_grid.get_value_at(10, 0), 1);
        assert_eq!(test_grid.get_value_at(9, 0), 1);
        assert_eq!(test_grid.get_value_at(8, 0), 1);
        assert_eq!(test_grid.get_value_at(7, 0), 1);
        assert_eq!(test_grid.get_value_at(6, 0), 1);
        assert_eq!(test_grid.get_value_at(5, 0), 1);
    }

    #[test]
    fn test_insert_overlap()
    {
        init();
        let mut test_grid = Grid:: new();
        let mut line = Line::new();
        let mut line2 = Line::new();

        test_grid.init(20, 20);

        line.x1 = 5;
        line.y1 = 0;
        line.x2 = 10;
        line.y2 = 0;

        line2.x1 = 5;
        line2.y1 = 0;
        line2.x2 = 5;
        line2.y2 = 10;

        test_grid.insert_flat_line(&line);
        test_grid.insert_flat_line(&line2);

        assert_eq!(test_grid.get_value_at(5, 0), 2);

    }

    #[test]
    fn test_insert_diag_down_right()
    {
        init();
        let mut test_grid = Grid:: new();
        let mut line = Line::new();

        test_grid.init(20, 20);

        line.x1 = 0;
        line.y1 = 0;
        line.x2 = 5;
        line.y2 = 5;

        test_grid.insert_flat_line(&line);

        assert_eq!(test_grid.get_value_at(0, 0), 1);
        assert_eq!(test_grid.get_value_at(1, 1), 1);
        assert_eq!(test_grid.get_value_at(2, 2), 1);
        assert_eq!(test_grid.get_value_at(3, 3), 1);
        assert_eq!(test_grid.get_value_at(4, 4), 1);
        assert_eq!(test_grid.get_value_at(5, 5), 1);
    }

    #[test]
    fn test_insert_diag_down_left()
    {

        init();
        let mut test_grid = Grid:: new();
        let mut line = Line::new();

        test_grid.init(20, 20);

        line.x1 = 5;
        line.y1 = 0;
        line.x2 = 0;
        line.y2 = 5;

        test_grid.insert_flat_line(&line);

        assert_eq!(test_grid.get_value_at(5, 0), 1);
        assert_eq!(test_grid.get_value_at(4, 1), 1);
        assert_eq!(test_grid.get_value_at(3, 2), 1);
        assert_eq!(test_grid.get_value_at(2, 3), 1);
        assert_eq!(test_grid.get_value_at(1, 4), 1);
        assert_eq!(test_grid.get_value_at(0, 5), 1);
    }

    #[test]
    fn test_insert_diag_up_right()
    {
        init();
        let mut test_grid = Grid:: new();
        let mut line = Line::new();

        test_grid.init(20, 20);

        line.x1 = 0;
        line.y1 = 5;
        line.x2 = 5;
        line.y2 = 0;

        test_grid.insert_flat_line(&line);

        assert_eq!(test_grid.get_value_at(0, 5), 1);
        assert_eq!(test_grid.get_value_at(1, 4), 1);
        assert_eq!(test_grid.get_value_at(2, 3), 1);
        assert_eq!(test_grid.get_value_at(3, 2), 1);
        assert_eq!(test_grid.get_value_at(4, 1), 1);
        assert_eq!(test_grid.get_value_at(5, 0), 1);
    }

    #[test]
    fn test_insert_diag_up_left()
    {
        init();
        let mut test_grid = Grid:: new();
        let mut line = Line::new();

        test_grid.init(20, 20);

        line.x1 = 5;
        line.y1 = 5;
        line.x2 = 0;
        line.y2 = 0;

        test_grid.insert_flat_line(&line);

        assert_eq!(test_grid.get_value_at(5, 5), 1);
        assert_eq!(test_grid.get_value_at(4, 4), 1);
        assert_eq!(test_grid.get_value_at(3, 3), 1);
        assert_eq!(test_grid.get_value_at(2, 2), 1);
        assert_eq!(test_grid.get_value_at(1, 1), 1);
        assert_eq!(test_grid.get_value_at(0, 0), 1);
    }

    #[test]
    fn test_insert_diag_intersection()
    {
        init();
        let mut test_grid = Grid::new();
        let mut line1 = Line::new();
        let mut line2 = Line::new();
        let mut line3 = Line::new();

        test_grid.init(20, 20);

        line1.x1 = 0;
        line1.y1 = 0;
        line1.x2 = 5;
        line1.y2 = 0;

        line2.x1 = 0;
        line2.y1 = 0;
        line2.x2 = 5;
        line2.y2 = 5;

        line3.x1 = 5;
        line3.y1 = 5;
        line3.x2 = 10;
        line3.y2 = 0;

        test_grid.insert_flat_line(&line1);
        test_grid.insert_diagonal_line(&line2);
        test_grid.insert_diagonal_line(&line3);

        assert_eq!(test_grid.get_value_at(0, 0), 2);
        assert_eq!(test_grid.get_value_at(5, 5), 2);
    }

}