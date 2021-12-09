pub mod bingo;
pub mod lines;
pub mod common;
pub mod day;
pub mod digit;

use std::io::{Write, stdin};

use day::{day2::challenge_day_two, day1::challenge_day_one, day3::challenge_day_three,
    day4::challenge_day_four,day5::challenge_day_five, day6::challenge_day_six};

use crate::{lines::grid::Grid, day::{day7::challenge_day_7, day8::challenge_day_8}, };

fn main() 
{
    env_logger::init();

    let mut choice = String::new();

    print!("Which day's challenge to run (1-8): ");
    std::io::stdout().flush().unwrap();
    stdin().read_line(&mut choice).expect("Apparently you are bad at typing?  Somehow?");    

    if choice.ends_with("\n")
    {
        choice.pop();
        
        if choice.ends_with("\r")
        {
            choice.pop();
        }
    }

    match choice.as_str()
    {
        "1" => {
            challenge_day_one()
        }
        "2" => {
            challenge_day_two()
        }
        "3" => {
            challenge_day_three()
        }
        "4" =>
        {
            challenge_day_four()
        }
        "5" =>
        {
            challenge_day_five()
        }
        "6" =>
        {
            challenge_day_six()
        }
        "7" =>
        {
            challenge_day_7();
        }
        "8" =>
        {
            challenge_day_8();
        }
        _ => {
            println!("Pick a real number next time.");
        }
    }
}













