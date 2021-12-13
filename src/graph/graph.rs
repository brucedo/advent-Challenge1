use std::collections::{HashMap, hash_map::Entry};

pub struct Graph
{
    nodes: HashMap<String, Node>

}

struct Node
{
    pub name: String,
    pub connections: Vec<String>
}

impl Graph
{
    pub fn new() -> Graph
    {
        Graph
        {
            nodes: HashMap::<String, Node>::new()
        }
    }

    pub fn add_node(&mut self, name: String)
    {
        let new_node = Node{
            name: "".to_string(),
            connections: Vec::<String>::new()
        };

        self.nodes.insert(name, new_node);
    }

    pub fn add_connection(&mut self, from: String, to: String)
    {
        // let from_lookup = self.nodes.get(&from);
        // let to_lookup = self.nodes.get(&to);
        let from_clone = from.clone();
        let to_clone = to.clone();


        match self.nodes.entry(from) {
            Entry::Vacant(entry) =>
            {
                let mut temp = Node{name: "".to_string(), connections: Vec::<String>::new()};
                temp.connections.push(to_clone);
                entry.insert(temp);
            }
            Entry::Occupied(entry) =>
            {
                entry.into_mut().connections.push(to_clone);
            }
        };

        match self.nodes.entry(to)
        {
            Entry::Vacant(entry) =>
            {
                let mut temp = Node {name: "".to_string(), connections: Vec::<String>::new()};
                temp.connections.push(from_clone);
                entry.insert(temp);
            }
            Entry::Occupied(entry) =>
            {
                entry.into_mut().connections.push(from_clone);
            }
        }


    }

    pub fn borrow_connections(&self, name: &String) -> Option<&Vec<String>>
    {
        
        let lookup = self.nodes.get(name)?;
        
        return Option::Some(&lookup.connections);
    }

    pub fn copy_connections(&self, name: &String) -> Option<Vec<String>>
    {
        let lookup = self.nodes.get(name)?;

        let copy = lookup.connections.clone();

        return Option::Some(copy);
    }
}