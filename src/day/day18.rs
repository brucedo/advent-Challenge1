use std::{io::{BufReader, SeekFrom, Seek}, fs::File, str::Chars, fmt::{Display,Write},};

use log::debug;

use crate::common::common::{get_reader, read_trimmed_line, char_to_i8};

pub fn challenge_day_18()
{

    let mut reader = get_reader();

    part_one(&mut reader);

    let seek = reader.seek(SeekFrom::Start(0));

    match seek
    {
        Ok(bytes_moved) => {},
        Err(e) => {
            println!("Something went wrong moving the read head back to the start of the file.  Reloading file...");
            reader = get_reader();
        }
    }

    part_two(&mut reader);

}

fn part_one(reader: &mut BufReader<File>)
{
    let buffer = &mut String::new();
    let mut result = read_trimmed_line(reader, buffer);

    let mut snailfish_numbers = read_snailfish_numbers(reader);
    // let mut snailfish_numbers = Vec::<SnailfishNumber>::new();

    // loop
    // {
    //     match result
    //     {
    //         Ok(size) =>
    //         {
    //             if size == 0
    //             {
    //                 break;
    //             }

    //             let mut sfn_iter = buffer.chars();
    //             match build_snailfish_number(&mut sfn_iter)
    //             {
    //                 Ok(snf) =>
    //                 {
    //                     debug!("snailfish for pushing: {}", snf);
    //                     snailfish_numbers.push(snf);
    //                 }
    //                 Err(e) =>
    //                 {
    //                     panic!("Some string was not well formatted.  Poor snailfish shan't get its homework done. {}", e);
    //                 }
    //             }
    //             buffer.clear();
    //             result = read_trimmed_line(reader, buffer);
    //         }
    //         Err(e) =>
    //         {
    //             panic!("There was an error reading the file.  oops.  {}", e);
    //         }
    //     }
    // }

    let mut first = snailfish_numbers.remove(0);
    while !snailfish_numbers.is_empty()
    {
        first.add(snailfish_numbers.remove(0));
    }

    println!("Final sum: {}", first);
    println!("Magnitude: {}", first.magnitude());
}

fn part_two(reader: &mut BufReader<File>)
{
    let numbers = read_snailfish_numbers(reader);
    let mut max:u64 = 0;

    debug!("Total size of numbers: {}", numbers.len());

    for i in 0..numbers.len()
    {
        debug!("Hitting index i = {}", i);
        for j in 0..numbers.len()
        {
            debug!("Hitting index j = {}", j);
            if i == j
            {
                continue;
            }
            let mut a = numbers[i].duplicate();
            let b = numbers[j].duplicate();
            
            a.add(b);
            let mag = a.magnitude();
            debug!("Magnitude: {}", mag);
            max = std::cmp::max(max, mag);
        }
    }

    println!("Max magnitude: {}", max);
}

fn read_snailfish_numbers(reader: &mut BufReader<File>) -> Vec<SnailfishNumber>
{
    let buffer = &mut String::new();
    let mut result = read_trimmed_line(reader, buffer);

    let mut snailfish_numbers = Vec::<SnailfishNumber>::new();

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

                let mut sfn_iter = buffer.chars();
                match build_snailfish_number(&mut sfn_iter)
                {
                    Ok(snf) =>
                    {
                        debug!("snailfish for pushing: {}", snf);
                        snailfish_numbers.push(snf);
                    }
                    Err(e) =>
                    {
                        panic!("Some string was not well formatted.  Poor snailfish shan't get its homework done. {}", e);
                    }
                }
                buffer.clear();
                result = read_trimmed_line(reader, buffer);
            }
            Err(e) =>
            {
                panic!("There was an error reading the file.  oops.  {}", e);
            }
        }
    }

    return snailfish_numbers;
}

