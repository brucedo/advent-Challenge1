use std::{cmp, };

use log::{debug, trace};

use crate::common::common::{get_reader, read_trimmed_line};

pub fn challenge_day_16()
{
    let mut reader = get_reader();
    let mut buffer = String::new();

    let result = read_trimmed_line(&mut reader, &mut buffer);
    match result
    {
        Ok(_) =>
        {
            part_one(buffer.clone());
            part_two(buffer.clone());
        }
        Err(e)=>
        {
            panic!("Unable to read the contents of the file: {}", e);
        }
    }
}

fn part_one(packet_buffer: String)
{

    let packet = packet_state_machine(packet_buffer);

    let version_sum = version_sum(packet);

    println!("Holy effin' poopsnakes.  Finally got there.  {}", version_sum);
}

fn part_two(packet_buffer: String)
{
    let packet = packet_state_machine(packet_buffer);
    let total = evaluate_packet(packet);

    println!("Your total packet value is {}", total);
}

fn version_sum(packet: Packet) -> u64
{
    let mut stack = Vec::<Packet>::new();
    let mut version: u64 = 0;

    stack.push(packet);

    while !stack.is_empty()
    {
        let mut temp = stack.pop().unwrap();
        version += temp.version as u64;
        if temp.nested.len() > 0
        {
            stack.append(&mut temp.nested);
        }
    }

    return version;
}

fn evaluate_packet(mut packet: Packet) -> u64
{
    debug!("Evaluation for packet with type {}", packet.packet_type_id);
    debug!("Size of child list: {}", packet.nested.len());
    match packet.packet_type
    {
        PacketType::OperatorSum => 
        {
            let mut return_value: u64 = 0;
            for child in packet.nested
            {
                return_value += evaluate_packet(child);
            }
            return return_value;
        },
        PacketType::OperatorProduct => 
        {
            let mut return_value: u64 = 1;
            for child in packet.nested
            {
                return_value *= evaluate_packet(child);
            }

            return return_value;
        },
        PacketType::OperatorMinimum => 
        {
            let mut min: u64 = u64::MAX;
            for child in packet.nested
            {
                min = std::cmp::min(min, evaluate_packet(child));
            }

            return min;
        },
        PacketType::OperatorMaximum => 
        {
            let mut max: u64 = 0;
            for child in packet.nested
            {
                max = std::cmp::max(max, evaluate_packet(child));
            }

            return max;
        },
        PacketType::OperatorGreaterThan => 
        {
            match evaluate_packet(packet.nested.pop().unwrap()) < evaluate_packet(packet.nested.pop().unwrap())
            {
                true =>{1},
                false=>{0}
            }
        },
        PacketType::OperatorLessThan => 
        {
            match evaluate_packet(packet.nested.pop().unwrap()) > evaluate_packet(packet.nested.pop().unwrap())
            {
                true => {1}
                false => {0}
            }
        },
        PacketType::OperatorEqualTo => 
        {
            match evaluate_packet(packet.nested.pop().unwrap()) == evaluate_packet(packet.nested.pop().unwrap())
            {
                true => {1}
                false => {0}
            }
        },
        PacketType::Literal => {debug!("This is a literal packet, value of {}", packet.value);packet.value},
    }
}

