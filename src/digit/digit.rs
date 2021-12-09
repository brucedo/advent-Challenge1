use std::char;

pub struct Digit
{
    pub segments: [i8;7],
    pub possible_digits: Vec<i8>,
    pub num_set_segments: i8,
    pub position: usize
}


impl Digit
{
    pub fn new() -> Digit
    {
        Digit { segments: [0;7], possible_digits: Vec::<i8>::new(), num_set_segments: 0, position: std::usize::MAX }
    }

    pub fn set_segment(&mut self, segment: char)
    {
        match segment
        {
            'a' => {self.segments[0] = 1; self.num_set_segments += 1;}
            'b' => {self.segments[1] = 1; self.num_set_segments += 1;}
            'c' => {self.segments[2] = 1; self.num_set_segments += 1;}
            'd' => {self.segments[3] = 1; self.num_set_segments += 1;}
            'e' => {self.segments[4] = 1; self.num_set_segments += 1;}
            'f' => {self.segments[5] = 1; self.num_set_segments += 1;}
            'g' => {self.segments[6] = 1; self.num_set_segments += 1;}
            _ => {panic!("Character {} is not valid for 7-segment display.", segment)}
        }
    }
}

#[cfg(test)]
pub mod tests
{
    use super::Digit;


    pub fn init()
    {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    pub fn set_a_segment()
    {
        let mut test_digit = Digit::new();

        test_digit.set_segment('a');

        assert_eq!(test_digit.segments[0], 1);
        assert_eq!(test_digit.segments[1], 0);
        assert_eq!(test_digit.segments[2], 0);
        assert_eq!(test_digit.segments[3], 0);
        assert_eq!(test_digit.segments[4], 0);
        assert_eq!(test_digit.segments[5], 0);
        assert_eq!(test_digit.segments[6], 0);
        assert_eq!(test_digit.num_set_segments, 1);
    }

    #[test]
    pub fn set_b_segment()
    {
        let mut test_digit  = Digit::new();

        test_digit.set_segment('b');

        assert_eq!(test_digit.segments[0], 0);
        assert_eq!(test_digit.segments[1], 1);
        assert_eq!(test_digit.segments[2], 0);
        assert_eq!(test_digit.segments[3], 0);
        assert_eq!(test_digit.segments[4], 0);
        assert_eq!(test_digit.segments[5], 0);
        assert_eq!(test_digit.segments[6], 0);
        assert_eq!(test_digit.num_set_segments, 1);
    }

    #[test]
    pub fn set_c_segment()
    {
        let mut test_digit = Digit::new();

        test_digit.set_segment('c');

        assert_eq!(test_digit.segments[0], 0);
        assert_eq!(test_digit.segments[1], 0);
        assert_eq!(test_digit.segments[2], 1);
        assert_eq!(test_digit.segments[3], 0);
        assert_eq!(test_digit.segments[4], 0);
        assert_eq!(test_digit.segments[5], 0);
        assert_eq!(test_digit.segments[6], 0);
        assert_eq!(test_digit.num_set_segments, 1);
    }

    #[test]
    pub fn set_d_segment()
    {
        let mut test_digit = Digit::new();

        test_digit.set_segment('d');

        assert_eq!(test_digit.segments[0], 0);
        assert_eq!(test_digit.segments[1], 0);
        assert_eq!(test_digit.segments[2], 0);
        assert_eq!(test_digit.segments[3], 1);
        assert_eq!(test_digit.segments[4], 0);
        assert_eq!(test_digit.segments[5], 0);
        assert_eq!(test_digit.segments[6], 0);
        assert_eq!(test_digit.num_set_segments, 1);
    }

    #[test]
    pub fn set_e_segment()
    {
        let mut test_digit = Digit::new();

        test_digit.set_segment('e');

        assert_eq!(test_digit.segments[0], 0);
        assert_eq!(test_digit.segments[1], 0);
        assert_eq!(test_digit.segments[2], 0);
        assert_eq!(test_digit.segments[3], 0);
        assert_eq!(test_digit.segments[4], 1);
        assert_eq!(test_digit.segments[5], 0);
        assert_eq!(test_digit.segments[6], 0);
        assert_eq!(test_digit.num_set_segments, 1);
    }

    #[test]
    pub fn set_f_segment()
    {
        let mut test_digit = Digit::new();

        test_digit.set_segment('f');

        assert_eq!(test_digit.segments[0], 0);
        assert_eq!(test_digit.segments[1], 0);
        assert_eq!(test_digit.segments[2], 0);
        assert_eq!(test_digit.segments[3], 0);
        assert_eq!(test_digit.segments[4], 0);
        assert_eq!(test_digit.segments[5], 1);
        assert_eq!(test_digit.segments[6], 0);
        assert_eq!(test_digit.num_set_segments, 1);
    }

    #[test]
    pub fn set_g_segment()
    {
        let mut test_digit = Digit::new();

        test_digit.set_segment('g');

        assert_eq!(test_digit.segments[0], 0);
        assert_eq!(test_digit.segments[1], 0);
        assert_eq!(test_digit.segments[2], 0);
        assert_eq!(test_digit.segments[3], 0);
        assert_eq!(test_digit.segments[4], 0);
        assert_eq!(test_digit.segments[5], 0);
        assert_eq!(test_digit.segments[6], 1);
        assert_eq!(test_digit.num_set_segments, 1);
    }

    #[test]
    pub fn set_all_segments()
    {
        let mut test_digit = Digit::new();

        test_digit.set_segment('a');
        test_digit.set_segment('b');
        test_digit.set_segment('c');
        test_digit.set_segment('d');
        test_digit.set_segment('e');
        test_digit.set_segment('f');
        test_digit.set_segment('g');

        assert_eq!(test_digit.segments[0], 1);
        assert_eq!(test_digit.segments[1], 1);
        assert_eq!(test_digit.segments[2], 1);
        assert_eq!(test_digit.segments[3], 1);
        assert_eq!(test_digit.segments[4], 1);
        assert_eq!(test_digit.segments[5], 1);
        assert_eq!(test_digit.segments[6], 1);
        assert_eq!(test_digit.num_set_segments, 7);       
    }
}