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
    use std::collections::HashMap;

    #[test]
    fn test_bfs_distances() {
        let mut graph = Graph::new();
        // Adding sample edges
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 3);

        // Perform BFS from vertex 0
        let distances = graph.bfs_distances(0);

        // Expected distances after BFS traversal
        let expected_distances: HashMap<usize, usize> = [
            (0, 0), // Distance from start vertex to itself is 0
            (1, 1),
            (2, 1),
            (3, 2),
        ]
        .iter()
        .cloned()
        .collect();

        // Assert that the distances match the expected distances
        assert_eq!(distances, expected_distances);
    }

    #[test]
    fn test_shortest_path() {
        let mut graph = Graph::new();
        // Adding some sample edges
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);

        // Finding the shortest path from vertex 0 to vertex 4
        let shortest_path = graph.shortest_path(0, 4);

        // Expected shortest path
        let expected_path = Some(vec![0, 2, 4]);

        // Assert that the shortest path matches the expected path
        assert_eq!(shortest_path, expected_path);
    }
}





