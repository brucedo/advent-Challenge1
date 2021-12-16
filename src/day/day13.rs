use std::cmp::max;
use std::{io::BufReader, fs::File};

use crate::common::common::{get_reader, read_trimmed_line};
use crate::lines::grid::Grid;
use log::debug;

pub fn challenge_day_13()
{
    let mut reader = get_reader();

    part_one(&mut reader);
}

fn part_one(reader: &mut BufReader<File>)
{
    let coords = read_in_coords(reader);
    let fold_list = read_fold_list(reader);

    println!("fold list: {:?}", fold_list);

    let mut max_x: usize = 0;
    let mut max_y: usize = 0;

    for (x, y) in &coords
    {
        max_x = max(*x as usize, max_x);
        max_y = max(*y as usize, max_y);
    }

    let mut grid = Grid::new();
    grid.init(max_y + 1, max_x + 1);

    load_grid(coords, &mut grid);

    // Do one fold for part one.
    let (x, y) = fold_list[0];
    if y > 0
    {
        grid.fold_up(y);
    }
    else if x > 0
    {
        grid.fold_left(x);
    }
    else
    {
        println!("Well crapcakes, this just went to hell in a real hurry.");
    }

    let mut count = 0;
    for row in 0..grid.get_width()
    {
        for col in 0..grid[row].len()
        {
            if grid[row][col] > 0
            {
                count += 1;
            }
        }
    }

    println!("Total count of marks on the magical transparency: {}", count);

    // DO the rest of the folds
    for index in 1..fold_list.len()
    {
        let (x, y) = fold_list[index];
        if y > 0
        {
            grid.fold_up(y);
        }
        else if x > 0
        {
            grid.fold_left(x);
        }
        else
        {
            println!("Well crapcakes, this just went to hell in a real hurry.");
        }
    }

    for row in 0..grid.get_width()
    {
        for col in 0..grid[row].len()
        {
            if grid[row][col] == 0
            {
                print!(" ");
            }
            else
            {
                print!("*");
            }
        }
        print!("\n");
    }
}

fn read_fold_list(reader: &mut BufReader<File>) -> Vec<(usize, usize)>
{
    let mut buffer = String::new();
    let mut read_result = read_trimmed_line(reader, &mut buffer);

    let mut fold_list = Vec::<(usize, usize)>::new();

    loop
    {
        match read_result
        {
            Ok(size) =>
            {
                if size == 0
                {
                    break;
                }
                debug!("Buffer contents: {}", buffer);
                match buffer.strip_prefix("fold along ")
                {
                    Some(fold_line) =>
                    {
                        debug!("fold_line: {}", fold_line);
                        let mut iter = fold_line.split("=");
                        let coord = iter.next().unwrap();
                        if coord == "y"
                        {
                            fold_list.push((0, iter.next().unwrap().parse::<usize>().unwrap()));
                        }
                        else if coord == "x"
                        {
                            fold_list.push((iter.next().unwrap().parse::<usize>().unwrap(), 0));
                        }
                    }
                    None =>
                    {
                        panic!("The folding section should always have 'fold along ' in the string prefix.");
                    }
                }

                buffer.clear();
                read_result = read_trimmed_line(reader, &mut buffer);
            }
            Err(e) =>
            {
                panic!("An error occurred during the read-in of the file. {}", e);
            }
        }
    }

    return fold_list;
}

fn load_grid(coords: Vec<(usize, usize)>, grid: &mut Grid)
{
    // A little silly, but the Grid is kinda backwards compared to what this needs - so swap coords on load.
    for (y, x) in coords
    {
        grid[x][y] = 1;
    }
}

fn read_in_coords(reader: &mut BufReader<File>) -> Vec::<(usize, usize)>
{
    let mut buffer = String::new();
    let mut read_result = read_trimmed_line(reader, &mut buffer);

    let mut coords = Vec::<(usize, usize)>::new();

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
                
                let (x, y) = (iter.next().unwrap().parse::<usize>().unwrap(), iter.next().unwrap().parse::<usize>().unwrap());
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