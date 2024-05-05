use std::fs::File;
use std::io::{self, BufRead};
use crate::graph::*;

pub mod graph;

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
    let mut distance: Vec<Option<usize>> = vec![None; graph.edges.len()];
    distance[start] = Some(0); // <= we know this distance

    // Perform BFS starting from the specified vertex
    let bfs_result = graph.bfs_distances(start);
    println!("BFS traversal result: {:?}", bfs_result);

    // Calculate the average distance
    let avg_distance = graph.average_distance(start);
    println!("Average distance from node {}: {}", start, avg_distance);

    // Calculating the shortest path
    let end: usize = 400; // <= end at this vertex
    if let Some(shortest_path) = graph.shortest_path(start, end) {
        println!("Shortest path from {} to {}: {:?}", start, end, shortest_path);
    } else {
        println!("No path found from {} to {}", start, end);
    }

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

        // does the BFS traversal result match the expected result?
        assert_eq!(bfs_result, expected_result);
    }
}
#[test]
fn test_shortest_path() {
    // Sample graph for testing
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
    for (from, to) in edges {
        graph.add_edge(from, to);
    }

    // Test cases for shortest paths
    let test_cases = vec![
        // vertex 0 to vertex 5
        (0, 5, vec![0, 2, 4, 5]),
        // vertex 1 to vertex 5
        (1, 5, vec![1, 3, 5]),
        // vertex 2 to vertex 5
        (2, 5, vec![2, 4, 5]),
        // vertex 3 to vertex 5
        (3, 5, vec![3, 5]),
    ];

    for (start, end, expected_path) in test_cases {
        if let Some(shortest_path) = graph.shortest_path(start, end) {
            assert_eq!(shortest_path, expected_path);
        } else {
            panic!("No path found from {} to {}", start, end);
        }
    }
}




