use rand::Rng;
use rand::seq::SliceRandom;

#[derive(Debug)] // To Print Our Graph
pub struct Graph {
    n: usize, // First Line of Data.txt
    adj_list: Vec<Vec<usize>>, // Adjaceny List Values of Data.Tzt
}

impl Graph {
    pub fn new(n: usize) -> Graph { // Creates the Initial Graph
        Graph { n, adj_list: vec![vec![]; n] }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) { // Edges for Our Graph
        self.adj_list[u].push(v);
    }

    pub fn random_walk(&self, start: usize, steps: usize) -> usize { // The Start and Steps of Our Random Walk
        let mut rng = rand::thread_rng(); // Random Initialization
        let mut current = start;
        for _ in 0..steps { // Skips First Line
            let neighbors = &self.adj_list[current];
            if neighbors.is_empty() {
                // Jump to a Random Vertex in Graph
                current = rng.gen_range(0..self.n);
            } else {
                // With Probability 0.9 Follow an Edge, with 0.1 Jump Randomly
                if rng.gen_bool(0.9) {
                    current = *neighbors.choose(&mut rng).unwrap();
                } else {
                    current = rng.gen_range(0..self.n);
                }
            }
        }
        current // Prints the Ending Step of our Walk
    }

    pub fn simulate_pagerank(&self, walks_per_vertex: usize, walk_length: usize) -> Vec<f64> {
        let mut counts = vec![0; self.n];
        for start in 0..self.n {
            for _ in 0..walks_per_vertex {
                let end = self.random_walk(start, walk_length);
                counts[end] += 1; // Ranking System
            }
        }
        let total_walks = walks_per_vertex * self.n; // Calculates Walks
        counts.iter().map(|&c| c as f64 / total_walks as f64).collect() // Returns Counts --> Our Ranking System for PageRank
    }
}