fn build_snailfish_number(buffer: &mut Chars) -> Result<SnailfishNumber, String>
{
    let mut left: Option<SnailfishNumber> = None;
    let mut right: Option<SnailfishNumber> = None;

    // left side first
    match buffer.next()
    {
        Some(char) =>
        {
            debug!("Matching char {}", char);
            match char
            {
                '[' => {left = Some(build_snailfish_number(buffer)?)},
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' =>
                {
                    let mut temp = SnailfishNumber::new();
                    temp.value = Some(char_to_i8(char) as u64);
                    return Ok(temp);
                }
                _ => {
                    debug!("Malformed char sequence - {} received, [ or some number expected.", char);
                    return Err(format!("Malformed character sequence - {} when either [ or some number was expected.", char).to_string());}

            }
        },
        None =>
        {
            return Err("Reached end of char stream before end of SNF.".to_string());
        }
    }

    // Next character should be a ','
    match buffer.next()
    {
        Some(char) =>
        {
            match char
            {
                ',' => {right = Some(build_snailfish_number(buffer)?)}
                _ => {return Err(format!("Malformed character sequence - {} when ',' was expected.", char).to_string());}
            }
        }
        None =>
        {
            return Err("Reached end of char stream before end of SNF.".to_string());
        }
    }

    // final removal - the ']'.
    match buffer.next()
    {
        Some (char) =>
        {
            match char
            {
                ']' => {debug!("Closed out set of SNFs.");}
                _ => {return Err(format!("Malformed character sequence - {} when ']' was expected.", char).to_string());}
            }
        }
        None =>
        {
            return Err("Reached end of char stream before end of SNF.".to_string());
        }
    }

    let mut result = SnailfishNumber::new();
    result.set_left(left);
    result.set_right(right);

    return Ok(result);
}


struct SnailfishNumber
{
    pub value: Option<u64>,
    pub left: Option<Box<SnailfishNumber>>,
    pub right: Option<Box<SnailfishNumber>>
}

impl SnailfishNumber {

    pub fn new() -> SnailfishNumber
    {
        SnailfishNumber {
            value: None,
            left: None,
            right: None
        }
    }

    pub fn duplicate(&self) -> SnailfishNumber
    {
        let mut new_me = SnailfishNumber::new();

        match self.value
        {
            Some(val) => 
            {
                debug!("Setting value for new_me from {}", val);
                new_me.value = Some(val.clone());
                debug!("new_me value is now {}", new_me.value.unwrap());
                
            }
            None => 
            {
                debug!("Branch node.  Hitting left.");
                new_me.set_left(Some(self.left.as_ref().unwrap().duplicate()));
                
                new_me.set_right(Some(self.right.as_ref().unwrap().duplicate()));
            }
        }

        return new_me;
    }

    pub fn add(&mut self, operand: SnailfishNumber)
    {
        let mut new_left = SnailfishNumber::new();

        new_left.value = self.value;
        new_left.left = self.left.take();
        new_left.right = self.right.take();

        self.set_left(Some(new_left));
        self.set_right(Some(operand));

        debug!("{}", self);

        self.reduce();
    }

    pub fn set_left(&mut self, left: Option<SnailfishNumber>)
    {
        match left
        {
            None => {self.left = None}
            Some(new_left) => {self.left = Some(Box::<SnailfishNumber>::new(new_left));}
        }
    }

    pub fn set_right(&mut self, right: Option<SnailfishNumber>)
    {
        match right
        {
            None => {self.right = None}
            Some(new_right) => {self.right = Some(Box::<SnailfishNumber>::new(new_right));}
        }
    }

    pub fn reduce(&mut self)
    {
        let mut action_taken = true;

        while action_taken
        {
            // debug!("Action taken last round:  true.");
            action_taken = SnailfishNumber::explode(self, 0).2 || self.split();
        }
        // debug!("Action taken last round: false.");
    }

    pub fn magnitude(&mut self) -> u64
    {
        match self.value
        {
            Some(num) => {num}
            None =>{3 * self.left.as_mut().unwrap().magnitude() + 2 * self.right.as_mut().unwrap().magnitude()}
        }
    }

