pub mod bingo;

use std::{io::{stdin, BufRead, BufReader, Seek, SeekFrom, Write}, path::Path, fs::File, };

use bingo::bingo::Bingo;

fn main() 
{
    let mut choice = String::new();

    print!("Which day's challenge to run (1-4): ");
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
        _ => {
            println!("Pick a real number next time.");
        }
    }
}

fn challenge_day_four()
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

    // Load the game state, starting with the string of numbers to call
    let mut read_line= read_trimmed_line(reader, &mut call_string);

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

fn challenge_day_three()
{
    let mut reader = get_reader();
    let mut row_count = 0;

    // The frequency count is used by both problems.  Get it here.
    let mut frequency = [0;12];
    loop 
    {
        let line_result = read_line_from_file(&mut reader);
        match line_result {
            Ok(result) => {
                if result == ""
                {
                    break;
                }
                row_count += 1;
                frequency_calculator_bitstring(result, &mut frequency)
            },
            Err(e) => {
                println!("An error occurred reading a line from the file.  Dumping.\n{}",e);
                return;
            }
        }
    }

    // rewind for task one
    let mut seek = reader.seek(SeekFrom::Start(0));
    if seek.is_err()
    {
        println!("Problem seeking to the start of the file stream.  Reopening file:");
        reader = get_reader();
    }

    day_3_part_one(&mut reader, &frequency, row_count);

    // rewind for task two
    seek = reader.seek(SeekFrom::Start(0));
    if seek.is_err()
    {
        println!("Problem seeking to the start of the file stream.  Reopening file:");
        reader = get_reader();
    }

    day_3_part_two(&mut reader);
}

fn bitstring_to_u16(bit_string: String) -> Result<u16, String>
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

fn calc_most_common_bit(list: &Vec<u16>, bit_position: u16) -> u16
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

fn calc_least_common_bit(list: &Vec<u16>, bit_position: u16) -> u16
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

fn day_3_part_two(reader: &mut BufReader<File>) -> Result<String, String>
{
    
    let mut o2_candidates: Vec<u16> = Vec::new();
    let mut co2_candidates: Vec<u16> = Vec::new();
    let mut curr_bit_pos:u16 = 0x0800;

    // Load'em up.
    loop 
    {
        let bit_string = read_line_from_file(reader)?;
        if bit_string == ""
        {
            break;
        }
        let diagnostic_value = bitstring_to_u16(bit_string)?;
        o2_candidates.push(diagnostic_value);
        co2_candidates.push(diagnostic_value);
    }

    // clean 02 candidates first
    loop 
    {
        if o2_candidates.len() == 1
        {
            break;
        }

        let most_common = calc_most_common_bit(&o2_candidates, curr_bit_pos);

        let mut o2_index = 0;
        while o2_index < o2_candidates.len()
        {
            let masked_candidate = o2_candidates[o2_index] & curr_bit_pos; // e.g. 010110101001 & 000000001000 = 000000001000; 010110100001 & 000000001000 = 000000000000

            if ((most_common == 1) && (masked_candidate == 0)) || ((most_common == 0) && (masked_candidate > 0))
            {
                o2_candidates.remove(o2_index);
            }
            else
            {
                o2_index += 1;
            }
        }
        curr_bit_pos >>= 1;
    }

    let o2 = o2_candidates[0];

    // reset, then the co2 candidates
    curr_bit_pos = 0x0800;
    loop
    {
        if co2_candidates.len() == 1
        {
            break;
        }

        let least_common = calc_least_common_bit(&co2_candidates, curr_bit_pos);

        let mut co2_index = 0;
        while co2_index < co2_candidates.len()
        {
            
            let masked_candidate = co2_candidates[co2_index] & curr_bit_pos;

            if ((least_common == 1) && (masked_candidate == 0)) || ((least_common == 0) && (masked_candidate > 0))
            {
                co2_candidates.remove(co2_index);
            }
            else
            {
                co2_index += 1;
            }
        }
        curr_bit_pos >>= 1;
    }

    let co2 = co2_candidates[0];


    println!("O2 value: {}, {:b} in binary.", o2, o2);
    println!("CO2 value: {}, {:b} in binary.", co2, co2);
    println!("O2 x CO2: {}", i32::from(o2) * i32::from(co2));

    return Ok("".to_string());

}

fn day_3_part_one(reader: &mut BufReader<File>, frequency: &[i32], row_count: i32)
{
    
    let mut gamma:i32 = 0;
    let mut index = 0;
    let mut epsilon:i32 = 0;

    
    // Analysis
    println!("Total rows: {}", row_count);
    print!("Frequency count: ");
    loop
    {
        print!("{},", frequency[index]);
        gamma <<= 1;
        epsilon <<= 1;
        if frequency[index] > (row_count - frequency[index])
        {
            gamma += 1;
        }
        else if frequency[index] < (row_count - frequency[index])
        {
            epsilon += 1;
        }
        else
        {
            println!("There's an equality case here.  You were wrong, it _can_ happen.");
        }
        index += 1;
        if index >= 12
        {
            break;
        }
    }
    print!("\n");
    print!("\n");

    println!("Gamma: {}", gamma);
    println!("Epsion: {}", epsilon);
    println!("Gamma x Epsilon: {}", gamma * epsilon);
}

fn frequency_calculator_bitstring(binary: String, counter: &mut [i32])
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

fn challenge_day_two()
{
    let mut reader = get_reader();
    day_two_part_one(&mut reader);
    
    let rewind_result = reader.seek(SeekFrom::Start(0));
    
    if rewind_result.is_err()
    {
        println!("Attempted to rewind to re-read file, but something blew up.  Reloading file.");
        reader = get_reader();
    }

    day_two_part_two(&mut reader);
}

