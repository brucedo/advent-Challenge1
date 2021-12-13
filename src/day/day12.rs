use std::{io::BufReader, fs::File};

use log::debug;

use crate::{graph::graph::Graph, common::common::{get_reader, read_trimmed_line}};



pub fn challenge_day_12()
{
    let mut reader = get_reader();

    part_one(&mut reader);
}



fn part_one(reader: &mut BufReader<File>)
{
    let graph = build_graph(reader);

    let mut current_path = Vec::<String>::new();
    let mut visited = Vec::<String>::new();
    let mut completed_walks = Vec::<Vec<String>>::new();
    let mut to_visit = Vec::<Vec<String>>::new();
    let mut double_visit_history = Vec::<bool>::new();
    let mut double_visit = false;

    current_path.push("start".to_string());
    visited.push("start".to_string());
    double_visit_history.push(double_visit);
    to_visit.push(graph.copy_connections(&"start".to_string()).unwrap());

    debug!("Starting walks...");
    while !current_path.is_empty()
    {
        if double_visit_history.len() == 0
        {
            double_visit = false;
        }
        else
        {
            double_visit = double_visit_history[double_visit_history.len() - 1];
        }

        debug!("-------------------");
        debug!("Current path count: {:?}", current_path);
        debug!("Double visit hist:  {:?}", double_visit_history);
        debug!("Current visited set: {:?}", visited);
        debug!("Current double visit state: {}", double_visit);
        debug!("Current set of sets to visit: {:?}", to_visit);
        if current_path[current_path.len() - 1] == "end"
        {
            
            let temp = current_path.clone();
            debug!("We've found a complete walk: {:?}", temp);
            completed_walks.push(temp);
            // Pop the "end" from the current path.  Also need to pop its connected list from the to_visit, and pop it out of "visited"
            current_path.pop();
            double_visit_history.pop().unwrap();
            // double_visit = double_visit_history[double_visit_history.len() - 1];
            to_visit.pop();
            visited.pop();
        }
        else
        {
            // Check the current node - is it 
            let mut next_visit_set = to_visit.pop().unwrap();
            if next_visit_set.is_empty()
            {
                let current_node = current_path.pop().unwrap();
                double_visit_history.pop().unwrap();
                // double_visit = double_visit_history[double_visit_history.len() - 1];
                if current_node.chars().all(char::is_lowercase)
                {
                    visited.pop();
                }
            }
            else
            {
                // Get the next item in the next_visit_set.
                let mut next = next_visit_set.pop();
                // Now, for as long as the next item to visit is in our already visited set...
                loop {
                    match next {
                        None => {
                            // Either we run out of items on the next_visit_set...
                            next = None;
                            break;
                        }
                        Some(node_name) => {
                            // Or we have at least one more item.
                            if (visited.contains(&node_name) && double_visit) || node_name == "start" {
                                next = next_visit_set.pop();
                            }
                            else {
                                // if that one more item is not on our visited list...
                                if visited.contains(&node_name) 
                                {
                                    double_visit = true;
                                }
                                next = Some(node_name);
                                break;
                            }
                        }
                    }
                }
                // This is repeated and maybe better organization might have let me do this once but...
                match next {
                    None => {
                        // If next is none, we have run through all of the contents of next_visit_set.
                        // push the empty set back on and let the next iteration take care of popping the dead end
                        // current_path node
                        to_visit.push(next_visit_set);
                    }
                    Some(next_node) =>
                    {
                        // if next is some, we have at least one more node on this path to review.
                        if next_node.chars().all(char::is_lowercase)
                        {
                            visited.push(next_node.clone());
                        }
                        to_visit.push(next_visit_set);
                        to_visit.push(graph.copy_connections(&next_node).unwrap());
                        double_visit_history.push(double_visit);
                        current_path.push(next_node);

                    }
                }
            }
        }
    }

    println!("We've found {} completed walks.", completed_walks.len());
    println!("Walks: ");
    for walk in completed_walks
    {
        println!("{:?}", walk);
    }
}

fn build_graph(reader: &mut BufReader<File>) -> Graph
{
    let mut buffer = String::new();
    let mut read_result = read_trimmed_line(reader, &mut buffer);

    let mut graph = Graph::new();
    
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

                let nodes = buffer.split("-").collect::<Vec<&str>>();
                graph.add_connection(nodes[0].to_string(), nodes[1].to_string());

                buffer.clear();
                read_result = read_trimmed_line(reader, &mut buffer);
            }
            Err(e) =>
            {
                panic!("oh blast, something died during the read in. {}", e);
            }
        }
    }

    return graph;
}