    pub fn split(&mut self) -> bool
    {
        // debug!("Attempting split.");
        match self.value
        {
            Some(num) => {
                // debug!("In a leaf node - value is {}", num);
                if num > 9
                {
                    // debug!("Splitting this single leaf node in twain.");
                    let mut left_node = SnailfishNumber::new();
                    left_node.value = Some(num / 2);
                    // debug!("Left leaf node getting value {}", left_node.value.unwrap());
                    let mut right_node = SnailfishNumber::new();
                    right_node.value = Some((num / 2) + (num % 2));
                    // debug!("Right leaf node getting value {}", right_node.value.unwrap());
                    self.value = None;
                    self.set_left(Some(left_node));
                    self.set_right(Some(right_node));
                    // debug!("old lear node now transformed to branch.");
                    return true;
                }
            },
            None => {
                // debug!("Attempting to call split on either the left or the right, whichever comes first.");
                return self.left.as_mut().unwrap().split() || self.right.as_mut().unwrap().split();
            }
        }

        return false;
    }

    pub fn explode(node: &mut  SnailfishNumber, depth: usize) -> (Option<u64>, Option<u64>, bool)
    {
        // debug!("Explode called.");
        // base case
        if node.left.as_ref().unwrap().value.is_some() && node.right.as_ref().unwrap().value.is_some()
        {
            // debug!("Base case - this is a branch node with two child nodes that are leaves");
            if depth >= 4
            {
                // debug!("Our depth is at least 4.");
                let val = 
                    (Some(node.left.as_ref().unwrap().value.unwrap()), 
                    Some(node.right.as_ref().unwrap().value.unwrap()),
                    true);
                // debug!("left and right values: {}, {}", val.0.unwrap(), val.1.unwrap());
                node.left = None;
                node.right = None;
                node.value = Some(0);
                return val;
            }
            // if we hit a leaf node but are not nested 4 deep - good.  Just return.
            else
            {
                // debug!("Higher level leaf node - depth: {}", depth);
                return (None, None, false);
            }
        }

        if node.left.is_some() && node.left.as_ref().unwrap().value.is_none()
        {
            // debug!("Left child exists and is not a leaf node - check subtree for explosions.");
            // Test the left branch for explodeyness
            let result = SnailfishNumber::explode(node.left.as_mut().unwrap(), depth + 1);

            // debug!("Subtree explosion status: {}", result.2);
            
            // if the left branch indicated it exploded, we need to try to consume the left and right values of the explosion
            if result.2
            {
                // debug!("I coulda just put output here to indicate splosion but hey.");
                // Since we already received an explosion from our left side, we cannot re-consume the left value from that same
                // side.  But we can consume the right value down our right branch IF it has not already been consumed.
                let val2 = match result.1
                {
                    Some(num) => {SnailfishNumber::left_consume(node.right.as_mut().unwrap(), num)}
                    None => {None}
                };
                
                // Note the return - if we have caught an explosion, we do not want to keep searching for explosions.  So we back
                // out here.
                return (result.0, val2, result.2);
            }
            // debug!("Left side had no explosions.");
        }

        if node.right.is_some() && node.right.as_ref().unwrap().value.is_none()
        {
            // debug!("Right child exists and is not a leaf node - check subtree for explosions.");
            let result = SnailfishNumber::explode(node.right.as_mut().unwrap().as_mut(), depth + 1);
            // Same deal as for left, but sides swapped.
            if result.2
            {
                // debug!("Right side had explosions.");
                let val2 = match result.0
                {
                    Some(num) => {SnailfishNumber::right_consume(node.left.as_mut().unwrap(), num)},
                    None => {None}
                };

                return (val2, result.1, result.2);
            }
        }

        return (None, None, false);
    }

    pub fn left_consume(node: &mut  SnailfishNumber, value: u64) -> Option<u64>
    {

        if node.value.is_none()
        {
            if node.left.is_none()
            {
                panic!("Node with no left or no value.  Malformed tree.");
            }
            else
            {
                return SnailfishNumber::left_consume(node.left.as_mut().unwrap().as_mut(), value);
            }
        }
        else
        {
            let temp = node.value.unwrap();
            node.value = Some(temp + value);
            return None;
        }

        // return Some(0);
    }

