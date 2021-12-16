use std::{io::{BufReader}, fs::File};

use log::{error, debug};

use crate::common::common::{get_reader, read_trimmed_line};

pub fn challenge_day_six()
{
    let mut reader = get_reader();

    day6_part_one(&mut reader);
}

fn day6_part_one(reader: &mut BufReader<File>)
{
    let mut buffer = String::new();
    let response = read_trimmed_line(reader, &mut buffer);

    let mut fishes: [u64; 9] = [0; 9];



    match response 
    {
        Ok(_size) =>
        {
            // This one's input has only one line - no need to loop input in or check to see when we hit EOF.
            for fish_count in buffer.split(',')
            {
                // we know from inspection that the input is always between 0 and 7.  No, I would not normally just do this.
                fishes[fish_count.parse::<usize>().unwrap()] += 1;
            }
        }
        Err(e) =>
        {
            error!("An error occurred reading the file: {}", e);
            return
        }
    }

    // okay, fishes should be loaded.  loop 80 times and on each iteration: if some fish entries value is 0,
    // add 6 and push a new value of 8 onto the end.  Of course we don't want to trigger any of the newly added
    // fish, so be sure that we only iterate across indices 0 .. vec.len() at the start of the day.
    
    for _j in 0..256
    {
        debug!("today (day {}) there are {:?} fish.", _j, fishes);
        let temp = fishes[0];
        for i in 1..9
        {
            debug!("i: {}, fishes[i-1]: {}, fishes[i]: {}", i, fishes[i-1], fishes[i]);
            fishes[i-1] = fishes[i];
        }
        // empty the 8th bucket
        fishes[8] = 0;

        // now update the 8th and 6th buckets.
        fishes[6] += temp;
        fishes[8] += temp;
    }

    let mut total = 0;
    for _i in 0..9
    {
        total += fishes[_i];
    }
    println!("There are a total of {} fish.", total);

    
}

