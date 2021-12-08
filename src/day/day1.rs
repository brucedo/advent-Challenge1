use std::{io::{SeekFrom, Seek, BufReader}, fs::File};

use crate::common::common::{get_reader, read_int_from_file};

pub fn challenge_day_one()
{

    let mut reader = get_reader();

    let increase_result = count_increases(&mut reader);

    if increase_result.is_err()
    {
        println!("{}", increase_result.unwrap_err());
    }
    else
    {
        let count = increase_result.unwrap();
        println!("Total number of increases: {}.", count);
    }

    let reset_success = reader.seek(SeekFrom::Start(0));

    if reset_success.is_err()
    {
        println!("Something went wrong rewinding the file to run part 2.  Reopening file...");
        reader = get_reader();
    }

    let rolling_increase_result = count_rolling_increases(&mut reader);
    if rolling_increase_result.is_err()
    {
        println!("{}", rolling_increase_result.unwrap_err());
    }
    else
    {
        let rolling_count = rolling_increase_result.unwrap();
        println!("Total number of rolling window increases: {}", rolling_count);
    }
}

pub fn count_rolling_increases(reader: &mut BufReader<File>) -> Result<i32, String>
{
    // initialize our window
    let mut sum_window: [i32; 3] = [0, 0, 0];
    let mut first:i32;
    let mut second = -1;
    let mut k = 0;

    let mut count = 0;

    // Prime the pump...
    let mut next_num = read_int_from_file(reader)?;
    sum_window[0] += next_num;

    next_num = read_int_from_file(reader)?;
    sum_window[0] += next_num;
    sum_window[1] += next_num;

    next_num = read_int_from_file(reader)?;


    while next_num > -1 {

        sum_window[0] += next_num;
        sum_window[1] += next_num;
        sum_window[2] += next_num;

        first = second;
        second = sum_window[k];
        sum_window[k] = 0;

        k = (k + 1) % 3;

        if first > -1 && second > -1
        {
            if first < second
            {
                count += 1;
            }
        }

        next_num = read_int_from_file(reader)?;
    }

    return Ok(count);
}

pub fn count_increases(reader: &mut BufReader<File>) -> Result<i32, String>
{
    let mut count: i32 = 0;

    let mut last_num = read_int_from_file(reader);

    if last_num.is_err()
    {
        return Err("Something went wrong during the read in.".to_string());
    }

    let mut first_num = last_num.unwrap();
    
    last_num = read_int_from_file(reader);
    if last_num.is_err()
    {
        return Err("Something went wrong during the read in.".to_string());
        
    }
    let mut second_num = last_num.unwrap();

    while second_num != -1 
    {
        if second_num > first_num
        {
            count += 1;
        }
        first_num = second_num;

        last_num = read_int_from_file(reader);
        if last_num.is_err()
        {
            return Err("Something went wrong during the read in.".to_string());
        
        }
        second_num = last_num.unwrap();
    }

    return Ok(count);
}