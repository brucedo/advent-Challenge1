use std::{io::{stdin, BufRead, BufReader}, path::Path, fs::File, process::exit};


fn main() 
{
    let mut reader = get_reader();

    let mut count: i32 = 0;

    let mut last_num = read_int_from_file(&mut reader);

    if last_num.is_err()
    {
        println!("Something went wrong during the read in.");
        exit(-1);
    }

    let mut first_num = last_num.unwrap();
    
    last_num = read_int_from_file(&mut reader);
    if last_num.is_err()
    {
        println!("Something went wrong during the read in.");
        exit(-1);
    }
    let mut second_num = last_num.unwrap();

    while second_num != -1 
    {
        if second_num > first_num
        {
            count += 1;
        }
        first_num = second_num;

        last_num = read_int_from_file(&mut reader);
        if last_num.is_err()
        {
        println!("Something went wrong during the read in.");
        exit(-1);
        }
        second_num = last_num.unwrap();
    }

    println!("Total number of increases: {}.", count);
}

fn read_int_from_file(reader:&mut BufReader<File>) -> Result<i32, String>
{
    let mut line_buffer = String::new();
    

    let byte_count = reader.read_line(&mut line_buffer);

    if byte_count.is_err()
    {
        return Err(String::from("Error occurred during read-in of line."));
    }
    else if byte_count.unwrap() == 0 {
        return Ok(-1);
    }

    if line_buffer.ends_with("\n")
    {
        line_buffer.pop();
    }
    if line_buffer.ends_with("\r")
    {
        line_buffer.pop();
    }

    let possible_number = line_buffer.parse::<i32>();
    if possible_number.is_err()
    {
        return Err(format!("The value {} is not a valid number.", line_buffer));
    }

    return Ok(possible_number.unwrap());
}

fn get_reader() -> BufReader<File>
{
    let mut filename = String::new();

    println!("Path to input file: ");
    stdin().read_line(&mut filename).expect("Apparently you are bad at typing?  Somehow?");
    if filename.ends_with("\n")
    {
        filename.pop();
    }
    if filename.ends_with("\r")
    {
        filename.pop();
    }
    println!("Path read in: {}", filename);
    let path = Path::new(&mut filename);

    let mut open_result = File::open(path);
    // let file: File;

    while open_result.is_err()
    {
        let error = open_result.unwrap_err();
        println!("The path provided does not point to a valid readable file.  Error: {}", error.to_string());
        
        // println!("Results from opening file {}", String::from(path.to_str().unwrap()));
        println!("Path to input file: ");

        stdin().read_line(&mut filename).expect("Apparently you are bad at typing?  Somehow?");

        let path = Path::new(&mut filename);

        open_result = File::open(path);
    }

    BufReader::new(open_result.unwrap())
}

