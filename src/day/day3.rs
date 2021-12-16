use std::{io::{SeekFrom, Seek, BufReader}, fs::File};

use crate::common::common::{get_reader, read_line_from_file, frequency_calculator_bitstring, bitstring_to_u16, calc_most_common_bit, calc_least_common_bit};




pub fn challenge_day_three()
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

    day_3_part_one(&frequency, row_count);

    // rewind for task two
    seek = reader.seek(SeekFrom::Start(0));
    if seek.is_err()
    {
        println!("Problem seeking to the start of the file stream.  Reopening file:");
        reader = get_reader();
    }

    let _ = day_3_part_two(&mut reader);
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

fn day_3_part_one(frequency: &[i32], row_count: i32)
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