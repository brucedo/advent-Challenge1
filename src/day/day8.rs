use std::{io::BufReader, fs::File,};

use log::{debug, error};

use crate::{digit::digit::Digit, common::common::{get_reader, read_trimmed_line}};

pub fn challenge_day_8()
{
    let mut reader = get_reader();

    part_one(&mut reader);
}

fn part_one(reader: &mut BufReader<File>)
{
    let mut buffer = String::new();
    let mut result = read_trimmed_line(reader, &mut buffer);

    let mut digits: Vec<Vec<Digit>> = Vec::new();

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
                debug!("buffer: {} ", buffer);
                digits.push(build_digits(&buffer));
                debug!("Last set pushed should be 14 long: {}", digits[digits.len() - 1].len());
                buffer.clear();
            }
            Err(ref e) =>
            {
                error!("Encountered error reading in file: {}", e);
            }
        }
        result = read_trimmed_line(reader, &mut buffer);
    }

    for display in &mut digits
    {
        analyze_digits(display)
    }

    let mut total_easy = 0;
    for display in &digits
    {
        for digit in &display[10..14]
        {
            if digit.possible_digits.len() == 1
            {
                match digit.possible_digits[0]
                {
                    1 | 4 |7 | 8 => 
                    {
                        total_easy += 1;
                    }
                    _ =>{}
                }
            }
        }
    }

    println!("Total number of 1, 4, 7 and 8: {}", total_easy);
}



fn analyze_digits(display: &mut Vec<Digit>)
{
    for digit in display
    {
        let result = size_analysis(digit);
        match result
        {
            Ok(matches) => {
                digit.possible_digits = matches;
            }
            Err(e) =>
            {
                panic!("A problem has occurred - the digit did not match any known segment pattern.  Error: {}", e);
            }
        }
    }

    // Size analysis done.  This is enough to answer part one, or should be.
}

fn segment_analysis(display: &mut Vec<Digit>)
{
    let precalc: [[i8; 10]; 10] = [
        [6, 2, 4, 4, 3, 4, 5, 3, 6, 5],
        [2, 2, 1, 2, 2, 1, 1, 2, 2, 2],
        [4, 1, 5, 4, 2, 3, 4, 2, 5, 4],
        [4, 2, 4, 5, 3, 4, 4, 3, 5, 5],
        [3, 2, 2, 3, 4, 3, 3, 2, 4, 4],
        [4, 1, 3, 4, 3, 5, 5, 2, 5, 5],
        [5, 1, 4, 4, 3, 5, 6, 2, 6, 5],
        [3, 2, 2, 3, 2, 2, 2, 3, 3, 3],
        [6, 2, 5, 5, 4, 5, 6, 3, 7, 6],
        [5, 2, 4, 5, 4, 5, 5, 3, 6, 6]
    ];
    let mut unknown = Vec::<&mut Digit>::new();
    let mut known = Vec::<&mut Digit>::new();

    // move all of the known digits into the known list to start.
    for digit in display
    {
        if digit.possible_digits.len() == 1
        {
            known.push(digit);
        }
        else
        {
            unknown.push(digit);
        }
    }

    // Now - start doing comparisons against known values.
    let unknown_counter = 0;
    while unknown.len() > 0
    {
        let digit = unknown[i];
        // compare against all of the knowns;

    }
    
}

fn common_segment(digit1: &Digit, digit2: &Digit) -> i8
{
    let common = 0;
    for i in 0..8
    {
        if digit1.segments[i] == 1 && digit2.segments[i] == 1
        {
            common += 1;
        }
    }

    return common;
}

fn size_analysis(digit: &Digit) -> Result<Vec<i8>, String>
{
    let mut matches = Vec::<i8>::new();
    debug!("Starting size analysis on digit of set segment size: {}", digit.num_set_segments);
    match digit.num_set_segments
    {
        2 => {
            debug!("Matches segment count for just 1");
            matches.push(1);
        }
        3 => {
            debug!("Matches segment count for just 7");
            matches.push(7);
        }
        4 => {
            debug!("Matches segment count for just 4");
            matches.push(4);
        }
        5 => {
            debug!("Maches segment count for 2, 3, and 5");
            matches.push(2);
            matches.push(3);
            matches.push(5);
        }
        6 => {
            debug!("Matches segment count for 0, 6, and 9");
            matches.push(0);
            matches.push(6);
            matches.push(9);
        }
        7 => {
            debug!("Maches segment count for just 8.");
            matches.push(8);
        }
        _ => {
            return Err("The number of active segments does not match any digit.".to_string());
        }
    }

    debug!("Total number of digits matched: {}", matches.len());
    return Ok(matches);
}

fn build_digits(digits: &String) -> Vec<Digit>
{
    let mut built_digits = Vec::<Digit>::new();

    for half in digits.split("|")
    {
        debug!("Half: {}", half);

        for digit_string in half.split(" ")
        {
            debug!("Digit: {}", digit_string);
            if !digit_string.is_empty()
            {
                debug!("Digit string is not empty.");
                let mut digit = Digit::new();
                for segment_str in digit_string.chars()
                {
                    debug!("Setting digit segment for segment char: {}", segment_str);
                    digit.set_segment(segment_str);
                    debug!("Number of segments lit: {}", digit.num_set_segments);
                }
                built_digits.push(digit);
            }
        }
    }

    return built_digits;
}


#[cfg(test)]
pub mod tests
{
    use crate::{digit::digit::Digit, day::day8::size_analysis};

    use super::build_digits;

    pub fn test_common_segments()
    {
        
    }

    #[test]
    pub fn test_size_analysis()
    {
        let mut test_digit = Digit::new();

        test_digit.set_segment('a');
        test_digit.set_segment('b');
        test_digit.set_segment('c');

        let result = size_analysis(&test_digit).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], 7);
    }

    #[test]
    pub fn test_single_segment()
    {
        let segments = "a |".to_string();

        let digits = build_digits(&segments);

        assert_eq!(digits.len(), 1);
        assert_eq!(digits[0].num_set_segments, 1);
        assert_eq!(digits[0].segments[0], 1);
    }

    #[test]
    pub fn test_one_digit_multiple_segments()
    {
        let segments = "abdf |".to_string();

        let digits = build_digits(&segments);

        assert_eq!(digits.len(), 1);
        assert_eq!(digits[0].num_set_segments, 4);
        assert_eq!(digits[0].segments[0], 1);
        assert_eq!(digits[0].segments[1], 1);
        assert_eq!(digits[0].segments[3], 1);
        assert_eq!(digits[0].segments[5], 1);
    }

    #[test]
    pub fn test_two_digit_single_segments()
    {
        let segments = "a a |".to_string();

        let digits = build_digits(&segments);

        assert_eq!(digits.len(), 2);
        assert_eq!(digits[0].num_set_segments, 1);
        assert_eq!(digits[0].segments[0], 1);
        assert_eq!(digits[1].num_set_segments, 1);
        assert_eq!(digits[1].segments[0], 1);
    }

    #[test]
    pub fn test_three_digit_single_segments()
    {
        let segments = "a ab | b".to_string();

        let digits = build_digits(&segments);

        assert_eq!(digits.len(), 3);
        assert_eq!(digits[0].num_set_segments, 1);
        assert_eq!(digits[0].segments[0], 1);
        assert_eq!(digits[1].num_set_segments, 2);
        assert_eq!(digits[1].segments[0], 1);
        assert_eq!(digits[1].segments[1], 1);
        assert_eq!(digits[2].num_set_segments, 1);
        assert_eq!(digits[2].segments[1], 1);
    }
}