use std::{io::BufReader, fs::File, panic};

use crate::common::common::{get_reader, read_trimmed_line};

pub fn challenge_day_10()
{
    let mut reader = get_reader();

    part_one(&mut reader);
}

fn part_one(reader: &mut BufReader<File>)
{
    let mut buffer = String::new();
    let mut program = Vec::<char>::new();
    let mut illegals = Vec::<char>::new();

    let mut incompletes = Vec::<Vec<char>>::new();

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

                for character in buffer.chars()
                {
                    match character
                    {
                        '(' | '<' | '{' | '[' =>
                        {
                            program.push(character);
                        }
                        ')' | '>' | '}' | ']' =>
                        {
                            let scope = program.pop().unwrap();
                            if get_closer(scope) != character
                            {
                                illegals.push(character);
                                // illegal character - clear the program.
                                program.clear();
                                break;
                            }                            
                        }
                        _ =>
                        {
                            panic!("Unexpected character {} in input.", character);
                        }
                    }
                }
                
                // clear  the buffer for a new line
                if !program.is_empty()
                {
                    incompletes.push(program);
                    program = Vec::<char>::new();
                }
                buffer.clear();
                result = read_trimmed_line(reader, &mut buffer);
            }
            Err(e) =>
            {
                println!("An error occurred reading a line in: {}", e);
                return;
            }
        }
    }

    println!("We found the following illegal characters: {:?}", illegals);
    println!("Total score of all illegals: {}", score_illegals(&illegals));

    println!("There are {} incomplete lines to evaluate.", incompletes.len());
    part_two(incompletes);
}

fn part_two(incompletes: Vec<Vec<char>>)
{
    let mut closer = Vec::<char>::new();
    let mut scores = Vec::<u64>::new();

    for mut incomplete in incompletes
    {
        while !incomplete.is_empty()
        {
            let open = incomplete.pop().unwrap();
            let matcher = get_closer(open);
            closer.push(matcher);
        }
        scores.push(evaluate_closer_score(&closer));
        closer.clear();
    }

    scores.sort();
    println!("We have the following scores: {:?}", scores);
    println!("There are {} total scores", scores.len());
    println!("The middle score which apparently matters is {}, located at index {}", scores[(scores.len() / 2)], (scores.len() / 2))
    

}

fn evaluate_closer_score(closer: &Vec<char>) -> u64
{
    let mut total = 0;

    for character in closer
    {
        total *= 5;
        match character
        {
            ')' => {total += 1;}
            ']' => {total += 2;}
            '}' => {total += 3;}
            '>' => {total += 4;}
            _ => { panic! ("Totally invalid closing character {}.  How are these getting in here?!", character); }
        }
    }

    return total;
}

fn get_closer(open: char) -> char
{
    match open
    {
        '(' => {')'}
        '[' => {']'}
        '{' => {'}'}
        '<' => {'>'}
        _ => {
            panic!("Definitely illegal character {}.", open);
        }
    }
}

fn score_illegals(illegals: &Vec<char>) -> u32
{
    let mut score = 0;
    for illegal_character in  illegals
    {
        match illegal_character
        {
            ')' =>
            {
                score += 3;
            }
            ']' =>
            {
                score += 57;
            }
            '}' =>
            {
                score += 1197
            }
            '>' =>
            {
                score += 25137
            }
            _ =>
            {
                panic!("Non-illegal character {} in mix.  Panic at the disco.", illegal_character);
            }
        }
    }

    return score;
}