use std::{io::BufReader, fs::File};

use crate::common::common::{read_trimmed_line, get_reader, char_to_i8};

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
    let mut risk: i32 = 0;
    let mut low_points = Vec::<(usize,usize)>::new();
    let top_edge = 0;
    let left_edge = 0;
    let right_edge = field[0].len();
    let bottom_edge = field.len();
    for i in 0..field.len()
    {
        for j in 0..field[i].len()
        {
            // if the i + 1, i - 1, j + 1, j - 1 indices are not outside the bounds of the field
            // AND the value at any one of above, below, to the right 
            debug!("i: {}, j: {}", i, j);
            if i >= top_edge + 1 && field[i-1][j] <= field[i][j]
            {
                continue;
            }
            if j >= left_edge + 1 && field[i][j-1] <= field[i][j]
            {
                continue;
            }
            if i + 1 < bottom_edge && field[i+1][j] <= field[i][j]
            {
                
                continue;
            }
            if j + 1 < right_edge && field[i][j+1] <= field[i][j]
            {
                continue;
            }
            
            low_points.push((i,j));
        }
    }

    println!("Low points of this field are: {:?}", low_points);
    for (i,j) in &low_points
    {
        risk += 1 + (field[*i][*j] as i32);
    }
    println!("Risk total is: {}", risk);

    part_two(field, low_points);
}

fn part_two(field: Vec<Vec<i8>>, low_points: Vec<(usize, usize)>)
{
    let mut first = 0;
    let mut second = 0;
    let mut third = 0;
    let mut current; // size of the basin we are currently inspecting
    let mut queue = Vec::<(usize, usize)>::new();
    let mut searched = Vec::<(usize, usize)>::new();

    // bounds
    let top = 0;
    let left = 0;
    let right = field[0].len();
    let bottom = field.len();

    for (i, j) in low_points
    {
        // start with the first low point
        debug!("Starting search at [{}][{}]", i, j);
        queue.push((i, j));
        searched.clear();
        current = 0;
        while !queue.is_empty()
        {
            debug!("queue size is: {}", queue.len());
            let (next_i, next_j) = queue.pop().unwrap();
            searched.push((next_i, next_j));

            if field[next_i][next_j] == 9
            {
                debug!("hit basin boundary at [{}][{}]", next_i, next_j);
                continue;
            }
            else
            {
                debug!("Not a boundary value at [{}][{}]: {}", next_i, next_j, field[next_i][next_j]);
                current += 1;
                debug!("Current basin size: {}", current);
                if next_i >= top + 1 && !searched.contains(&(next_i-1, next_j)) && !queue.contains(&(next_i-1, next_j))
                {
                    debug!("Adding new entry to queue: [{}][{}]", next_i-1, next_j);
                    debug!("Searched contents: {:?}", searched);
                    queue.push((next_i-1, next_j));
                }
                if next_i + 1 < bottom && !searched.contains(&(next_i+1, next_j)) && !queue.contains(&(next_i+1, next_j))
                {
                    debug!("Adding new entry to queue: [{}][{}]", next_i+1, next_j);
                    debug!("Searched contents: {:?}", searched);
                    queue.push((next_i+1, next_j));
                }
                if next_j >= left + 1 && !searched.contains(&(next_i, next_j-1)) && !queue.contains(&(next_i, next_j-1))
                {
                    debug!("Adding new entry to queue: [{}][{}]", next_i, next_j-1);
                    debug!("Searched contents: {:?}", searched);
                    queue.push((next_i, next_j-1));
                }
                if next_j + 1 < right && !searched.contains(&(next_i, next_j+1)) && !queue.contains(&(next_i, next_j+1))
                {
                    debug!("Adding new entry to queue: [{}][{}]", next_i, next_j+1);
                    debug!("Searched contents: {:?}", searched);
                    queue.push((next_i, next_j+1))
                }
            }
        }

        // insert the new current into the set of three largest basins in the appropriate place.
        if current > first
        {
            third = second;
            second = first;
            first = current;
        }
        else if current > second
        {
            third = second;
            second = current;
        }
        else if current > third
        {
            third = current;
        }
    }

    println!("The first, second and third largest basins are {}, {}, and {}", first, second, third);
    println!("Size multiplied is {}", first * second * third);
}