    pub fn right_consume(node: &mut SnailfishNumber, value: u64) -> Option<u64>
    {
        // debug!("Attempting to apply value {} to the right side of this subtree.", value);
        match node.value
        {
            Some(node_value) =>
            {
                // debug!("This node has a value: {}", node_value);
                node.value = Some(node_value + value);
                // debug!("The node's value is now {}", node.value.unwrap());
                return None;
            }
            None => 
            {
                match &node.right
                {
                    None => {panic!("Node with no right or no value.  Malformed tree.");}
                    Some(snf) => 
                    {
                        return SnailfishNumber::right_consume(node.right.as_mut().unwrap().as_mut(), value);
                    }
                }
            }
        }
    }

    fn build_output(&self) -> String
    {
        let mut form = String::new();
        match self.value
        {
            Some(num) => {write!(form, "{}", num);}
            None => {
                write!(form, "[{},{}]", self.left.as_ref().unwrap().build_output(), self.right.as_ref().unwrap().build_output());
            }
        }

        return form;
    }
}

impl Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build_output())
    }
}

#[cfg(test)]
pub mod tests
{
    use log::debug;

    use super::{SnailfishNumber, build_snailfish_number};

    fn init()
    {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    pub fn test_build_snf()
    {
        init();

        let mut a = "[3,2]".chars();
        let mut snf: SnailfishNumber;
        let mut left: Box<SnailfishNumber>;
        let mut right: Box<SnailfishNumber>;

        let mut result = build_snailfish_number(&mut a);
        assert!(result.is_ok());
        snf = result.unwrap();
        assert!(snf.left.is_some());
        assert!(snf.right.is_some());
        left = snf.left.unwrap();
        assert!(left.value.is_some());
        assert_eq!(3,left.value.unwrap());
        right = snf.right.unwrap();
        assert!(right.value.is_some());
        assert_eq!(2, right.value.unwrap());

        a = "[3,[3,2]]".chars();
        result = build_snailfish_number(&mut a);
        assert!(result.is_ok());
        snf = result.unwrap();
        assert!(snf.left.is_some());
        assert!(snf.right.is_some());
        left = snf.left.unwrap();
        assert!(left.value.is_some());
        assert_eq!(3, left.value.unwrap());
        right = snf.right.unwrap();
        assert!(right.value.is_none());
        assert!(right.left.is_some());
        assert!(right.right.is_some());

        left = right.left.unwrap();
        right = right.right.unwrap();
        assert!(left.value.is_some());
        assert!(right.value.is_some());
        assert_eq!(3, left.value.unwrap());
        assert_eq!(2, right.value.unwrap());

        a = "[[2,3],3]".chars();
        result = build_snailfish_number(&mut a);
        assert!(result.is_ok());
        snf = result.unwrap();
        assert!(snf.left.is_some());
        assert!(snf.right.is_some());

        left = snf.left.unwrap();
        right = snf.right.unwrap();

        assert!(left.value.is_none());
        assert!(right.value.is_some());
        assert_eq!(3, right.value.unwrap());
        assert!(left.left.is_some());
        assert!(left.right.is_some());
        
        right = left.right.unwrap();
        left = left.left.unwrap();

        assert!(left.value.is_some());
        assert!(right.value.is_some());
        assert_eq!(2, left.value.unwrap());
        assert_eq!(3, right.value.unwrap());
    }

    #[test]
    pub fn test_duplicate()
    {
        init();

        let mut a = "[9,1]".chars();
        let mut snf = build_snailfish_number(&mut a).unwrap();
        let mut snf_copy = snf.duplicate();

        debug!("{}\n{}", snf, snf_copy);
    }

    #[test]
    pub fn test_add()
    {
        init();

        let mut a = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]".chars();
        let mut b = "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]".chars();
        let mut c = "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]".chars();
        let mut d = "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]".chars();
        let mut e = "[7,[5,[[3,8],[1,4]]]]".chars();
        let mut f = "[[2,[2,2]],[8,[8,1]]]".chars();
        let mut g = "[2,9]".chars();
        let mut h = "[1,[[[9,3],9],[[9,0],[0,7]]]]".chars();
        let mut i = "[[[5,[7,4]],7],1]".chars();
        let mut j = "[[[[4,2],2],6],[8,7]]".chars();

        let mut snf = build_snailfish_number(&mut a).unwrap();
        debug!("{}", snf);
        snf.add(build_snailfish_number(&mut b).unwrap());
        debug!("{}", snf);
        snf.add(build_snailfish_number(&mut c).unwrap());
        debug!("{}", snf);
        snf.add(build_snailfish_number(&mut d).unwrap());
        debug!("{}", snf);
        snf.add(build_snailfish_number(&mut e).unwrap());
        debug!("{}", snf);
        snf.add(build_snailfish_number(&mut f).unwrap());
        debug!("{}", snf);
        snf.add(build_snailfish_number(&mut g).unwrap());
        debug!("{}", snf);
        snf.add(build_snailfish_number(&mut h).unwrap());
        debug!("{}", snf);
        snf.add(build_snailfish_number(&mut i).unwrap());
        debug!("{}", snf);
        snf.add(build_snailfish_number(&mut j).unwrap());

        debug!("{}", snf);
    }

