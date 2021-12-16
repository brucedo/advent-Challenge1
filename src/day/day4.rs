use std::{io::{BufReader}, fs::File};

use crate::{bingo::bingo::Bingo, common::common::{read_trimmed_line, get_reader}};

pub fn challenge_day_four()
{
    let mut reader = get_reader();

    day_4_part_one(&mut reader);
}

fn day_4_part_one(reader: &mut BufReader<File>)
{
    let mut game = Bingo::new();
    let mut call_string = String::new();
    let mut board_line = String::new();
    let mut board_finished = true;
    let mut current_board: usize = 0;
    let mut row: usize = 0;
    let mut col: usize = 0;
    let mut read_line:Result<usize, String>;

    // Load the game state, starting with the string of numbers to call
    let _= read_trimmed_line(reader, &mut call_string);

    // now the boards
    loop
    {
        board_line.clear();
        read_line = read_trimmed_line(reader, &mut board_line);
        match read_line
        {
            Ok(size) => {
                if size == 0
                {
                    break
                }
            }
            Err(message) => {
                println!("{}", message);
                return;
            }
        }

        if board_line == ""
        {
            board_finished = true;
            continue
        }
        else
        {
            if board_finished
            {
                current_board = game.new_board();
                row = 0;
                board_finished = false;
            }

            for num in board_line.split(" ")
            {
                if num == ""
                {
                    continue;
                }
                let result = num.parse::<i32>();
                match result {
                    Ok(value) => {
                        game.set_number_on_board(current_board, row, col, value);
                    },
                    Err(e) => {
                        println!("Unable to convert string {} to int with error {}", num, e.to_string());
                        return;
                    }
                }
                col += 1;
            }
            row += 1;
            col = 0;
        }
    }

    println!("Finished loading game boards.");
    println!("{}", game);
    
    // boards loaded.  Start simulating the game.
    for num in call_string.split(",")
    {
        println!("Calling bingo number {}", num);
        let result = num.parse::<i32>();
        match result {
            Ok(value) => {
                game.call(value);
            },
            Err(e) =>
            {
                println!("Unable to convert called number in string {} to int with error {}", num, e.to_string());
            }
        }
    }
    println!("First game won  with score {:?}",  game.won_scores);
}