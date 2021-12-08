use std::{io::{SeekFrom, Seek, BufReader}, fs::File, cmp::max, convert::TryInto};

use log::{error, debug};

use crate::{lines::{line::Line, grid::Grid}, common::common::{read_trimmed_line, get_reader}};

pub fn challenge_day_five()
{
    let mut reader = get_reader();

    day5_part_one(&mut reader);
}

fn day5_part_one(reader: &mut BufReader<File>)
{
    let mut buffer = String::new();
    let mut x_max = 0;
    let mut y_max = 0;
    let mut lines:Vec<Line> = Vec::new();

    loop
    {
        // clean the string buffer out
        buffer.clear();
        let read_result = read_trimmed_line(reader, &mut buffer);
        match read_result
        {
            Ok(size) =>
            {
                // 0 size - EOF
                if size == 0
                {
                    break;
                }
                else
                {
                    // Read the line from number, number -> number, number into a Line
                    let line = convert_to_line(&buffer);
                    debug!("Read line {}, {} -> {}, {}", line.x1, line.y1, line.x2, line.y2);
                    // if !(line.is_horizontal() || line.is_vertical())
                    // {
                    //     debug!("Skipping line {}, {} -> {}, {} for being crooked", line.x1, line.y1, line.x2, line.y2);
                    //     continue;
                    // }
                    x_max = max(x_max, max(line.x1, line.x2));
                    y_max = max(y_max, max(line.y1, line.y2));
                    lines.push(line);
                }
            },
            Err(e ) =>
            {
                error!("An error occurred during the read in of a line of the file: {}", e);
                return;
            }
        }
        
    }

    debug!("Max x and max y: {}, {}", x_max, y_max);

    // Lines read in - generate grid and then insert.
    let mut grid = Grid::new();
    grid.init((x_max + 1).try_into().unwrap(), (y_max + 1).try_into().unwrap());

    for line in lines
    {
        if !(line.is_horizontal() || line.is_vertical())
        {
            grid.insert_diagonal_line(&line)
        }
        else
        {
            grid.insert_flat_line(&line);
        }
    }

    // Now get intersection coverage
    let mut coverage = 0;
    for i in 0..(x_max + 1)
    {
        for j in 0..(y_max + 1)
        {
            if grid.get_value_at(i.try_into().unwrap(), j.try_into().unwrap()) > 1
            {
                coverage += 1
            }
        }
    }

    println!("The intersection coverage is {} squares.", coverage);
}

fn convert_to_line(input: &String) -> Line
{
    let mut line = Line::new();
    let mut index = 0;
    let mut temp:[i16; 4] = [0;4];

    debug!("Converting string '{}'", input);

    // Every string is of the general form 'number,number -> number,number.  We can get the number pairs with a spliterator
    for pair in input.split(" -> ")
    {
        for coord in pair.split(",")
        {
            temp[index] = coord.parse::<i16>().unwrap();
            index += 1;
        }
    }

    line.x1 = temp[0];
    line.y1 = temp[1];
    line.x2 = temp[2];
    line.y2 = temp[3];

    return line;
}