    #[test]
    pub fn test_magnitude()
    {
        let mut a = "[9,1]".chars();
        let mut snf = build_snailfish_number(&mut a).unwrap();
        assert_eq!(29, snf.magnitude());

        a = "[1,9]".chars();
        snf = build_snailfish_number(&mut a).unwrap();
        assert_eq!(21, snf.magnitude());

        a = "[[9,1],[1,9]]".chars();
        snf = build_snailfish_number(&mut a).unwrap();
        assert_eq!(129, snf.magnitude());

        a = "[[1,2],[[3,4],5]]".chars();
        snf = build_snailfish_number(&mut a).unwrap();
        assert_eq!(143, snf.magnitude());

        a = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".chars();
        snf = build_snailfish_number(&mut a).unwrap();
        assert_eq!(1384, snf.magnitude());

        a = "[[[[1,1],[2,2]],[3,3]],[4,4]]".chars();
        snf = build_snailfish_number(&mut a).unwrap();
        assert_eq!(445, snf.magnitude());

        a = "[[[[3,0],[5,3]],[4,4]],[5,5]]".chars();
        snf = build_snailfish_number(&mut a).unwrap();
        assert_eq!(791, snf.magnitude());

        a = "[[[[5,0],[7,4]],[5,5]],[6,6]]".chars();
        snf = build_snailfish_number(&mut a).unwrap();
        assert_eq!(1137, snf.magnitude());

        a = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".chars();
        snf = build_snailfish_number(&mut a).unwrap();
        assert_eq!(3488, snf.magnitude());
    }

    #[test]
    pub fn test_split()
    {
        init();

        let mut a = "[3,2]".chars();
        let mut snf = build_snailfish_number(&mut a).unwrap();

        let result = snf.split();
        assert!(!result);
        assert!(snf.left.is_some());
        assert!(snf.right.is_some());
        assert!(snf.left.as_ref().unwrap().value.is_some());
        assert_eq!(3, snf.left.as_ref().unwrap().value.unwrap());
        assert!(snf.right.as_ref().unwrap().value.is_some());
        assert_eq!(2, snf.right.as_ref().unwrap().value.unwrap());

        a = "[3,2]".chars();
        let mut snf = build_snailfish_number(&mut a).unwrap();
        SnailfishNumber::left_consume(&mut snf, 10);

        let result = snf.split();
        assert!(result);
        assert!(snf.left.is_some());
        assert!(snf.right.is_some());
        assert!(snf.left.as_ref().unwrap().value.is_none());
        assert!(snf.left.as_ref().unwrap().left.is_some());
        assert!(snf.left.as_ref().unwrap().left.as_ref().unwrap().value.is_some());
        assert_eq!(6, snf.left.as_ref().unwrap().left.as_ref().unwrap().value.unwrap());
        assert_eq!(7, snf.left.as_ref().unwrap().right.as_ref().unwrap().value.unwrap());

        debug!("{}", snf);
    }