fn packet_state_machine(packet_buffer: String) -> Packet
{
    let mut current_segment = ParsingSegment::Version;

    let mut bit_stream = BitStream::new(packet_buffer);
    let mut current_packet: Packet = Packet::new();
    let mut siblings = Vec::<Packet>::new();
    let mut packet_stack = Vec::<Vec<Packet>>::new();
    let mut pair_count: (usize, usize) = (0, 0);
    let mut pair_stack = Vec::<(usize, usize)>::new();

    debug!("bit_stream remaining: {}", bit_stream.remaining());
    debug!("bit_stream len: {}", bit_stream.len());
    debug!("bit_stream position: {}", bit_stream.stream_pos());

    // hooo boy.
    // while bit_stream.remaining() > 0
    loop
    {
        
        debug!("Next pass...");

        match current_segment
        {
            ParsingSegment::Version =>
            {
                debug!("Starting packet!");
                // current_packet = Packet::new();
                match bit_stream.mask_into_u8(3)
                {
                    Some(triad) => {
                        current_packet.version = triad;
                        debug!("Current packet version: {}", current_packet.version);
                    }
                    None => {
                        debug!("No remaining bits in the stream.  possibly an error, or maybe we've tapped the end-of-stream zeros.");
                        break;
                    }
                }
                current_segment = ParsingSegment::Type
            }
            ParsingSegment::Type =>
            {
                debug!("Decoding type bits.");
                match bit_stream.mask_into_u8(3)
                {
                    Some(triad) => 
                    {
                        current_packet.packet_type_id = triad;
                        current_packet.packet_type = match triad 
                        {
                            4 => {debug!("Literal!");PacketType::Literal}, 
                            0 => {debug!("OperatorSum!");PacketType::OperatorSum}
                            1 => {debug!("OperatorProduct!");PacketType::OperatorProduct}
                            2 => {debug!("OperatorMinimum!");PacketType::OperatorMinimum}
                            3 => {debug!("OperatorMaximum!");PacketType::OperatorMaximum}
                            5 => {debug!("OperatorGreaterThan!");PacketType::OperatorGreaterThan}
                            6 => {debug!("OperatorLessThan!");PacketType::OperatorLessThan}
                            7 => {debug!("OperatorEqualTo!");PacketType::OperatorEqualTo}
                            _ => {panic!("Unrecognized operator type {}", triad);}
                        };
                        current_segment = match current_packet.packet_type {PacketType::Literal=>{ParsingSegment::Literal}, _=>{ParsingSegment::Operator}};
                    }
                    None =>
                    {
                        if current_packet.version == 0
                        {
                            debug!("Zero value captured in version and out of bits - end of stream zeros?");
                            break;
                        }
                        debug!("No remaining bits in the stream.  Definitely an error.");

                    }
                }
                
            }
            ParsingSegment::Literal =>
            {
                let mut temp = match bit_stream.mask_into_u8(5)
                {
                    Some(value) => {value},
                    None => {
                        if current_packet.version == 0 && current_packet.packet_type_id == 0
                        {
                            debug!("Possibly just working through zero-value end of stream?");
                            break;
                        }
                        panic!("Yup, serious problems with this here bit stream...");}
                };

                while temp & 0b10000 > 0
                {
                    current_packet.value <<= 4;
                    current_packet.value += (temp & 0b01111) as u64;
                    temp = match bit_stream.mask_into_u8(5)
                    {
                        Some(value) => {value},
                        None => {panic!("Yup, serious problems with this here bit stream...");}
                    };
                }

                current_packet.value <<= 4;
                current_packet.value += (temp & 0b01111) as u64;
                debug!("current_packet value: {}", current_packet.value);

                current_segment = ParsingSegment::Complete;
                
            }
            ParsingSegment::Operator =>
            {
                let length_type = match bit_stream.mask_into_u8(1)
                {
                    Some(value)=>{value},None=>{panic!("We've got a broken input string here.");}
                };

                debug!("Pushing pair count {:?}", pair_count);
                pair_stack.push(pair_count);
                pair_count = (0, 0);

                if length_type == 0
                {
                    debug!("Length type is bits");
                    current_segment = ParsingSegment::LengthTypeBits
                }
                else
                {
                    debug!("Length type is count of packets.");
                    current_segment = ParsingSegment::LengthTypeCount
                }
            }
            ParsingSegment::LengthTypeBits =>
            {
                let bit_count = match bit_stream.mask_into_u16(15)
                {
                    Some(value)=>{value},None=>{panic!("broken input bitstream when expecting 15 bits of length data.");}
                };

                debug!("expect {} bits of subpackets.", bit_count);
                // Rather than store the number of bits of sub-packets, we will store the value that is bit_stream.position + 
                // bit_count - that is, the position in the bit stream that the pointer should be at or past when we have
                // finally read in all the data for this packet's subpackets.
                pair_count.0 = bit_stream.stream_pos() + bit_count as usize;
                debug!("Pair count now: {:?}", pair_count);
                siblings.push(current_packet);
                packet_stack.push(siblings);
                
                current_packet = Packet::new();
                siblings = Vec::<Packet>::new();
                current_segment = ParsingSegment::Version;
            }
            ParsingSegment::LengthTypeCount =>
            {
                let packet_count = match bit_stream.mask_into_u16( 11)
                {
                    Some(value)=>{value},None=>{panic!("broken input bitstream when expecting 11 bits of count data.");}
                };
                debug!("Expect {} subpackets.", packet_count);

                pair_count.1 = packet_count as usize;
                debug!("Pair count now: {:?}", pair_count);
                siblings.push(current_packet);
                packet_stack.push(siblings);
                
                current_packet = Packet::new();
                siblings = Vec::<Packet>::new();
                current_segment = ParsingSegment::Version;
            }
            ParsingSegment::Complete =>
            {
                siblings.push(current_packet);
                debug!("Sibling count after adding most recent complete: {}", siblings.len());
                
                current_packet = Packet::new(); 

                if pair_count.1 > 0
                {
                    pair_count.1 -= 1;
                }

                current_segment = ParsingSegment::TestSiblings;
            }
            ParsingSegment::TestSiblings =>
            {
                if pair_count == (0, 0) || (pair_count.0 > 0 && bit_stream.stream_pos() >= pair_count.0)
                {
                    let mut temp = match packet_stack.pop()
                    {
                        None => {debug!("Nothing left on packet stack, no siblings to fill - done!"); break;}
                        Some(value) => {value}
                    };
                    pair_count = pair_stack.pop().unwrap();
                    current_packet = temp.pop().unwrap();
                    current_packet.nested = siblings;
                    siblings = temp;
                    current_segment = ParsingSegment::Complete;
                }
                else
                {
                    debug!("Not done yet, more siblings to discover...{:?}", pair_count);
                    current_segment = ParsingSegment::Version;
                }
            }
        }

        debug!("Bits left? {}", bit_stream.remaining());
    }

    debug!("Number of siblings in the list should be 1: {}", siblings.len());
    debug!("Number of children of the main sibling: {}", siblings[0].nested.len());

    return siblings.pop().unwrap();
}

