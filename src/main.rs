use std::io::{stdin};

fn main() {
    let mut infile_path = String::new();

    println!("Path to input file: ");
    stdin().read_line(&mut &mut infile_path).expect("Apparently you are bad at typing?  Somehow?");

}

