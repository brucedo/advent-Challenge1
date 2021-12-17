use std::{io::{BufReader}, fs::File, collections::{HashMap, hash_map::{Entry}}, cmp::max, cmp::min};

use log::debug;

use crate::common::common::{get_reader, read_trimmed_line};

pub fn challenge_day_14()
{
    let mut reader = get_reader();

    part_two(&mut reader);
}


fn part_two(reader: &mut BufReader<File>)
{
    let mut starting_compound = read_starting_compound(reader);
    let polymerization_rules = read_polymerization_rules(reader);
    let mut count_letters: [u64; 26] = [0;26];

    let mut table = Vec::<HashMap<(usize, usize), u64>>::new();

    // pre-count the input string
    
    // Explicitly increment the left-most letter of the top of the stack (which is the former left-most pair in the string)
    count_letters[starting_compound[starting_compound.len() - 1].0] += 1;
    for pair in &mut starting_compound
    {
        // and then increment only the right-most letter of every pair including the first.
        count_letters[pair.1] += 1;
    }

    debug!("input sequence is: {:?}", starting_compound);

    // Load the first row of the table
    let mut row = HashMap::<(usize, usize), u64>::new();
    for pair in &starting_compound
    {
        let entry = row.entry(*pair);
        match entry
        {
            Entry::Occupied(mut value) => 
            {
                *value.get_mut() += 1;
            },
            Entry::Vacant(value) => 
            {
                value.insert(1);
            },
        }
    }

    table.push(row);

    println!("State of lookup table: {:?}", polymerization_rules);

    println!("Starting 40 rounds...");
    for round_i in 0..40
    {
        debug!("hitting round {}", round_i);

        debug!("current state of table: ");
        for temp_row in &table
        {
            for ((temp_row, temp_col), temp_val) in temp_row
            {
                debug!("{}{}: {} ", (*temp_row as u8 + 65) as char,(*temp_col as u8 + 65) as char, temp_val);
            }
            debug!("\n");
        }

        
        let row = &table[round_i];
        let mut next_row = HashMap::<(usize, usize), u64>::new();

        for (key, value) in row
        {

            let lookup_row = key.0;
            let lookup_col = key.1;
            debug!("Looking up polymerization entry {}, {}", key.0, key.1);
            let pair = polymerization_rules[lookup_row][lookup_col];
            debug!("Retrieved pair: {:?}", pair);
            debug!("Pair.0.1: {}", pair.0.1);

            // We cannot just add one to count_letters anymore - we may have 4 or 5 or 10 different
            // occurrences of the key that generates the new letter, but we only visit each key once on a
            // pass.
            count_letters[pair.0.1] += value;

            match next_row.entry(pair.0)
            {
                Entry::Occupied(mut entry) =>
                {
                    (*entry.get_mut()) += value;
                }
                Entry::Vacant(entry) =>
                {
                    entry.insert(*value);
                }
            }

            debug!("next_row.entry: {}", next_row.get(&pair.0).unwrap());

            match next_row.entry(pair.1)
            {
                Entry::Occupied(mut entry) =>
                {
                    (*entry.get_mut()) += value;
                }
                Entry::Vacant(entry) =>
                {
                    entry.insert(*value);
                }
            }
        }

        table.push(next_row);
    }


    debug!("Finished contents of letter frequencies (excepting the last letter in the input pattern: \n{:?}", count_letters);

    let mut max_frequency: u64 = 0;
    let mut min_frequency: u64 = u64::MAX;

    for i in 0..count_letters.len() - 1
    {
        max_frequency = max(max_frequency, count_letters[i]);
        if count_letters[i] > 0
        {
            min_frequency = min(min_frequency, count_letters[i]);
        }
    }

    println!("The most frequently occurring element occurs {} times.", max_frequency);
    println!("The least frequently occurring element occurs {} times.", min_frequency);
    println!("The incredibly arbitrary metric for this challenge therefore is {}", max_frequency - min_frequency);
}

fn read_starting_compound(reader: &mut BufReader<File>) -> Vec<(usize, usize)>
{
    let mut buffer = String::new();
    let mut read_result = read_trimmed_line(reader, &mut buffer);
    let mut starting_compound = Vec::<usize>::new();
    let mut processed_stack = Vec::<(usize, usize)>::new();

    loop {
        match read_result
        {
            Ok(size) =>
            {
                if size == 0
                {
                    panic!("File read ended while consuming the starting compound.");
                }
                if buffer == ""
                {
                    break;
                }
                // process String into vec of chars
                for element in buffer.chars()
                {
                    starting_compound.push((element.to_ascii_uppercase() as u8 - 'A'.to_ascii_uppercase() as u8) as usize);
                }

                for i in 1..starting_compound.len()
                {
                    processed_stack.insert(0, (starting_compound[i-1], starting_compound[i]));
                }


                buffer = String::new();

                read_result = read_trimmed_line(reader, &mut buffer);
            }
            Err(e) =>
            {
                panic!("Read file went wrong: {}", e);
            }
        }
    }

    debug!("Returning starting_compound string: {:?}", starting_compound);

    return processed_stack;
}

fn read_polymerization_rules(reader: &mut BufReader<File>) -> [[((usize,usize),(usize,usize));26];26]
{
    let mut buffer = String::new();
    let mut read_result = read_trimmed_line(reader, &mut buffer);
    let mut polymerization_rules: [[((usize,usize),(usize,usize));26];26] = [[((0,0),(0,0));26];26];


    loop {
        match read_result
        {
            Ok(size) =>
            {
                if size == 0
                {
                    break;
                }

                let mut rule_iter = buffer.split(" -> ");
                let (start, inject) = (rule_iter.next().unwrap().to_string(), rule_iter.next().unwrap().chars().next().unwrap());
                // 
                debug!("start pair: {}\ninjected: {}", start, inject);
                let mut pair = start.chars();
                let row = (pair.next().unwrap() as u8 - 'A' as u8) as usize;
                let col = (pair.next().unwrap() as u8 - 'A' as u8) as usize;
                let first = (row, (inject as u8 - 'A' as u8) as usize);
                let second = ((inject as u8 - 'A' as u8) as usize, col);

                polymerization_rules[row as usize][col as usize] = (first, second);

                buffer.clear();
                read_result = read_trimmed_line(reader, &mut buffer);
            }
            Err(e) =>
            {
                panic!("Read file went wrong: {}", e);
            }
        }
    }

    return polymerization_rules;
}