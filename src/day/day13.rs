use std::cmp::max;
use std::{io::BufReader, fs::File};

use crate::common::common::{get_reader, read_trimmed_line};
use log::debug;
use log::error;

pub fn challenge_day_13()
{
    let reader = get_reader();
}

fn part_one(reader: &mut BufReader<File>)
{
    let coords = read_in_coords(reader);

    let mut max_x: usize = 0;
    let mut max_y: usize = 0;

    for (x, y) in coords
    {
        max_x = max(x as usize, max_x);
        max_y = max(y as usize, max_y);
    }
}

fn read_in_coords(reader: &mut BufReader<File>) -> Vec::<(i16, i16)>
{
    let mut buffer = String::new();
    let mut read_result = read_trimmed_line(reader, &mut buffer);

    let mut coords = Vec::<(i16, i16)>::new();

    loop
    {
        match read_result
        {
            Ok(size) =>
            {
                if size == 0
                {
                    panic!("Unexpected end of file in coordinates section.");
                }
                if buffer.len() == 0
                {
                    // a double newline with no text denotes the end of this first section.
                    break;
                }

                let mut iter = buffer.split(",");
                
                let (x, y) = (iter.next().unwrap().parse::<i16>().unwrap(), iter.next().unwrap().parse::<i16>().unwrap());
                coords.push((x, y));

                buffer.clear();
                read_result = read_trimmed_line(reader, &mut buffer);
            }
            Err(e) =>
            {
                panic!("An error occurred during the read-in of the file. {}", e);
            }
        }
    }

    return coords;
}