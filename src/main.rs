use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};


struct Graph {
    edges: HashMap<usize, HashSet<usize>>,  // HashMap to store edges
}

impl Graph {
    // Constructor for Graph
    fn new() -> Self {
        Self { edges: HashMap::new() }
    }

    // Method to add a directed edge between two vertices
    fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.entry(from).or_insert_with(HashSet::new).insert(to);
    }
}

// reading a directed graph from a file
fn read_graph_from_file(filename: &str) -> Result<Graph, io::Error> {
    let file = File::open(filename)?;               // Open the file
    let reader = io::BufReader::new(file);          // Create a buffered reader
    let mut graph = Graph::new();                   // Create a new graph instance

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line?;                          
        let mut iter = line.split_whitespace();     // Split line into whitespace-separated parts
        if let (Some(from), Some(to)) = (iter.next(), iter.next()) {
            let from = from.parse::<usize>().unwrap();  // Parse 'from' vertex ID
            let to = to.parse::<usize>().unwrap();      // Parse 'to' vertex ID
            graph.add_edge(from, to);               // Add the directed edge to the graph
        }
    }

    Ok(graph)  // Return the constructed graph
}

fn main() -> Result<(), io::Error> {
    let filename = "ingredients.txt";          // File containing graph data
    let _graph = read_graph_from_file(filename)?;  // Read graph from file
    Ok(())
}



