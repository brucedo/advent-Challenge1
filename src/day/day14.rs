use std::{io::{BufReader, Seek, SeekFrom}, fs::File, collections::{HashMap, hash_map::{Entry}}, cmp::max, cmp::min};

use log::debug;

use crate::common::common::{get_reader, read_trimmed_line};

pub fn challenge_day_14()
{
    let mut reader = get_reader();

    part_one(&mut reader);

    let result = reader.seek(SeekFrom::Start(0));
    match result
    {
        Err(e) => 
        {
            println!("An error occurred attempting to rewind the file.  Error: {}\nAttempting reopen: ", e);
            reader = get_reader();
        }
        _ => {}
    }

    part_two(&mut reader);
}

fn part_one(reader: &mut BufReader<File>)
{
    let mut starting_compound = read_starting_compound(reader);
    let polymerization_rules = read_polymerization_rules(reader);
    let mut combination = String::new();

    for i in 0 .. 10
    {
        // for j in (1 .. starting_compound.len()).step_by(2)
        let mut j = 1;
        while j < starting_compound.len()
        {
            let char0 = starting_compound[j - 1];
            let char1 = starting_compound[j];
            
            combination.push(char0);
            combination.push(char1);

            let injectable = polymerization_rules.get(&combination).unwrap();

            starting_compound.insert(j, *injectable);

            combination.clear();
            j += 2;
        }
        debug!("After pass {} our compound is: {:?}", i, starting_compound);
    }

    letter_analysis(starting_compound);
}

fn part_two(reader: &mut BufReader<File>)
{
    let starting_compound = read_starting_compound(reader);
    let polymerization_rules = read_polymerization_rules(reader);
    let mut count_letters = HashMap::<char, u64>::new();

    for j in 1..starting_compound.len()
    {
        println!("Starting solution between pairs {} and {}", j-1, j);
        let left = starting_compound[j-1];
        let right = starting_compound[j];
        // additive_merge
        // (
        //     &mut count_letters, 
        //     &expand_pair(25, left, right, &polymerization_rules)
        // );
        expand_pair(40, left, right, &polymerization_rules, &mut count_letters);
    }

    // last fixer upper
    let entry = count_letters.entry(starting_compound[starting_compound.len()-1]);
    match entry
    {
        Entry::Vacant(entry) =>
        {
            entry.insert(1);
        }
        Entry::Occupied(mut entry) =>
        {
            entry.insert(entry.get() + 1);
        }
    }

    debug!("Finished contents of letter frequencies (excepting the last letter in the input pattern: \n{:?}", count_letters);

    let mut max_frequency: u64 = 0;
    let mut min_frequency: u64 = u64::MAX;

    for value in count_letters.values()
    {
        max_frequency = max(max_frequency, *value);
        min_frequency = min(min_frequency, *value);
    }

    println!("The most frequently occurring element occurs {} times.", max_frequency);
    println!("The least frequently occurring element occurs {} times.", min_frequency);
    println!("The incredibly arbitrary metric for this challenge therefore is {}", max_frequency - min_frequency);
}

fn expand_pair(depth: usize, left_char: char, right_char: char, rules: &HashMap<String, char>, count_letters: &mut HashMap<char, u64>) //-> HashMap<char, u64>
{
    // let mut count_letters;

    let mut pair = String::new();
    pair.push(left_char);
    pair.push(right_char);
    let middle = *rules.get(&pair).unwrap();

    debug!("Starting expansion round at depth {} with : {}{}", depth, left_char, right_char);

    if depth == 0
    {
        // if is_right_branch
        // {
        //     debug!("right-branch: {}{}{}", left_char, middle, right_char);
        //     count_letters = letter_count(left_char, middle, Some(right_char));
        // }
        // else
        // {
            debug!("left-branch: {}{}", left_char, middle);
            // count_letters = letter_count(left_char, middle, None);
            letter_count(left_char, middle, None, count_letters);
            return
        // }
    }
    
        // count_letters = expand_pair(depth - 1, left_char, middle, rules);
    expand_pair(depth - 1, left_char, middle, rules, count_letters);
        // let right_count = expand_pair(depth - 1, middle, right_char, rules);
    expand_pair(depth - 1, middle, right_char, rules, count_letters);

        // additive_merge(&mut count_letters, &right_count);
        // for (element, count) in right_count
        // {
        //     match &count_letters.get(&element)
        //     {
        //         Some(&left_count) =>
        //         {
        //             &count_letters.insert(element, count + left_count);
        //         }
        //         None =>
        //         {
        //             count_letters.insert(element, count);
        //         }
        //     }
        // }

    // }

    // return count_letters;
}

fn additive_merge(left: &mut HashMap<char, u64>, right: &HashMap<char, u64>)
{
    for (element, count) in right
    {
        match &left.get(&element)
        {
            Some(&left_count) =>
            {
                &left.insert(*element, count + left_count);
            }
            None =>
            {
                left.insert(*element, *count);
            }
        }
    }
}

fn letter_count(left: char, middle: char, right: Option<char>, letter_count: &mut HashMap<char, u64>) //-> HashMap<char, u64>
{
    // let mut letter_count = HashMap::<char, u64>::new();
    if letter_count.contains_key(&left)
    {
        letter_count.insert(left, *(letter_count.get(&left).unwrap()) + 1);
    }
    else 
    {
        letter_count.insert(left, 1);
    }

    if letter_count.contains_key(&middle)
    {
        letter_count.insert(middle, *(letter_count.get(&middle).unwrap()) + 1);
    }
    else 
    {
        letter_count.insert(middle, 1);
    }

    match right
    {
        Some(letter) =>
        {
            if letter_count.contains_key(&letter)
            {
                letter_count.insert(letter, *(letter_count.get(&letter).unwrap()) + 1);
            }
            else 
            {
                letter_count.insert(letter, 1);
            }
        }
        _ => {}
    }
    // return letter_count;
}

fn letter_analysis(polymer: Vec<char>)
{
    let mut letter_count = HashMap::<char, u64>::new();

    for element in polymer
    {
        if letter_count.contains_key(&element)
        {
            letter_count.insert(element, *(letter_count.get(&element).unwrap()) + 1);
        }
        else 
        {
            letter_count.insert(element, 1);
        }
    }

    println!("Element frequency count is: {:?}", letter_count);
    let mut max_frequency: u64 = 0;
    let mut min_frequency: u64 = u64::MAX;

    for value in letter_count.values()
    {
        max_frequency = max(max_frequency, *value);
        min_frequency = min(min_frequency, *value);
    }

    println!("The most frequently occurring element occurs {} times.", max_frequency);
    println!("The least frequently occurring element occurs {} times.", min_frequency);
    println!("The incredibly arbitrary metric for this challenge therefore is {}", max_frequency - min_frequency);
}

fn read_starting_compound(reader: &mut BufReader<File>) -> Vec<u8>
{
    let mut buffer = String::new();
    let mut read_result = read_trimmed_line(reader, &mut buffer);
    let mut starting_compound = Vec::<u8>::new();

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
                    starting_compound.push(element.to_ascii_uppercase() as u8);
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

    return starting_compound;
}

fn read_polymerization_rules(reader: &mut BufReader<File>) -> HashMap<String, char>
{
    let mut buffer = String::new();
    let mut read_result = read_trimmed_line(reader, &mut buffer);
    let mut polymerization_rules = HashMap::<String, char>::new();

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
                polymerization_rules.insert(start, inject);

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