    #[test]
    pub fn test_reduce()
    {
        init();

        // let mut a = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".chars();
        // let mut snf = build_snailfish_number(&mut a).unwrap();

        // debug!("{}", snf);
        // snf.reduce();
        // debug!("{}", snf);

        let mut a = "[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],0]".chars();
        let mut snf = build_snailfish_number(&mut a).unwrap();
        let result = snf.reduce();
        debug!("{}", snf);
    }

    #[test]
    pub fn test_explode()
    {
        init();

        let mut a = "[3,2]".chars();
        let mut snf = build_snailfish_number(&mut a).unwrap();

        let result = SnailfishNumber::explode(&mut snf, 5);
        assert!(result.2);
        assert_eq!(0, snf.value.unwrap());

        a = "[[3,2],2]".chars();
        snf = build_snailfish_number(&mut a).unwrap();

        let result = SnailfishNumber::explode(&mut snf, 4);
        assert!(result.2);
        assert!(snf.left.as_ref().unwrap().value.is_some());
        assert_eq!(0, snf.left.as_ref().unwrap().value.unwrap());
        assert!(snf.right.as_ref().unwrap().value.is_some());
        assert_eq!(4, snf.right.as_ref().unwrap().value.unwrap());

        a = "[3,[2,2]]".chars();
        snf = build_snailfish_number(&mut a).unwrap();
        let result = SnailfishNumber::explode(&mut snf, 4);
        assert!(result.2);
        assert!(snf.left.as_ref().unwrap().value.is_some());
        assert_eq!(5, snf.left.as_ref().unwrap().value.unwrap());
        assert!(snf.right.as_ref().unwrap().value.is_some());
        assert_eq!(0, snf.right.as_ref().unwrap().value.unwrap());

        a = "[[2,[4,4]],[3,3]]".chars();
        snf = build_snailfish_number(&mut a).unwrap();
        let result = SnailfishNumber::explode(&mut snf, 3);
        assert!(result.2);
        assert!(snf.left.as_ref().unwrap().value.is_none());
        assert!(snf.right.as_ref().unwrap().value.is_none());
        assert!(snf.left.as_ref().unwrap().left.as_ref().is_some());
        assert!(snf.left.as_ref().unwrap().right.as_ref().is_some());

        a = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".chars();
        snf = build_snailfish_number(&mut a).unwrap();
        let result = SnailfishNumber::explode(&mut snf, 0);
        debug!("{}", snf);


    }

    #[test]
    pub fn test_consume()
    {
        let mut a = "[3,2]".chars();
        let mut snf = build_snailfish_number(&mut a).unwrap();

        SnailfishNumber::left_consume(&mut snf, 3);
        assert!(snf.left.is_some());
        assert!(snf.right.is_some());
        assert!(snf.left.as_ref().unwrap().value.is_some());
        assert_eq!(6, snf.left.as_ref().unwrap().value.unwrap());

        a = "[3,[3,2]]".chars();
        snf = build_snailfish_number(&mut a).unwrap();

        SnailfishNumber::left_consume(&mut snf, 4);
        assert!(snf.left.is_some());
        assert!(snf.right.is_some());
        assert!(snf.left.as_ref().unwrap().value.is_some());
        assert_eq!(7, snf.left.as_ref().unwrap().value.unwrap());

        a = "[[3,2],3]".chars();
        snf = build_snailfish_number(&mut a).unwrap();

        SnailfishNumber::left_consume(&mut snf, 5);
        assert!(snf.left.is_some());
        assert!(snf.right.is_some());
        assert!(snf.left.as_ref().unwrap().left.is_some());
        assert!(snf.left.as_ref().unwrap().left.as_ref().unwrap().value.is_some());
        assert_eq!(8, snf.left.as_ref().unwrap().left.as_ref().unwrap().value.unwrap());
    }

}