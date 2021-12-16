use std::{io::{BufReader, Seek, SeekFrom}, fs::File, cmp::max, cmp::min, i32::MAX};

use crate::common::common::{read_trimmed_line, get_reader};
use log::{error, debug};

pub fn challenge_day_7()
{
    let mut reader = get_reader();

    let part1_calculator = |&distance:&i32| {
        return distance
    };

    let part2_calculator = |&distance:&i32| {
        (distance * (distance + 1)) / 2
    };

    part_one(&mut reader, part1_calculator);

    let result = reader.seek(SeekFrom::Start(0));

    if result.is_err()
    {
        println!("An error occurred while rewinding the file.  Try reopening it.");
        reader = get_reader();
    }

    part_one(&mut reader, part2_calculator);
}

fn part_one<F>(reader: &mut BufReader<File>, calculator: F) where F: Fn(&i32)->i32
{
    // Get the line of numbers out of the file.
    let mut buffer = String::new();
    let result = read_trimmed_line(reader, &mut buffer);

    let mut crab_set = Vec::<i32>::new();
    let mut max_size = 0;
    let mut total = 0;

    match result
    {
        Ok(_) =>
        {
            for number in buffer.split(",")
            {
                let temp = number.parse::<i32>().unwrap();
                debug!("Loading number {} into the set.", temp);
                crab_set.push(temp);
                max_size = max(max_size, temp);
                debug!("Max size is now: {}", max_size);
            }
        },
        Err(e) =>
        {
            error!("Something went wrong reading a line in from file: {}", e);
        }
    }

    let mut min_fuel_usage = MAX;

    debug!("About to loop, what's max here? {}", max_size);

    // step across the set of possible numbers
    for i in 0..(max_size + 1)
    {
        for crab_horizontal in &crab_set
        {
            total += calculator(&(i - crab_horizontal).abs());
        }
        min_fuel_usage = min(min_fuel_usage, total);
        println!("Fuel utilization for horizontal line {} is {}", i, total);
        total = 0;
    }

    println!("Minimum fuel usage should be {}", min_fuel_usage);
}