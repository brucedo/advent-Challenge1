use std::{io::BufReader, fs::File, ops::Index,};

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

    // part 2 - analyze all digits
    let mut total:i64 = 0;
    for display in &mut digits
    {
        let complete = segment_analysis(display);
        let mut subtotal:i64 = 0;
        for i in 10..14
        {
            subtotal *= 10;
            subtotal += (complete[i].possible_digits[0]) as i64;
            println!("Total is now: {}", subtotal);
        }
        total += subtotal;
    }

    println!("Final total of all supposed end-of-line values: {}", total);
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

fn segment_analysis(display: &mut Vec<Digit>) -> Vec<Digit>
{
    // A tediously-by-hand-pre-calculated matrix of common elements between digits.
    // The number at the intersection of any (digit1, digit2) pair is number of segments
    // that the two digits have in common when lit up.  Example: the digit 4 has 3 segments
    // in common with the digit 0 (b,c,f) - and so the intersection of 4,0 is 3.
    // Note that 0,4 is also 3 - the matrix is symmetrical, as the digit 0 also has 3 segments
    // in common with the digit 4.
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

    // pain in the fucking ass borrow system and loops.  Just tear display apart and be done with it.
    let mut unknown = Vec::<Digit>::new();
    let mut known = Vec::<Digit>::new();

    // Dismantle display.  Put its elements into known and unknown.
    while !display.is_empty()
    {
        let temp = display.pop().unwrap();
        debug!("Consuming entry from position {}", temp.position);
        if temp.possible_digits.len() == 1
        {
            known.push(temp);
        }
        else
        {
            unknown.push(temp);
        }
    }

    drop(display);

    // Now, iterate across all of the unknowns.  Check each Digit at display[unknown]
    // against all of the known indices.  Compare the common segment counts against the table
    // above.
    let last_pass = unknown.len();
    let curr_pass = std::usize::MAX;
    while !unknown.is_empty()
    {
        // if last_pass == curr_pass
        // {
        //     panic!("Last pass and curr_pass have not changed.  Probable infinite loop.");
        // }
        let mut digit1 = unknown.pop().unwrap();

        debug!("Unknown has {} entries left.", unknown.len());
        debug!("Digit1 count of possibles: {}", digit1.possible_digits.len());
        debug!{"Digit1 set of possibles: {:?}", digit1.possible_digits};
        debug!("Digit1 set segments: {:?}", digit1.segments);

        for digit2 in &known
        {
            
            let common_count = common_segment(&digit1, digit2);
            debug!("Common segment count between digit1 and digit2: {}", common_count);
            debug!("digit2 count of possibles: {}", digit2.possible_digits.len());
            debug!("digit2 set of possibles: {:?}", digit2.possible_digits);
            debug!("digit2 set segments: {:?}", digit2.segments);
            // Digit2 should have exactly 1 possible digit, otherwise it should not be in known
            let row = digit2.possible_digits[0];
            debug!("Row in precalc: {}", row);
            let mut i = 0;

            while i < digit1.possible_digits.len()
            {
                debug!("comparing segment count to digit1.possible[i] to digit2.possible[0]");
                // if the common element count of row, col equals the common_count value - then this digit remains
                // a possibility.  If they do not equal then the possible digit, known digit pair do not have the 
                // same set of common segments that the unknown digit, known digit pair have - and therefore the 
                // possible digit cannot be the same.
                debug!("precalc common value: {}", precalc[row as usize][(digit1.possible_digits[i]) as usize]);
                if common_count != precalc[row as usize][(digit1.possible_digits[i]) as usize]
                {
                    digit1.possible_digits.remove(i);
                }
                else {
                    i += 1;
                }
            }
        }

        if digit1.possible_digits.len() == 1
        {
            debug!("digit1 is known.");
            debug!("digit: {}", digit1.possible_digits[0]);
            known.push(digit1);

        }
        else
        {
            debug!("Inserting back into unknown at position 0.");
            unknown.insert(0, digit1);
        }
    }

    // sort known back into order
    known.sort_by_key(|f| f.position);
    return known;
}

fn common_segment(digit1: &Digit, digit2: &Digit) -> i8
{
    let mut common = 0;
    for i in 0..7
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

    let mut count: usize = 0;

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
                digit.position = count;
                count += 1;
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
    use crate::{digit::digit::Digit, day::day8::{size_analysis, common_segment}};

    use super::build_digits;

    #[test]
    pub fn test_common_segments()
    {
        let mut digit1 = Digit::new();
        let mut digit2 = Digit::new();

        digit1.set_segment('a');
        digit1.set_segment('b');
        digit1.set_segment('d');

        digit2.set_segment('a');
        digit2.set_segment('d');
        digit2.set_segment('f');

        assert_eq!(common_segment(&digit1, &digit2), 2);
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