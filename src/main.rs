use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};

struct Graph {
    edges: HashMap<usize, 
    HashSet<usize>>, // HashMap to store edges
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

    fn bfs_distances(&self, start: usize) -> HashMap<usize, usize> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut distances = HashMap::new();

        queue.push_back((start, 0));
        visited.insert(start);

        while let Some((node, dist)) = queue.pop_front() {
            distances.insert(node, dist);
            if let Some(neighbors) = self.edges.get(&node) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        queue.push_back((neighbor, dist + 1));
                    }
                }
            }
        }

        distances
    }

    // Method to find the average distance between nodes using BFS
    fn average_distance(&self, start: usize) -> f64 {
        let distances = self.bfs_distances(start);
        let total_distance: usize = distances.values().sum();
        let num_nodes = distances.len();
        total_distance as f64 / num_nodes as f64
    }

    fn shortest_path(&self, start: usize, end: usize) -> Option<Vec<usize>> {
        let mut distance: HashMap<usize, usize> = HashMap::new();
        let mut previous: HashMap<usize, Option<usize>> = HashMap::new();
        let mut queue = VecDeque::new();

        distance.insert(start, 0);
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            if node == end {
                break; // destination vertex
            }

            if let Some(neighbors) = self.edges.get(&node) {
                for &neighbor in neighbors {
                    let new_distance = distance[&node] + 1;
                    if !distance.contains_key(&neighbor) || new_distance < distance.get(&neighbor).copied().unwrap_or(usize::MAX) {
                        distance.insert(neighbor, new_distance);
                        previous.insert(neighbor, Some(node));
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        if let Some(_) = distance.get(&end) {
            let mut path = vec![];
            let mut current = end;
            while let Some(prev) = previous.get(&current).unwrap() {
                path.push(current);
                if *prev == start {
                    path.push(*prev);
                    break;
                }
                current = *prev;
            }
            path.reverse();
            Some(path)
        } else {
            None // No path found
        }
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