enum ParsingSegment {
    Version,
    Type,
    Literal,
    Operator,
    LengthTypeBits,
    LengthTypeCount,
    Complete,
    TestSiblings,
}

struct Packet
{
    packet_type: PacketType,
    packet_type_id: u8,
    version: u8,
    value: u64,
    nested: Vec<Packet>
}

impl Packet
{
    pub fn new() -> Packet
    {
        Packet {
            packet_type: PacketType::Literal,
            packet_type_id: 0,
            version: 0,
            value: 0,
            nested: Vec::<Packet>::new()
        }
    }
}

enum PacketType 
{
    OperatorSum,
    OperatorProduct,
    OperatorMinimum,
    OperatorMaximum,
    OperatorGreaterThan,
    OperatorLessThan,
    OperatorEqualTo,
    Literal,
}

struct  BitStream {
    bits: Vec<u8>,
    stream_pos: usize,
    start_bit: usize
}

impl BitStream
{
    pub fn new(hex_string: String) -> BitStream
    {
        let stream_pos = match hex_string.len() % 2 { 1 => {4}, 0 => {0}, _=>{panic!("Math is broken.");} };
        debug!("stream_pos init: {}", stream_pos);
        BitStream {
            bits: BitStream::hex_string_to_vec(hex_string),
            stream_pos: stream_pos,
            start_bit: stream_pos
        }

    }

    fn hex_string_to_vec(hex_string: String) -> Vec<u8>
    {
        let mut converted = Vec::<u8>::new();
    
        
        let mut v = vec![];
        let mut cur = hex_string.clone();
        while !cur.is_empty()
        {
            let hex_right: char;
            let hex_left: char;

            match cur.pop()
            {
                Some(char) => {hex_right = char;}
                None =>{break;}
            }
            match cur.pop()
            {
                Some(char) =>
                {hex_left = char;}
                None => {hex_left = '0';}
            }
            // cur.as_bytes()
            // let (chunk, rest) = cur.split_at(cmp::min(2, cur.len()));
            // v.push(chunk);
            // cur = rest;
            let mut temp = "".to_string();
            temp.push(hex_left);
            temp.push(hex_right);
            v.insert(0, temp);
        }

        for pair in v
        {
            let mut temp = pair.to_string();
            if temp.len() == 1
            {
                temp.push('0');
            }

            converted.push(BitStream::hex_to_u8(temp));
        }
        
        return converted;
    }

    fn hex_to_u8(hex: String) -> u8
    {
        if hex.len() > 2
        {
            panic!("I'm lazy and don't want to worry about real errors here, but your string is too long.");
        }

        let mut number: u8 = 0;

        for hex_char in hex.chars()
        {
            number <<= 4;
            match hex_char.to_digit(16)
            {
                Some(digit) =>
                {
                    number += digit as u8;
                }
                None =>
                {
                    panic!("Definitely an issue in the character sequence.");
                }
            }
        }

        return number;
    }

