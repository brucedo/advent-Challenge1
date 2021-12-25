use std::{io::BufReader, fs::File, str::Chars};

use log::debug;

use crate::common::common::{get_reader, read_trimmed_line, char_to_i8};

pub fn challenge_day_18()
{

    let mut reader = get_reader();

    part_one(&mut reader);

}

fn part_one(reader: &mut BufReader<File>)
{
    let buffer = &mut String::new();
    let mut result = read_trimmed_line(reader, buffer);

    let mut snailfish_numbers = Vec::<SnailfishNumber>::new();

    loop
    {
        match result
        {
            Ok(size) =>
            {
                if size == 0
                {
                    break;
                }

                let mut sfn_iter = buffer.chars();
                match build_snailfish_number(&mut sfn_iter)
                {
                    Ok(snf) =>
                    {
                        snailfish_numbers.push(snf);
                    }
                    Err(e) =>
                    {
                        panic!("Some string was not well formatted.  Poor snailfish shan't get its homework done. {}", e);
                    }
                }
                buffer.clear();
                result = read_trimmed_line(reader, buffer);
            }
            Err(e) =>
            {
                panic!("There was an error reading the file.  oops.  {}", e);
            }
        }
    }
}

fn build_snailfish_number(buffer: &mut Chars) -> Result<SnailfishNumber, String>
{
    let mut left: Option<SnailfishNumber> = None;
    let mut right: Option<SnailfishNumber> = None;

    // left side first
    match buffer.next()
    {
        Some(char) =>
        {
            debug!("Matching char {}", char);
            match char
            {
                '[' => {left = Some(build_snailfish_number(buffer)?)},
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' =>
                {
                    let mut temp = SnailfishNumber::new();
                    temp.value = Some(char_to_i8(char) as u64);
                    return Ok(temp);
                }
                _ => {
                    debug!("Malformed char sequence - {} received, [ or some number expected.", char);
                    return Err(format!("Malformed character sequence - {} when either [ or some number was expected.", char).to_string());}

            }
        },
        None =>
        {
            return Err("Reached end of char stream before end of SNF.".to_string());
        }
    }

    // Next character should be a ','
    match buffer.next()
    {
        Some(char) =>
        {
            match char
            {
                ',' => {right = Some(build_snailfish_number(buffer)?)}
                _ => {return Err(format!("Malformed character sequence - {} when ',' was expected.", char).to_string());}
            }
        }
        None =>
        {
            return Err("Reached end of char stream before end of SNF.".to_string());
        }
    }

    // final removal - the ']'.
    match buffer.next()
    {
        Some (char) =>
        {
            match char
            {
                ']' => {debug!("Closed out set of SNFs.");}
                _ => {return Err(format!("Malformed character sequence - {} when ']' was expected.", char).to_string());}
            }
        }
        None =>
        {
            return Err("Reached end of char stream before end of SNF.".to_string());
        }
    }

    let mut result = SnailfishNumber::new();
    result.set_left(left);
    result.set_right(right);

    return Ok(result);
}

struct SnailfishNumber
{
    pub value: Option<u64>,
    pub left: Option<Box<SnailfishNumber>>,
    pub right: Option<Box<SnailfishNumber>>
}

impl SnailfishNumber {

    pub fn new() -> SnailfishNumber
    {
        SnailfishNumber {
            value: None,
            left: None,
            right: None
        }
    }

    pub fn set_left(&mut self, left: Option<SnailfishNumber>)
    {
        match left
        {
            None => {self.left = None}
            Some(new_left) => {self.left = Some(Box::<SnailfishNumber>::new(new_left));}
        }
    }

    pub fn set_right(&mut self, right: Option<SnailfishNumber>)
    {
        match right
        {
            None => {self.right = None}
            Some(new_right) => {self.right = Some(Box::<SnailfishNumber>::new(new_right));}
        }
    }

    pub fn reduce(&mut self)
    {
        let mut action_taken = true;

        while action_taken
        {
            action_taken = SnailfishNumber::explode(self, 0).is_some();
        }
    }

    pub fn explode(node: &mut  SnailfishNumber, depth: usize) -> Option<(u64, u64)>
    {

        // base case
        if node.left.is_none() && node.right.is_none()
        {
            if depth >= 4
            {
                let val = (node.left.as_ref().unwrap().value.unwrap(), node.right.as_ref().unwrap().value.unwrap());
                node.left = None;
                node.right = None;
                node.value = Some(0);
                return Some(val);
            }
            // if we hit a leaf node but are not nested 4 deep - good.  Just return.
            else
            {
                return None
            }
        }

        if node.left.is_some()
        {
            let left = node.left.as_mut().unwrap();
            match SnailfishNumber::explode(left.as_mut(), depth + 1)
            {
                Some(pair) =>
                {
                    
                    return Some(pair);
                }
                None =>
                {
                    // Nothing to do - explosion hit no base case at a depth of 4 nested.
                }
            }
        }

        if node.right.is_some()
        {
            let right = node.right.as_mut().unwrap().as_mut();
            match SnailfishNumber::explode(right, depth + 1)
            {
                Some(pair) =>
                {

                }
                None =>
                {
                    
                }
            }
        }

        return None;
    }
}

#[cfg(test)]
pub mod tests
{
    use super::{SnailfishNumber, build_snailfish_number};

    fn init()
    {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    pub fn test_build_snf()
    {
        init();

        let mut a = "[3,2]".chars();
        let mut snf: SnailfishNumber;
        let mut left: Box<SnailfishNumber>;
        let mut right: Box<SnailfishNumber>;

        let mut result = build_snailfish_number(&mut a);
        assert!(result.is_ok());
        snf = result.unwrap();
        assert!(snf.left.is_some());
        assert!(snf.right.is_some());
        left = snf.left.unwrap();
        assert!(left.value.is_some());
        assert_eq!(3,left.value.unwrap());
        right = snf.right.unwrap();
        assert!(right.value.is_some());
        assert_eq!(2, right.value.unwrap());

        a = "[3,[3,2]]".chars();
        result = build_snailfish_number(&mut a);
        assert!(result.is_ok());
        snf = result.unwrap();
        assert!(snf.left.is_some());
        assert!(snf.right.is_some());
        left = snf.left.unwrap();
        assert!(left.value.is_some());
        assert_eq!(3, left.value.unwrap());
        right = snf.right.unwrap();
        assert!(right.value.is_none());
        assert!(right.left.is_some());
        assert!(right.right.is_some());

        left = right.left.unwrap();
        right = right.right.unwrap();
        assert!(left.value.is_some());
        assert!(right.value.is_some());
        assert_eq!(3, left.value.unwrap());
        assert_eq!(2, right.value.unwrap());

        a = "[[2,3],3]".chars();
        result = build_snailfish_number(&mut a);
        assert!(result.is_ok());
        snf = result.unwrap();
        assert!(snf.left.is_some());
        assert!(snf.right.is_some());

        left = snf.left.unwrap();
        right = snf.right.unwrap();

        assert!(left.value.is_none());
        assert!(right.value.is_some());
        assert_eq!(3, right.value.unwrap());
        assert!(left.left.is_some());
        assert!(left.right.is_some());
        
        right = left.right.unwrap();
        left = left.left.unwrap();

        assert!(left.value.is_some());
        assert!(right.value.is_some());
        assert_eq!(2, left.value.unwrap());
        assert_eq!(3, right.value.unwrap());
    }

}