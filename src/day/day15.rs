use std::{io::BufReader, fs::File, collections::{HashMap, hash_map::Entry, BinaryHeap}, cmp::Ordering};

use log::debug;

use crate::common::common::{get_reader, read_trimmed_line, char_to_i8};

pub fn challenge_day_15()
{
    let mut reader = get_reader();

    let mut map = load_map(&mut reader);

    debug!("Map: {:?}", map);

    djikstra(&mut map);

    let mut expanded_map = map_expander(map, 5, 5);

    for row in 0..expanded_map.len()
    {
        for column in 0..expanded_map[row].len()
        {
            print!("{} ", expanded_map[row][column].risk);
        }
        print!("\n");
    }

    djikstra(&mut expanded_map);
}

fn djikstra(map: &mut Vec<Vec<Node>>)
{
    let mut pqueue = BinaryHeap::<NodeOrder>::new();
    let total_risk: u32;
    pqueue.push(NodeOrder {distance: 0, index:(0,0)});

    let mut previous: Option<(usize, usize)> = None;


    loop
    {
        let next = pqueue.pop().unwrap();
        let mut index = next.index;

        debug!("Looking at index {},{}", index.0, index.1);

        while map[index.0][index.1].visited
        {
            let next = pqueue.pop().unwrap();
            index = next.index;
            debug!("Looking at index {},{}", index.0, index.1);
        }

        // neighbor indices
        let neighbors = build_neighbors(index, map);
        debug!("Neighbors: {:?}", neighbors);
        debug!("Risk from start to visited node: {}", map[index.0][index.1].tentative_distance);
        for neighbor in neighbors
        {
            if map[neighbor.0][neighbor.1].visited
            {
                continue;
            }

            let distance = map[index.0][index.1].tentative_distance + map[neighbor.0][neighbor.1].risk;
            debug!("Risk from start to neighbor {}, {}: {}", neighbor.0, neighbor.1, distance);
            map[neighbor.0][neighbor.1].tentative_distance = std::cmp::min(distance, map[neighbor.0][neighbor.1].tentative_distance);
            pqueue.push(NodeOrder{distance: map[neighbor.0][neighbor.1].tentative_distance, index: (neighbor.0, neighbor.1)});
        }

        
        map[index.0][index.1].visited = true;
        map[index.0][index.1].previous = previous;
        previous = Some((index.0, index.1));

        if index.0 == map.len() - 1 && index.1 == map[index.0].len() - 1
        {
            debug!("Stopping at index {}, {}", index.0, index.1);
            total_risk = map[index.0][index.1].tentative_distance;
            break;
        }

        // break;
    }

    // Trace back the path to the start
    // let mut trace_path = (map.len() - 1, map[0].len() - 1);

    // while trace_path != (0, 0)
    // {
    //     println!("Index: {:?}", trace_path);
    //     match map[trace_path.0][trace_path.1].previous
    //     {
    //         None => {println!("The path back to root is broken.  Something has gone badly wrong."); break;}
    //         Some(tuple) => {trace_path = tuple;}
    //     }
    // }

    println!("Total risk: {}", total_risk);
}

fn build_neighbors(index: (usize, usize), map: &Vec<Vec<Node>>) -> Vec<(usize, usize)>
{
    let mut neighbors = Vec::<(usize, usize)>::new();
    let previous_row = index.0.checked_sub(1);
    let next_row = index.0.checked_add(1);
    let previous_col = index.1.checked_sub(1);
    let next_col = index.1.checked_add(1);
    if !previous_row.is_none()
    {
        neighbors.push((previous_row.unwrap(), index.1));
    }
    if !previous_col.is_none()
    {
        neighbors.push((index.0, previous_col.unwrap()))
    }
    if !next_row.is_none() && next_row.unwrap() < map.len()
    {
        neighbors.push((next_row.unwrap(), index.1));
    }
    if !next_col.is_none() && next_col.unwrap() < map[index.0].len()
    {
        neighbors.push((index.0, next_col.unwrap()));
    }

    return neighbors;
}

#[derive(Debug)]
struct Node {
    risk: u32,
    tentative_distance: u32,
    visited: bool,
    previous: Option<(usize, usize)>
}

struct NodeOrder {
    index: (usize, usize),
    distance: u32
}

impl Ord for NodeOrder
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for NodeOrder
{

}

impl PartialOrd for NodeOrder
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.eq(other)
        {
            Some(Ordering::Equal)
        }
        else if self.distance < other.distance
        {
            Some(Ordering::Greater)
        }
        else
        {
            Some(Ordering::Less)
        }
    }
}

impl PartialEq for NodeOrder
{

    fn eq(&self, rhs: &NodeOrder) -> bool 
    { 
        self.distance == rhs.distance
    }
}

fn map_expander(map: Vec<Vec<Node>>, height_multiplier: usize, width_multiplier: usize) -> Vec<Vec<Node>>
{
    // Starting with the old map, we copy it across 5 times, increasing the risk of every 
    // new node by 1 modulo 10.  This function assumes a rectangular map.
    let map_height = map.len();
    let map_width = map[0].len();

    let mut expanded = Vec::<Vec<Node>>::new();

    debug!("expanded map height multiplier: {}", height_multiplier);
    debug!("expanded map width multiplier: {}", width_multiplier);

    for more_maps_down_thataway in 0 .. height_multiplier
    {
        for row_index in 0..map_height
        {
            let mut row_vector = Vec::<Node>::new();
            for j in more_maps_down_thataway..more_maps_down_thataway + width_multiplier
            {
                debug!("Risk addition factor: {}", j);
                for column_index in 0..map_width
                {
                    let mut temp = Node
                    {
                        risk: 0,
                        tentative_distance: u32::MAX,
                        visited: false,
                        previous: None
                    };

                    temp.risk = map[row_index][column_index].risk + j as u32;
                    if temp.risk > 9 {temp.risk -= 9;}
                    row_vector.push(temp);

                }
            }
            expanded.push(row_vector);
        }
    }

    // Fix up the tentative_distance of our entry node.
    expanded[0][0].tentative_distance = 0;

    return expanded;
}

fn load_map(reader: &mut BufReader<File>) -> Vec<Vec<Node>> //HashMap<(usize, usize), Node>
{
    let mut buffer = String::new();
    let mut read_result = read_trimmed_line(reader, &mut buffer);

    // let mut map = HashMap::<(usize, usize), Node>::new();
    let mut map = Vec::<Vec<Node>>::new();

    loop
    {
        match read_result
        {
            Ok(size) =>
            {
                if size == 0
                {
                    break;
                }
                let mut next_row = Vec::<Node>::new();
                for area in buffer.chars()
                {
                    let node = Node {
                        risk: char_to_i8(area) as u32,
                        tentative_distance: u32::MAX,
                        visited: false,
                        previous: None
                    };
                    // map.insert((row, col), node);
                    next_row.push(node);
                }
                map.push(next_row);

                buffer.clear();
                read_result = read_trimmed_line(reader, &mut buffer);
            }
            Err(e) =>
            {
                panic!("An error occurred while reading in the file: {}", e);
            }
        }
    }

    // Fix up the start entry a bit.
    map[0][0].tentative_distance = 0;

    return map;
}