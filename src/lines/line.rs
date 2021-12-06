pub struct Line
{
    pub x1: i16,
    pub y1: i16,
    pub x2: i16,
    pub y2: i16
}

impl Line 
{
    pub fn new() -> Line
    {
        Line {
            x1: -1,
            y1: -1,
            x2: -1,
            y2: -1
        }
    }

    pub fn set_point1(&mut self, x: i16, y: i16)
    {
        self.x1 = x;
        self.y1 = y;
    }

    pub fn set_point2(&mut self, x: i16, y: i16)
    {
        self.x2 = x;
        self.y2 = y;
    }

    pub fn is_horizontal(&self) -> bool
    {
        self.x1 == self.x2
    }

    pub fn is_vertical(&self) -> bool
    {
        self.y1 == self.y2
    }
}