use std::{io::BufReader, fs::File};

use crate::common::common::{get_reader, read_trimmed_line};

pub fn challenge_day_11()
{
    let mut reader = get_reader();

}

pub fn part_one(reader: &mut BufReader<File>)
{
    let mut grid_result = build_grid(reader);
    let mut total_flash = 0;
    let mut follow_up = Vec::<(usize, usize)>::new();
    let mut flashed = Vec::<(usize, usize)>::new();

    if grid_result.is_err()
    {
        println!("Error reading file - quitting.");
        return;
    }
    let mut grid = grid_result.unwrap();

    for i in 0 .. 100
    {
        power_up(&mut grid, &mut follow_up, &mut flashed)
    }

}

fn power_up(grid: &mut Vec<Vec<i8>>, follow_up: &mut Vec<(usize, usize)>, flashed: &mut Vec<(usize, usize)>)
{
    for i in 0 .. grid.len()
    {
        for j in 0 .. grid[i].len()
        {
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
    let grid = Vec::<Vec<i8>>::new();
    let mut buffer = String::new();

    let result = read_trimmed_line(reader, &mut buffer);

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