use std::{io::BufReader, fs::File, collections::{HashMap, HashSet}, };

use log::debug;

use crate::common::common::{get_reader, read_trimmed_line};

pub fn challenge_day_17()
{
    let mut reader = get_reader();

    let bounding_box = get_target_area(&mut reader);

    part_one(bounding_box);
    part_two(bounding_box);
}

fn part_two(bounding_box:((i32, i32), (i32, i32)))
{
    // Part two - we need to find all possible initial velocities that will get us into the box.  Now, while we know what the
    // maximum velocity can be, we cannot simply say that it's all possible values between 0 and max_depth.  If we have a value
    // y such that y = max_y - 1 but max_y - min_y < y, then when the next velocity change of y_new = y + (y+1) occurs, we will
    // jump right over that box.  So - now we actually DO have to simulate the possible y values, we just don't have to go to
    // more than max_depth.
    let x_pair = bounding_box.0;
    let y_pair = bounding_box.1;



    simulate_all(x_pair.0, x_pair.1, y_pair.0, y_pair.1);

}

fn simulate_all(x_min: i32, x_max: i32, y_min: i32, y_max: i32)
{
    let max_init_x_velocity = x_max.abs() + 1;
    let mut x_inits = Vec::<i32>::new();

    let max_init_y_velocity = y_min.abs() + 1;
    let mut y_inits = Vec::<i32>::new();

    let mut good_shots = HashSet::<(i32, i32)>::new();

    for x_velocity in 1 .. (max_init_x_velocity + 1)
    {
        x_inits.push(x_velocity);
        x_inits.push(-x_velocity);
    }
    x_inits.push(0);

    debug!("Starting simulation of y???");


    for velocity in 1 .. (max_init_y_velocity + 1)
    {
        y_inits.push(velocity);
        y_inits.push(-velocity);
    }
    // no point in doubling up the zero
    y_inits.push(0);

    for x_start in x_inits
    {
        for y_start in &y_inits
        {
            debug!("Starting with initial velocities {}, {}", x_start, y_start);
            let mut x_pos = 0;
            let mut y_pos = 0;
            let mut x_vel = x_start;
            let mut y_vel = *y_start;

            debug!("x_max: {}, y_min: {}", x_max, y_min);
            
            while x_pos <= x_max && y_pos >= y_min
            {
                debug!("Testing position {} > {}, {} < {}",x_pos, x_min, y_pos, y_max);
                if x_pos >= x_min && y_pos <= y_max
                {
                    debug!("position inside of box {}-{}, {}-{}", x_min, x_max, y_max, y_min);
                    good_shots.insert((x_start, *y_start));
                    break;
                }
                
                x_pos += x_vel;
                y_pos += y_vel;
                y_vel -= 1;
                match x_vel.cmp(&0)
                {
                    std::cmp::Ordering::Less => x_vel += 1,
                    std::cmp::Ordering::Equal => {},
                    std::cmp::Ordering::Greater => x_vel -=1,
                }
            }
        }
    }

    debug!("set: {:?}", good_shots);
    println!("Well if this worked, then the total number of initial velocities that hits the target is {}", good_shots.len());
}

