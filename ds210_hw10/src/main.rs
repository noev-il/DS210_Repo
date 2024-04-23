mod graph; // Include the graph module
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use crate::graph::Graph; // Use Graph from the graph module

fn read_graph(filepath: &str) -> io::Result<Graph> { // Simple Read Graph Function 
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let n = lines.next().unwrap()?.parse::<usize>().unwrap();
    let mut graph = Graph::new(n);

    for line in lines {
        if let Ok(edge) = line {
            let parts: Vec<_> = edge.split_whitespace().collect();
            let u = parts[0].parse::<usize>().unwrap();
            let v = parts[1].parse::<usize>().unwrap();
            graph.add_edge(u, v);
        }
    }
    Ok(graph) // Ok Container to Return Graph
}

#[cfg(test)]
mod tests {
    use super::*; // Import Everything 
    #[test]
    fn test_simple_pagerank() {
        let mut graph = Graph::new(2); 
        graph.add_edge(0, 1); 
        let pageranks = graph.simulate_pagerank(100, 10);
        assert!(pageranks[1] > pageranks[0], "Vertex 1 should have a higher PageRank.");
    }
}
fn main() -> io::Result<()> {
    let graph = read_graph("data2.txt")?; // Replaces data2.txt with data{x}.txt where 0 <= x <= 2
    let pageranks = graph.simulate_pagerank(100, 100);

    let mut rankings: Vec<_> = pageranks.iter().enumerate().collect();
    rankings.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    for (vertex, rank) in rankings.iter().take(5) {
        println!("vertex {}: approximate PageRank {:.3}", vertex, rank);
    }
    Ok(()) // Returns OK Type to Fulfill Return Values
}
