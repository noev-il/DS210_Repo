use petgraph::graph::UnGraph;
use std::collections::{HashMap, BTreeMap};  // Import BTreeMap here

pub fn print_summarized_degree_distribution(graph: &UnGraph<String, u32>) {
    let mut degree_counts: HashMap<String, usize> = HashMap::new();

    for node_idx in graph.node_indices() {
        let degree = graph.edges(node_idx).count();
        let range = match degree {
            0..=5 => "0-5",
            6..=10 => "6-10",
            11..=20 => "11-20",
            21..=50 => "21-50",
            51..=100 => "51-100",
            101..=200 => "101-200",
            201..=500 => "201-500",
            501..=1000 => "501-1000",
            _ => "1001+",
        };
        *degree_counts.entry(range.to_string()).or_insert(0) += 1;
    }

    let mut degrees: Vec<_> = degree_counts.iter().collect();
    degrees.sort_by_key(|&(range, _)| range);
    
    println!("Summarized Degree Distribution:");
    for (range, count) in degrees {
        println!("{}: {} nodes", range, count);
    }
}

pub fn top_ten_candidates(vote_aggregation: &HashMap<String, BTreeMap<String, u32>>) -> Vec<(String, u32)> {
    let mut candidate_totals: HashMap<String, u32> = HashMap::new();

    // Aggregate total votes for each candidate across all precincts
    for candidates in vote_aggregation.values() {
        for (candidate, votes) in candidates {
            // Skip "UNDERVOTES" and "PUBLIC COUNTER" instances
            if candidate != "UNDERVOTES" && candidate != "PUBLIC COUNTER" {
                *candidate_totals.entry(candidate.clone()).or_insert(0) += votes;
            }
        }
    }

    // Collect and sort candidates by total votes
    let mut sorted_candidates: Vec<_> = candidate_totals.into_iter().collect();
    sorted_candidates.sort_by(|a, b| b.1.cmp(&a.1)); // Sort descending by votes

    // Return the top 10 candidates
    sorted_candidates.into_iter().take(10).collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, BTreeMap};

    #[test]
    fn test_exclude_special_candidates() {
        let mut vote_aggregation: HashMap<String, BTreeMap<String, u32>> = HashMap::new();
        vote_aggregation.entry("Precinct1".to_string()).or_insert_with(BTreeMap::new).insert("Candidate1".to_string(), 500);
        vote_aggregation.entry("Precinct1".to_string()).or_insert_with(BTreeMap::new).insert("UNDERVOTES".to_string(), 150);
        vote_aggregation.entry("Precinct1".to_string()).or_insert_with(BTreeMap::new).insert("PUBLIC COUNTER".to_string(), 300);

        let top_candidates = top_ten_candidates(&vote_aggregation);

        // Ensure No Unwanted Entries Included
        for (candidate, _) in &top_candidates {
            assert_ne!(candidate, "UNDERVOTES");
            assert_ne!(candidate, "PUBLIC COUNTER");
        }
    }
}