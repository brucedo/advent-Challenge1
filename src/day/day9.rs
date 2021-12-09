use std::{io::BufReader, fs::File};

use crate::common::common::{read_trimmed_line, get_reader};

use log::{debug, error};

pub fn challenge_day_9()
{
    let mut reader = get_reader();
    part_one(&mut reader);
}

fn part_one(reader: &mut BufReader<File>)
{
    let mut buffer = String::new();
    let mut result = read_trimmed_line(reader, &mut buffer);

    let mut field: Vec<Vec<i8>> = Vec::new();

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
                let mut new_row = Vec::<i8>::new();
                for digit in buffer.chars()
                {
                    new_row.push(char_to_i8(digit));
                }
                field.push(new_row);

                buffer.clear();
                result = read_trimmed_line(reader, &mut buffer);
            }
            Err(e) =>
            {
                error!("An error occurred reading the file in: {}", e);
                return;
            }
        }
    }

    debug!("Field loaded: \n{:?}", field);
    // run over x,y looking for low points and calculate risk
    let risk = 0;
    let top_edge = 0;
    let left_edge = 0;
    let right_edge = field[0].len();
    let bottom_edge = field.len();
    for i in 0..field.len()
    {
        for j in 0..field[i].len()
        {
            if i - 1 >= top_edge
            {
                
            }
        }
    }
}

fn char_to_i8(char_digit: char) -> i8
{
    match char_digit
    {
        '0' => {0}
        '1' => {1}
        '2' => {2}
        '3' => {3}
        '4' => {4}
        '5' => {5}
        '6' => {6}
        '7' => {7}
        '8' => {8}
        '9' => {9}
        _ => {panic!("This character is not a number.")}
    }
}