    pub fn len(&mut self) -> usize
    {
        return (self.bits.len() * 8) - self.start_bit;
    }

    pub fn remaining(&mut self) -> usize
    {
        return (self.bits.len() * 8) - self.stream_pos + self.start_bit;
    }

    pub fn stream_pos(&mut self) -> usize
    {
        return self.stream_pos - self.start_bit;
    }

    pub fn mask_into_u8(&mut self, mask_width: u8) -> Option<u8>
    {
        // This will work for reasonable mask sizes, but is gonna fall apart pretty badly if someone pushes 255 in...
        if mask_width > 8
        {
            panic!("msb_offset and mask_size are too large.");
        }

        let mut mask:u8;
        let mut value: u8;

        if self.stream_pos - self.start_bit >= self.len()
        {
            return None;
        }

        // Given a bit pos of, say, 32, our start byte is going to be 4 and our offset is 0.  So:
        // FF FF FF FF _FF_ is our byte.  We take mask_bits off the top of that.
        let start_byte = self.stream_pos / 8;
        let bit_offset: u8 = (self.stream_pos % 8) as u8;

        // If the mask width + the offset is greater than eight, it means we have fewer bits left in
        // the current start_byte than our mask is asking for.
        if mask_width + (bit_offset) > 8
        {
            trace!("Mask spills into next byte with width: {}", mask_width);
            // if the offset is at bit 6, we have only two remaining bits in the start_byte
            // Create the mask and get those two bits
            let part_mask = 2u8.pow(8u32 - bit_offset as u32) - 1;
            trace!("Mask to take last bit of start_byte: {:0>8b}", part_mask);
            value = self.bits[start_byte] & part_mask;
            trace!("Partial value: {}, or {:0>8b}", value, value);
            // If there is no last byte - if start_byte is the end of the vec - just return the value.
            if start_byte + 1 == self.bits.len()
            {
                self.stream_pos += mask_width as usize;
                return Some(value);
            }

            // now we have to figure out how many bits have not been read that were requested
            // original bit_width of mask_width, we filled 8 - bit_offset worth of them - so subtract that from
            // the original bit_width to see what's left
            let remaining:u8 = mask_width - (8u8 - bit_offset);
            trace!("Remaining bits to retrieve: {}", remaining);
            // shift the value to the left by remaining bits to make room for the last few
            value <<= remaining;
            trace!("Moved value {} bits to the left to make room: {:0>8b}", remaining, value);
            // make the mask for the bits of the next byte
            let mut other_part_mask = 2u8.pow(remaining.into()) - 1;
            // and now shift the other_part_mask 8 - remaining bits to the left
            other_part_mask <<= 8 - remaining;
            trace!("Mask for the second byte: {:0>8b}", other_part_mask);
            let mut temp = self.bits[start_byte + 1] & other_part_mask;
            trace!("Next byte {:0>8b} to produce temp value {}, bits: {:0>8b}", self.bits[start_byte + 1], temp, temp);
            temp >>= 8 - remaining;
            trace!("Temp shifted back {} bits to become {:0>8b}", 8 - remaining, temp);
            value += temp;
            trace!("final value: {}, {:0>8b}", value, value);

        }
        else
        {
            trace!("Mask width: {}", mask_width);
            // To defend against overflows, we need to take 2^mask_width as a u16 and then cast down to 8.
            mask = (2u16.pow(mask_width.into())- 1) as u8 ;
            trace!("Mask (pre-shift):   {:16b}", mask);
            // shift the mask over to the start of the unread bits
            mask <<= 8 - mask_width - bit_offset;
            trace!("Mask (post-shift):  {:16b}", mask);
            trace!("count of bits to shift back: {}", 8 - mask_width - bit_offset);
            value = (self.bits[start_byte] & mask) >> (8 - mask_width - bit_offset );
            trace!("shifted value: {:16b}bbbbbbb", value);
        }

        self.stream_pos += mask_width as usize;
    
        return Some(value);
    }

