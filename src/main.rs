use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};

struct Graph {
    edges: HashMap<usize, HashSet<usize>>, // HashMap to store edges
}

impl Graph {
    // Constructor for Graph
    fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    // Method to add a directed edge between two vertices
    fn add_edge(&mut self, from: usize, to: usize) {
        self.edges
            .entry(from)
            .or_insert_with(HashSet::new)
            .insert(to);
    }

    // Method to perform Breadth-First Search 
    fn bfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(node) = queue.pop_front() {
            result.push(node);
            if let Some(neighbors) = self.edges.get(&node) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        result
    }
}

// reading a directed graph from a file
fn read_graph_from_file(filename: &str) -> Result<Graph, io::Error> {
    let file = File::open(filename)?; // Open the file
    let reader = io::BufReader::new(file); // Create a buffered reader
    let mut graph = Graph::new(); // Create a new graph instance

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line?;
        let mut iter = line.split_whitespace(); // Split line into whitespace-separated parts
        if let (Some(from), Some(to)) = (iter.next(), iter.next()) {
            let from = from.parse::<usize>().unwrap(); // Parse 'from' vertex ID
            let to = to.parse::<usize>().unwrap(); // Parse 'to' vertex ID
            graph.add_edge(from, to); // Adding the directed edge to the graph
        }
    }

    Ok(graph) // Return the constructed graph
}

fn main() -> Result<(), io::Error> {
    let filename = "ingredients.txt"; // File containing graph data
    let graph = read_graph_from_file(filename)?; // Read graph from file

    let start: usize = 0; // <= start from this vertex
    let mut distance: Vec<Option<u32>> = vec![None; graph.edges.len()];
    distance[start] = Some(0); // <= we know this distance

    // Perform BFS starting from the specified vertex
    let bfs_result = graph.bfs(start);
    println!("BFS traversal result: {:?}", bfs_result);

    // vertex-distance pairs
    print!("vertex:distance");
    for v in 0..graph.edges.len() {
        print!("   {}:{}", v, distance[v].unwrap_or(u32::MAX));
    }
    println!();

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_traversal() {
        // sample graph for testing
        let mut graph = Graph::new();
        let edges = vec![
            (0, 1), 
            (0, 2),
            (1, 3),
            (2, 3), 
            (2, 4),
            (3, 5),
            (4, 5),
        ];

        // adding edges to the graph
        for (from, to) in edges {
            graph.add_edge(from, to);
        }

        // BFS traversal starting from vertex 0
        let bfs_result = graph.bfs(0);

        // expected BFS traversal result
        let expected_result = vec![0, 1, 2, 3, 4, 5];

        // does the BFS traversal result matche the expected result?
        assert_eq!(bfs_result, expected_result);
    }
}