fn day_two_part_two(reader: &mut BufReader<File>)
{
    let mut aim: i64 = 0;
    let mut horizontal: i64 = 0;
    let mut depth: i64 = 0;

    let mut result = read_line_from_file(reader);
    let mut line: String;

    if result.is_err()
    {
        println!("Unable to read a line.");
        return;
    }

    line = result.unwrap();

    while line != "".to_string()
    {
        let mut parts = line.split(" ");
        let direction = parts.next().unwrap();
        let parse_result = parts.next().unwrap().parse::<i64>();

        if parse_result.is_err()
        {
            println!("Well that didn't work.  The distance value is not an integer.");
        }

        let distance = parse_result.unwrap();

        match direction {
            "forward" =>
            {
                horizontal += distance;
                depth += distance * aim;
            },
            "up" =>
            {
                aim -= distance;
            }
            "down" =>
            {
                aim += distance;
            }
            _ =>
            {
                println!("Somehow got a string value for direction that is not forward, up, or down: {}", direction);
            }
        }

        result = read_line_from_file(reader);
        if result.is_err()
        {
            println!("Unable to read a line.");
            return;
        }
    
        line = result.unwrap();
    }

    println!("Final horizontal position: {}", horizontal);
    println!("Final depth: {}", depth);
    println!("Final product: {}", horizontal * depth);
}

fn day_two_part_one(reader: &mut BufReader<File>)
{
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;

    let mut result = read_line_from_file(reader);
    let mut line: String;

    if result.is_err()
    {
        println!("Unable to read a line.");
        return;
    }

    line = result.unwrap();

    while line != "".to_string()
    {
        
        let mut parts = line.split(" ");
        let direction = parts.next().unwrap();
        let distance = parts.next().unwrap().parse::<i32>();

        if distance.is_err()
        {
            println!("Well that didn't work.  The distance value is not an integer.");
        }

        match direction {
            "forward" =>
            {
                horizontal += distance.unwrap();
            },
            "up" =>
            {
                depth -= distance.unwrap();
            }
            "down" =>
            {
                depth += distance.unwrap();
            }
            _ =>
            {
                println!("Somehow got a string value for direction that is not forward, up, or down: {}", direction);
            }
        }

        result = read_line_from_file(reader);
        if result.is_err()
        {
            println!("Unable to read a line.");
            return;
        }
    
        line = result.unwrap();
    }

    println!("Total horizontal distance travelled: {}", horizontal);
    println!("Total depth traversed: {} ", depth);
    println!("Multiple: {} ", horizontal * depth);
}

fn challenge_day_one()
{

    let mut reader = get_reader();

    let increase_result = count_increases(&mut reader);

    if increase_result.is_err()
    {
        println!("{}", increase_result.unwrap_err());
    }
    else
    {
        let count = increase_result.unwrap();
        println!("Total number of increases: {}.", count);
    }

    let reset_success = reader.seek(SeekFrom::Start(0));

    if reset_success.is_err()
    {
        println!("Something went wrong rewinding the file to run part 2.  Reopening file...");
        reader = get_reader();
    }

    let rolling_increase_result = count_rolling_increases(&mut reader);
    if rolling_increase_result.is_err()
    {
        println!("{}", rolling_increase_result.unwrap_err());
    }
    else
    {
        let rolling_count = rolling_increase_result.unwrap();
        println!("Total number of rolling window increases: {}", rolling_count);
    }
}

fn count_rolling_increases(reader: &mut BufReader<File>) -> Result<i32, String>
{
    // initialize our window
    let mut sum_window: [i32; 3] = [0, 0, 0];
    let mut first:i32;
    let mut second = -1;
    let mut k = 0;

    let mut count = 0;

    // Prime the pump...
    let mut next_num = read_int_from_file(reader)?;
    sum_window[0] += next_num;

    next_num = read_int_from_file(reader)?;
    sum_window[0] += next_num;
    sum_window[1] += next_num;

    next_num = read_int_from_file(reader)?;


    while next_num > -1 {

        sum_window[0] += next_num;
        sum_window[1] += next_num;
        sum_window[2] += next_num;

        first = second;
        second = sum_window[k];
        sum_window[k] = 0;

        k = (k + 1) % 3;

        if first > -1 && second > -1
        {
            if first < second
            {
                count += 1;
            }
        }

        next_num = read_int_from_file(reader)?;
    }

    return Ok(count);
}

fn count_increases(reader: &mut BufReader<File>) -> Result<i32, String>
{
    let mut count: i32 = 0;

    let mut last_num = read_int_from_file(reader);

    if last_num.is_err()
    {
        return Err("Something went wrong during the read in.".to_string());
    }

    let mut first_num = last_num.unwrap();
    
    last_num = read_int_from_file(reader);
    if last_num.is_err()
    {
        return Err("Something went wrong during the read in.".to_string());
        
    }
    let mut second_num = last_num.unwrap();

    while second_num != -1 
    {
        if second_num > first_num
        {
            count += 1;
        }
        first_num = second_num;

        last_num = read_int_from_file(reader);
        if last_num.is_err()
        {
            return Err("Something went wrong during the read in.".to_string());
        
        }
        second_num = last_num.unwrap();
    }

    return Ok(count);
}

fn read_int_from_file(reader:&mut BufReader<File>) -> Result<i32, String>
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

fn read_trimmed_line(reader: &mut BufReader<File>, buffer: &mut String) -> Result<usize, String>
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

fn read_line_from_file(reader: &mut BufReader<File>) -> Result<String, String>
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

fn get_reader() -> BufReader<File>
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

