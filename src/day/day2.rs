use std::{io::{SeekFrom, BufReader, Seek}, fs::File};

use crate::common::common::{get_reader, read_line_from_file};

pub fn challenge_day_two()
{
    let mut reader = get_reader();
    day_two_part_one(&mut reader);
    
    let rewind_result = reader.seek(SeekFrom::Start(0));
    
    if rewind_result.is_err()
    {
        println!("Attempted to rewind to re-read file, but something blew up.  Reloading file.");
        reader = get_reader();
    }

    day_two_part_two(&mut reader);
}

fn day_two_part_two(reader: &mut BufReader<File>)
{
    let mut aim: i64 = 0;
    let mut horizontal: i64 = 0;
    let mut depth: i64 = 0;

    let mut result = read_line_from_file(reader);
    let mut line: String;

    if result.is_err()
    {
        println!("Unable to read a line.");
        return;
    }

    line = result.unwrap();

    while line != "".to_string()
    {
        let mut parts = line.split(" ");
        let direction = parts.next().unwrap();
        let parse_result = parts.next().unwrap().parse::<i64>();

        if parse_result.is_err()
        {
            println!("Well that didn't work.  The distance value is not an integer.");
        }

        let distance = parse_result.unwrap();

        match direction {
            "forward" =>
            {
                horizontal += distance;
                depth += distance * aim;
            },
            "up" =>
            {
                aim -= distance;
            }
            "down" =>
            {
                aim += distance;
            }
            _ =>
            {
                println!("Somehow got a string value for direction that is not forward, up, or down: {}", direction);
            }
        }

        result = read_line_from_file(reader);
        if result.is_err()
        {
            println!("Unable to read a line.");
            return;
        }
    
        line = result.unwrap();
    }

    println!("Final horizontal position: {}", horizontal);
    println!("Final depth: {}", depth);
    println!("Final product: {}", horizontal * depth);
}

fn day_two_part_one(reader: &mut BufReader<File>)
{
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;

    let mut result = read_line_from_file(reader);
    let mut line: String;

    if result.is_err()
    {
        println!("Unable to read a line.");
        return;
    }

    line = result.unwrap();

    while line != "".to_string()
    {
        
        let mut parts = line.split(" ");
        let direction = parts.next().unwrap();
        let distance = parts.next().unwrap().parse::<i32>();

        if distance.is_err()
        {
            println!("Well that didn't work.  The distance value is not an integer.");
        }

        match direction {
            "forward" =>
            {
                horizontal += distance.unwrap();
            },
            "up" =>
            {
                depth -= distance.unwrap();
            }
            "down" =>
            {
                depth += distance.unwrap();
            }
            _ =>
            {
                println!("Somehow got a string value for direction that is not forward, up, or down: {}", direction);
            }
        }

        result = read_line_from_file(reader);
        if result.is_err()
        {
            println!("Unable to read a line.");
            return;
        }
    
        line = result.unwrap();
    }

    println!("Total horizontal distance travelled: {}", horizontal);
    println!("Total depth traversed: {} ", depth);
    println!("Multiple: {} ", horizontal * depth);
}

