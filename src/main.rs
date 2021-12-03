use std::{io::{stdin, BufRead, BufReader, Seek, SeekFrom, Write}, path::Path, fs::File, };


fn main() 
{
    let mut choice = String::new();

    print!("Which day's challenge to run (1-2): ");
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
        _ => {
            println!("Pick a real number next time.");
        }
    }
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
                frequency_calculator(result, &mut frequency)
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

    day_3_part_two(&mut reader, &frequency, &row_count);
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

fn day_3_part_two(reader: &mut BufReader<File>, frequency: &[i32], total: &i32)
{
    // let binary_string: String;
    // let mut most_common:[char; 12] = ['\x00';12];
    let mut most_common:u16 = 0;
    // let mut least_common:[char; 12] = ['\x00';12];
    let mut least_common:u16 = 0;

    // this problem is annoying.
    let mut o2:Vec<u16> = Vec::new();
    let mut co2:Vec<u16> = Vec::new();

    let mut index = 0;
    while index < frequency.len()
    {
        most_common <<= 1;
        least_common <<= 1;
        if frequency[index] > (total - frequency[index])
        {
            most_common += 1;
            // most_common[index] = '1';
            // least_common[index] = '0';
        }
        else if frequency[index] < (total - frequency[index])
        {
            least_common += 1;
            // most_common[index] = '0';
            // least_common[index] = '1';
        }
        index += 1;
    }

    println!("Most common pattern: {:b}", most_common);
    println!("Least common pattern: {:b}", least_common);

    // Preload the o2 and co2 vectors with bit strings that match the conditions
    // char[0] == most_common and char[0] == least_common respectively.
    let mut comparable_bit = 0x0800;
    loop 
    {
        let result = read_line_from_file(reader);
        match result
        {
            Ok(bit_string) =>
            {
                if bit_string == ""
                {
                    break;
                }
                let reading = bitstring_to_u16(bit_string).unwrap();
                if (reading & comparable_bit) == (most_common & comparable_bit)
                {
                    o2.push(reading);
                }
                else if (reading & comparable_bit) == (least_common & comparable_bit)
                {
                    co2.push(reading);
                }
            }
            Err(e) =>{
                println!("An error occurred reading the next line from the string.");
                return;
            }
        }
    }

    // We've already sorted the values into o2 and co2 according to the MSB.  Start with
    // second msb.
    
    loop 
    {
        comparable_bit >>= 1;
        
        if comparable_bit == 0
        {
            break;
        }

        let mut o2_index = 0;
        let mut co2_index = 0;

        while o2_index < o2.len()
        {
            if (o2[o2_index] & comparable_bit) != (most_common & comparable_bit)
            {
                o2.remove(o2_index);
            }
            else
            {
                o2_index += 1;
            }

        }

        while co2_index < co2.len()
        {
            if (co2[co2_index] & comparable_bit) != (least_common & comparable_bit)
            {
                co2.remove(co2_index);
            }
            else 
            {
                co2_index += 1;
            }
        }

        if o2.len() == 1
        {
            o2_index = 2;
        }
        else
        {
            o2_index = 0;
        }

        if co2.len() == 1
        {
            co2_index = 2;
        }
        else
        {
            co2_index = 0;
        }
    }

    println!("Well this was bonkers.");
    println!("O2 value: {}, {:b} in binary.", o2[0], o2[0]);
    println!("CO2 value: {}, {:b} in binary.", co2[0], co2[0]);
    println!("O2 x CO2: {}", o2[0] * co2[0]);

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

fn frequency_calculator(binary: String, counter: &mut [i32])
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

