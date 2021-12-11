use std::{io::{BufReader, Write, stdin, BufRead}, fs::File, path::Path};

pub fn read_int_from_file(reader:&mut BufReader<File>) -> Result<i32, String>
{
    
    let line_buffer = read_line_from_file(reader)?;

    if line_buffer == ""
    {
        return Ok(-1);
    }

    let possible_number = line_buffer.parse::<i32>();
    if possible_number.is_err()
    {
        return Err(format!("The value {} is not a valid number.", line_buffer));
    }

    return Ok(possible_number.unwrap());
}

pub fn read_trimmed_line(reader: &mut BufReader<File>, buffer: &mut String) -> Result<usize, String>
{
    let byte_count = reader.read_line(buffer);

    match byte_count
    {
        Ok(bytes_read) => 
        {
            if buffer.ends_with("\n")
            {
                buffer.pop();
                if buffer.ends_with("\r")
                {
                    buffer.pop();
                }
            }
            return Ok(bytes_read);
        },
        Err(_e) =>
        {
            return Err("Unable to read file contents for some reason.".to_string());
        }
    }
}

pub fn read_line_from_file(reader: &mut BufReader<File>) -> Result<String, String>
{
    let mut line_buffer = String::new();
    

    let byte_count = reader.read_line(&mut line_buffer);

    if byte_count.is_err()
    {
        return Err(String::from("Error occurred during read-in of line."));
    }
    else if byte_count.unwrap() == 0 {
        return Ok("".to_string());
    }

    if line_buffer.ends_with("\n")
    {
        line_buffer.pop();
        if line_buffer.ends_with("\r")
        {
            line_buffer.pop();
        }
    }
    
    return Ok(line_buffer);
}

pub fn get_reader() -> BufReader<File>
{
    let mut filename = String::new();

    print!("Path to input file: ");
    std::io::stdout().flush().unwrap();
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
        print!("Path to input file: ");
        std::io::stdout().flush().unwrap();
        stdin().read_line(&mut filename).expect("Apparently you are bad at typing?  Somehow?");

        let path = Path::new(&mut filename);

        open_result = File::open(path);
    }
    BufReader::new(open_result.unwrap())
}


pub fn bitstring_to_u16(bit_string: String) -> Result<u16, String>
{
    let mut value:u16 = 0;

    if bit_string.len() > 16
    {
        return Err("This bit string will not fit in an i16.".to_string());
    }

    for char in bit_string.chars()
    {
        value <<= 1;
        if char == '1'
        {
            value += 1;
        }
    }

    return Ok(value);
}

pub fn calc_most_common_bit(list: &Vec<u16>, bit_position: u16) -> u16
{
    let mut zero_count: u16 = 0;
    let mut one_count: u16 = 0;

    for value in list
    {
        if (value & bit_position) > 0
        {
            one_count += 1;
        }
        else 
        {
            zero_count += 1;
        }
    }

    if zero_count > one_count
    {
        return 0;
    }
    else
    {
        return 1;
    }
}

pub fn calc_least_common_bit(list: &Vec<u16>, bit_position: u16) -> u16
{
    let mut zero_count: u16 = 0;
    let mut one_count: u16 = 0;

    for value in list
    {
        if (value & bit_position) > 0
        {
            one_count += 1;
        }
        else 
        {
            zero_count += 1;
        }
    }

    if zero_count <= one_count
    {
        return 0;
    }
    else
    {
        return 1;
    }
}

pub fn frequency_calculator_bitstring(binary: String, counter: &mut [i32])
{
    let mut index = 0;

        for char in binary.chars()
        {
            if char == '1'
            {
                counter[index] += 1;
            }
            index = (index + 1) % 12;
        }

}

pub fn char_to_i8(char_digit: char) -> i8
{
    match char_digit
    {
        '0' => {0}
        '1' => {1}
        '2' => {2}
        '3' => {3}
        '4' => {4}
        '5' => {5}
        '6' => {6}
        '7' => {7}
        '8' => {8}
        '9' => {9}
        _ => {panic!("This character is not a number.")}
    }
}