// x is calculated a bit differently since our x's are all positive.  x_max 
fn simulate_x(x_min: i32, x_max: i32) -> HashMap<i32, Vec<i32>>
{
    let mut x_velocities = HashMap::<i32, Vec<i32>>::new();

    debug!("Starting simulation of x.");

    let max_init_velocity = (x_max.abs()) + 1;
    let mut x_inits = Vec::<i32>::new();

    for velocity in 1 .. (max_init_velocity + 1)
    {
        x_inits.push(velocity);
        x_inits.push(-velocity);
    }
    x_inits.push(0);

    for starting_velocity in x_inits
    {
        let mut position = starting_velocity;
        let mut active_velocity = starting_velocity;
        debug!("Testing start position {}", position);
        let mut c = 1;
        while position <= x_max
        {
            debug!("Testing positive_velocity: {}", position);
            if position >= x_min
            {
                debug!("Positive velocity >= x_min: {} <= {}", position, &x_min);
                match x_velocities.entry(c)
                {
                    std::collections::hash_map::Entry::Occupied(mut entry) => 
                    {
                        let vec = entry.get_mut();
                        vec.push(starting_velocity);
                    },
                    std::collections::hash_map::Entry::Vacant(entry) => 
                    {
                        let mut vec = Vec::<i32>::new();
                        vec.push(starting_velocity);
                        entry.insert(vec);
                    },
                }
            }
            c += 1;
            match active_velocity.cmp(&0)
            {
                std::cmp::Ordering::Less => active_velocity += 1,
                std::cmp::Ordering::Equal => {break},
                std::cmp::Ordering::Greater => active_velocity -= 1,
            }
            position += active_velocity;
        }
    }


    return x_velocities;
}

// y_min is the value of y that is smallest, or most negative.  y_max is the value of y that is largest, or least negative.
fn simulate_y(y_min: i32, y_max: i32) -> HashMap<i32, Vec<i32>>
{
    let mut y_velocities = HashMap::<i32, Vec<i32>>::new();

    debug!("Starting simulation of y???");

    let max_init_velocity = y_min.abs() + 1;
    let mut y_inits = Vec::<i32>::new();

    for velocity in 1 .. (max_init_velocity + 1)
    {
        y_inits.push(velocity);
        y_inits.push(-velocity);
    }
    // no point in doubling up the zero
    y_inits.push(0);

    for starting_velocity in y_inits
    {
        // We should probably test both positive and negative variants of velos.
        let mut position = starting_velocity;
        let mut active_velocity = starting_velocity;
        debug!("Testing start position {}", position);
        let mut c= 1;
        debug!("On step {}", c);
        while position >= y_min
        {
            debug!("Testing positive_velocity: {}", position);
            if position <= y_max
            {
                debug!("Positive velocity <= y_max: {} <= {}", position, &y_max);
                match y_velocities.entry(c)
                {
                    std::collections::hash_map::Entry::Occupied(mut entry) => 
                    {
                        let vec = entry.get_mut();
                        vec.push(starting_velocity);
                    },
                    std::collections::hash_map::Entry::Vacant(entry) => 
                    {
                        let mut vec = Vec::<i32>::new();
                        vec.push(starting_velocity);
                        entry.insert(vec);
                    },
                }
            }
            c += 1;
            active_velocity -= 1;
            position += active_velocity;
        }
    }

    debug!("Finished map: {:?}", y_velocities);

    return y_velocities;
}

fn part_one(bounding_box: ((i32, i32), (i32, i32)))
{
    // Calculate the maximum depth of the bounding box
    let y_pair = bounding_box.1;
    let min_depth = y_pair.0;
    let mut height: i32 = 0;

    debug!("bounding_box: {:?}", bounding_box);

    // that was easy.

    // The y axis arc of the probe is symmetric - we decelerate at a constant rate, and the increments going down from k to 0
    // are exactly the same as the increments from 0 to k - meaning we are guarateed to hit the 0 axis on the dowstroke, since
    // we started on it.

    // Therefore, the maximum speed that the probe can be going after it hits the 0 axis is going to be the maximum_depth of
    // the bounding box - otherwise the probe will just skip straight through it.  Therefore, the maximum speed we can launch
    // the probe with in the y axis is one _less_ than that depth (since it gains one unit of acceleration after it is set at
    // axis 0).  Therefore, our maximum upwards speed is abs(max_depth) - 1.
    let max_y_velocity = min_depth.abs() - 1;
    

    // and our maximum height is simply the sum from max_y_velocity to zero.
    for speed in 0 .. max_y_velocity + 1
    {
        height += speed;
    }

    println!("Our max height is: {}", height);
}