    pub fn mask_into_u16(&mut self,  mask_width: u8) -> Option<u16>
    {
        // This will work for reasonable mask sizes, but is gonna fall apart pretty badly if someone pushes 255 in...
        if mask_width > 16
        {
            panic!("mask_size is too large.");
        }

        trace!("Starting mask_into_u16 with width {}", mask_width);

        let mut msb_mask:u8;
        let mut lsb_mask:u8 = mask_width;
        let mut value: u16 = 0;

        if self.stream_pos - self.start_bit >= self.len()
        {
            trace!("Gone over or have hit the end of the stream.  stream_pos: {}, bits.len: {}", self.stream_pos, self.bits.len());
            return None;
        }

        if mask_width > 8
        {
            lsb_mask = 8;
            trace!("Mask size is > 8: {}", mask_width);
            match self.mask_into_u8(mask_width - 8)
            {
                Some(temp) =>
                {
                    trace!("masked-out contents are {}", temp);
                    value = temp as u16;
                    value <<= 8;
                    trace!("Value is now {:0>16b}", value);

                }
                None =>
                {
                    trace!("None returned???");
                    return None;
                }
            }
        }

        match self.mask_into_u8(lsb_mask)
        {
            Some(temp) =>
            {
                value += temp as u16;
            }
            None =>
            {
                return None;
            }
        }
        
        trace!("Stream position now: {}", self.stream_pos);

    
        return Some(value);
    }
}

#[cfg(test)]
mod tests
{

    use log::debug;

    use crate::day::day16::{version_sum, evaluate_packet};

    use super::{BitStream, packet_state_machine};

