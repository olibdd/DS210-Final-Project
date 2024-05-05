use std::collections::{HashMap, HashSet, VecDeque};

pub struct Graph {
    pub edges: HashMap<usize, 
    HashSet<usize>>, // HashMap to store edges
}

impl Graph {
    // Constructor for Graph
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    // Method to add a directed edge between two vertices
    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.edges
            .entry(from)
            .or_insert_with(HashSet::new)
            .insert(to);
    }

    pub fn bfs_distances(&self, start: usize) -> HashMap<usize, usize> {
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
    pub fn average_distance(&self, start: usize) -> f64 {
        let distances = self.bfs_distances(start);
        let total_distance: usize = distances.values().sum();
        let num_nodes = distances.len();
        total_distance as f64 / num_nodes as f64
    }

    pub fn shortest_path(&self, start: usize, end: usize) -> Option<Vec<usize>> {
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