pub fn get_target_area( reader: &mut BufReader<File>) -> ((i32, i32), (i32, i32))
{
    // This challenge is in a single line
    let mut buffer = String::new();


    let read_result = read_trimmed_line(reader, &mut buffer);

    match read_result
    {
        Ok(_) =>
        {
            build_target_pairs(buffer)
        },
        Err(e) =>
        {
            panic!("File read has failed: {}", e);
        }
    }

    
}

fn build_target_pairs(input_string: String) -> ((i32, i32), (i32, i32))
{
    let mut x_min: i32 = 0;
    let mut x_max: i32 = 0;
    let mut y_min: i32 = 0;
    let mut y_max: i32 = 0;

    match input_string.strip_prefix("target area: ")
    {
        Some(remainder) => 
        {
            // remainder should be of the form 'x=###..###, y=###..###
            for coord_string in remainder.split(", ")
            {
                // Now down to "x|y=###..###"
                if coord_string.starts_with("x=")
                {
                    match coord_string.strip_prefix("x=")
                    {
                        Some(left_and_right) =>
                        {
                            // ###..###
                            let mut temp = Vec::<i32>::new();
                            for coord in left_and_right.split("..")
                            {
                                temp.push(coord.parse::<i32>().unwrap());
                            }
                            x_max = std::cmp::max(temp[0], temp[1]);
                            x_min = std::cmp::min(temp[0], temp[1]);
                        }
                        None =>
                        {
                            panic!("This is a malformed input string - are you sure you picked the right input file?");
                        }
                    }
                }
                else 
                {
                    match coord_string.strip_prefix("y=")
                    {
                        Some(left_and_right) =>
                        {
                            // ###..###
                            let mut temp = Vec::<i32>::new();
                            for coord in left_and_right.split("..")
                            {
                                temp.push(coord.parse::<i32>().unwrap());
                            }
                            y_max = std::cmp::max(temp[0], temp[1]);
                            y_min = std::cmp::min(temp[0], temp[1]);
                        }
                        None =>
                        {
                            panic!("This is a malformed input string - are you sure you picked the right input file?");
                        }
                    }
                }
            }
        },
        None => 
        {
            panic!("This is a malformed input string - are you sure you picked the right input file?");
        },
    }


    return ((x_min, x_max), (y_min, y_max));
}


#[cfg(test)]
pub mod tests
{
    use log::debug;

    use crate::day::day11::part_one;

    use super::{build_target_pairs, simulate_y, simulate_all};

    fn init()
    {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    pub fn test_simulate_y()
    {
        init();

        let mut min_y = -2;
        let mut max_y = -1;

        let mut y_velocities = simulate_y(min_y, max_y);
        assert_eq!(3, y_velocities.len());

        let mut min_y = -10;
        let mut max_y = -5;
        y_velocities = simulate_y(min_y, max_y);
    }

    #[test]
    pub fn test_simulate_all()
    {
        init();

        let min_x = 20;
        let max_x = 30;
        let min_y = -10;
        let max_y = -5;

        simulate_all(min_x, max_x, min_y, max_y);
    }

    #[test]
    pub fn test_build_target_pairs()
    {
        init();

        let example_input = "target area: x=20..30, y=-10..-5".to_string();

        let bounding_box = build_target_pairs(example_input);

        assert_eq!(20, bounding_box.0.0);
        assert_eq!(30, bounding_box.0.1);
        assert_eq!(-10, bounding_box.1.0);
        assert_eq!(-5, bounding_box.1.1);

        let real_input = "target area: x=57..116, y=-198..-148".to_string();

        let real_bounding_box = build_target_pairs(real_input);
        assert_eq!(57, real_bounding_box.0.0);
        assert_eq!(116, real_bounding_box.0.1);
        assert_eq!(-198, real_bounding_box.1.0);
        assert_eq!(-148, real_bounding_box.1.1);
    }
}