    fn init()
    {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    pub fn test_bit_stream_u8()
    {
        init();

        let mut test_stream1 = BitStream::new("3E00".to_string());
        let mut test_stream2 = BitStream::new("555555555".to_string());

        assert_eq!(Some(0b001), test_stream1.mask_into_u8(3));
        assert_eq!(Some(0b111), test_stream1.mask_into_u8(3));
        assert_eq!(Some(0b100), test_stream1.mask_into_u8(3));

        assert_eq!(Some(0b01010), test_stream2.mask_into_u8(5));
        assert_eq!(Some(0b10101), test_stream2.mask_into_u8(5));
    }

    #[test]
    pub fn test_mask_8_bits_u8()
    {
        init();

        let mut test_stream = BitStream::new("55555555".to_string());

        assert_eq!(Some(0b01010101), test_stream.mask_into_u8(8));

    }

    #[test]
    pub fn test_bit_stream_u16()
    {
        init();

        let mut test_stream = BitStream::new("555555555".to_string());

        assert_eq!(test_stream.len(), 36);

        assert_eq!(Some(0b01010), test_stream.mask_into_u16(5));
        assert_eq!(Some(0b101010101010), test_stream.mask_into_u16(12));
        assert_eq!(Some(0b1010101010101010), test_stream.mask_into_u16(16));
        assert_eq!(Some(0b101), test_stream.mask_into_u16(3));
        assert_eq!(None, test_stream.mask_into_u16(1));
    }

    #[test]
    pub fn packet_reader()
    {
        init();

        let literal_packet = "D2FE28".to_string();
        let length_id_operator_packet = "38006F45291200".to_string();
        let count_id_operator_packet = "EE00D40C823060".to_string();
        let four_deep = "8A004A801A8002F478".to_string();
        let three_deep = "620080001611562C8802118E34".to_string();
        let three_deep2 = "C0015000016115A2E0802F182340".to_string();
        let five_wide = "A0016C880162017C3686B18A3D4780".to_string();

        let test1 = packet_state_machine(literal_packet);
        debug!("{}, {}, {}", test1.value, test1.version, test1.packet_type_id);

        assert_eq!(2021, test1.value);
        assert_eq!(6, test1.version);
        assert_eq!(4, test1.packet_type_id);
        debug!("-------------LITERAL COMPLETE-------------");
        let test2 = packet_state_machine(length_id_operator_packet);
        assert_eq!(0b001, test2.version);
        assert_eq!(0b110, test2.packet_type_id);
        assert_eq!(2, test2.nested.len());
        assert_eq!(0b110, test2.nested[0].version);
        assert_eq!(0b100, test2.nested[0].packet_type_id);
        assert_eq!(10, test2.nested[0].value);
        assert_eq!(0b010, test2.nested[1].version);
        assert_eq!(0b100, test2.nested[1].packet_type_id);
        assert_eq!(20, test2.nested[1].value);
        debug!("-------------NESTED BIT-LENGTH OPERATOR COMPLETE-------------");
        let test3 = packet_state_machine(count_id_operator_packet);
        assert_eq!(0b111, test3.version);
        assert_eq!(0b011, test3.packet_type_id);
        assert_eq!(3, test3.nested.len());
        assert_eq!(0b010, test3.nested[0].version);
        assert_eq!(0b100, test3.nested[0].packet_type_id);
        assert_eq!(1, test3.nested[0].value);
        assert_eq!(0b100, test3.nested[1].version);
        assert_eq!(0b100, test3.nested[1].packet_type_id);
        assert_eq!(2, test3.nested[1].value);
        assert_eq!(0b001, test3.nested[2].version);
        assert_eq!(0b100, test3.nested[2].packet_type_id);
        assert_eq!(3, test3.nested[2].value);
        debug!("-------------NESTED COUNT OPERATOR COMPLETE-------------");
        let test4 = packet_state_machine(four_deep);
        assert_eq!(4, test4.version);
        assert_eq!(1, test4.nested.len());
        assert_eq!(1, test4.nested[0].version);
        assert_eq!(1, test4.nested[0].nested.len());
        assert_eq!(5, test4.nested[0].nested[0].version);
        assert_eq!(1, test4.nested[0].nested[0].nested.len());
        assert_eq!(6, test4.nested[0].nested[0].nested[0].version);
        debug!("-------------FOUR DEEP COMPLETE-------------");
        let test5 = packet_state_machine(three_deep);
        assert_eq!(3, test5.version);
        assert_eq!(2, test5.nested.len());
        assert_eq!(2, test5.nested[0].nested.len());
        assert_eq!(2, test5.nested[1].nested.len());
        debug!("-------------THREE DEEP 1 COMPLETE-------------");
        let test6 = packet_state_machine(three_deep2);
        // assert_eq!(3, test6.version);
        assert_eq!(2, test6.nested.len());
        assert_eq!(2, test6.nested[0].nested.len());
        assert_eq!(2, test6.nested[1].nested.len());
        debug!("-------------THREE DEEP 2 COMPLETE-------------");
        let test7 = packet_state_machine(five_wide);
        assert_eq!(1, test7.nested.len());
        assert_eq!(1, test7.nested[0].nested.len());
        assert_eq!(5, test7.nested[0].nested[0].nested.len());
    }

    #[test]
    pub fn test_ver_summation()
    {
        init();

        let literal_packet = "D2FE28".to_string();
        let length_id_operator_packet = "38006F45291200".to_string();
        let count_id_operator_packet = "EE00D40C823060".to_string();
        let four_deep = "8A004A801A8002F478".to_string();
        let three_deep = "620080001611562C8802118E34".to_string();
        let three_deep2 = "C0015000016115A2E0802F182340".to_string();
        let five_wide = "A0016C880162017C3686B18A3D4780".to_string();

        assert_eq!(6, version_sum(packet_state_machine(literal_packet)));
        assert_eq!(9, version_sum(packet_state_machine(length_id_operator_packet)));
        assert_eq!(14, version_sum(packet_state_machine(count_id_operator_packet)));
        assert_eq!(16, version_sum(packet_state_machine(four_deep)));
        assert_eq!(12, version_sum(packet_state_machine(three_deep)));
        assert_eq!(23, version_sum(packet_state_machine(three_deep2)));
        assert_eq!(31, version_sum(packet_state_machine(five_wide)));
    }

    #[test]
    pub fn test_value_calculation()
    {
        init();

        let sum_packet = "C200B40A82".to_string();
        let product_packet = "04005AC33890".to_string();
        let min_packet = "880086C3E88112".to_string();
        let max_packet = "CE00C43D881120".to_string();
        let less_than = "D8005AC2A8F0".to_string();
        let greater_than = "F600BC2D8F".to_string();
        let equal = "9C005AC2F8F0".to_string();
        let sum_equal_product = "9C0141080250320F1802104A08".to_string();

        assert_eq!(3, evaluate_packet(packet_state_machine(sum_packet)));
        assert_eq!(54, evaluate_packet(packet_state_machine(product_packet)));
        assert_eq!(7, evaluate_packet(packet_state_machine(min_packet)));
        assert_eq!(9, evaluate_packet(packet_state_machine(max_packet)));
        assert_eq!(1, evaluate_packet(packet_state_machine(less_than)));
        assert_eq!(0, evaluate_packet(packet_state_machine(greater_than)));
        assert_eq!(0, evaluate_packet(packet_state_machine(equal)));
        assert_eq!(1, evaluate_packet(packet_state_machine(sum_equal_product)));

    }
    
}