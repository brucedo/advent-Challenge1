use std::{io::BufReader, fs::File};

use log::debug;

use crate::common::common::{get_reader, read_trimmed_line, char_to_i8};

pub fn challenge_day_11()
{
    let mut reader = get_reader();

    part_one(&mut reader);
}

pub fn part_one(reader: &mut BufReader<File>)
{
    let grid_result = build_grid(reader);
    let mut total_flash = 0;
    let mut follow_up = Vec::<(usize, usize)>::new();
    let mut flashed = Vec::<(usize, usize)>::new();

    if grid_result.is_err()
    {
        println!("Error reading file - quitting.");
        return;
    }
    let mut grid = grid_result.unwrap();
    let octopus_count = grid.len() * grid[0].len();
    let mut all_flash: bool = false;
    let mut first_step:u64 = 0;
    let mut round = 0;

    loop
    {
        debug!("Round: {}", round);
        power_up(&mut grid, &mut follow_up, &mut flashed);
        debug!("There are {} octopodes to check on.", follow_up.len());
        while !follow_up.is_empty()
        {
            let (i, j) = follow_up.pop().unwrap();
            debug!("There are now {} remaining octopodes to check on.", follow_up.len());
            debug!("Grid at [{}][{}] has increased in power from {}.", i, j, grid[i][j]);
            grid[i][j] += 1;
            if grid[i][j] == 10 // so basically if this follow-up just flashed
            {
                debug!("Octopus is now at power 10 and will flash - adding more follow-ups to check.");
                flashed.push((i, j));
                set_follow_ups(i, j, &grid, &mut follow_up);
            }
        }
        // all follow ups are done.  Count how many flashed on this turn, add it to the total, and go on.
        round += 1;
        println!("Number of flashes this turn: {}", flashed.len());
        
        if flashed.len() == octopus_count && !all_flash
        {
            first_step = round;
            all_flash = true;
        }
        total_flash += flashed.len();
        // reset all the flashed entries and clear the set.
        reset_flashed(&mut grid, &mut flashed);
        flashed.clear();

        if all_flash && round >= 100
        {
            break;
        }
    }

    println!("Total set of flashes over 100 turns: {}", total_flash);
    println!("The first step on which all octopodes flash is: {}", first_step);
}

fn reset_flashed(grid: &mut Vec<Vec<i8>>, flashed: &mut Vec<(usize, usize)>)
{
    for (i, j) in flashed
    {
        grid[*i][*j] = 0;
    }
}

fn power_up(grid: &mut Vec<Vec<i8>>, follow_up: &mut Vec<(usize, usize)>, flashed: &mut Vec<(usize, usize)>)
{
    for i in 0 .. grid.len()
    {
        for j in 0 .. grid[i].len()
        {
            debug!("increasing power of [{}][{}]", i, j);
            grid[i][j] += 1;
            if grid[i][j] > 9
            {
                flashed.push((i, j));
                set_follow_ups(i, j, grid, follow_up);
            }
        }
    }
}

fn set_follow_ups(i: usize, j: usize, grid: &Vec<Vec<i8>>, follow_up: &mut Vec<(usize, usize)>)
{
    if in_grid(i.checked_sub(1), j.checked_sub(1), grid)
    {
        follow_up.push((i-1, j-1));
    }
    if in_grid(i.checked_sub(1), Option::Some(j), grid)
    {
        follow_up.push((i-1, j));
    }
    if in_grid(i.checked_sub(1), j.checked_add(1), grid)
    {
        follow_up.push((i-1, j+1));
    }
    if in_grid(Option::Some(i), j.checked_sub(1), grid)
    {
        follow_up.push((i, j-1));
    }
    if in_grid(Option::Some(i), j.checked_add(1), grid)
    {
        follow_up.push((i, j+1));
    }
    if in_grid(i.checked_add(1), j.checked_sub(1), grid)
    {
        follow_up.push((i+1, j-1));
    }
    if in_grid(i.checked_add(1), Option::Some(j), grid)
    {
        follow_up.push((i+1, j));
    }
    if in_grid(i.checked_add(1), j.checked_add(1), grid)
    {
        follow_up.push((i+1,j+1));
    }
}

pub fn in_grid(i: Option<usize>, j: Option<usize>, grid: &Vec<Vec<i8>>) -> bool
{
    if i.is_none() || j.is_none()
    {
        return false;
    }

    let i = i.unwrap();
    let j = j.unwrap();
    if i < grid.len() && j < grid[i].len()
    {
        return true;
    }

    return false;
}

pub fn build_grid(reader: &mut BufReader<File>) -> Result<Vec<Vec<i8>>, String>
{
    let mut grid = Vec::<Vec<i8>>::new();
    let mut row; 
    let mut buffer = String::new();

    let mut result = read_trimmed_line(reader, &mut buffer);

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

                row = Vec::<i8>::new();
                for digit in buffer.chars()
                {
                    row.push(char_to_i8(digit));
                }
                grid.push(row);
                

                buffer.clear();
                result = read_trimmed_line(reader, &mut buffer);
            }
            Err(e) =>
            {
                println!("An error occurred attempting to read a line into the buffer: {}", e);
                return Err(e);
            }
        }
    }

    return Ok(grid);
}