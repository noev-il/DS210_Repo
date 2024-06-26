use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::{HashMap, BTreeMap};
use crate::data_loading::ElectionRecord; // Ensure ElectionRecord is accessible here.

pub fn build_graph(records: &[ElectionRecord]) -> Result<(UnGraph<String, u32>, HashMap<String, BTreeMap<String, u32>>), String> {
    let mut graph = UnGraph::<String, u32>::new_undirected();
    let mut index_map: HashMap<String, NodeIndex> = HashMap::new();
    let mut vote_aggregation: HashMap<String, BTreeMap<String, u32>> = HashMap::new();
    let mut candidate_to_precincts: HashMap<String, Vec<String>> = HashMap::new();

    // Aggregate votes
    for record in records {
        if let Some(precinct) = &record.precinct {
            if let Some(candidate) = &record.candidate {
                let votes: u32 = record.votes.as_ref().map(|v| v.parse().unwrap_or(0)).unwrap_or(0);
                vote_aggregation.entry(precinct.clone())
                    .or_insert_with(BTreeMap::new)
                    .entry(candidate.clone())
                    .and_modify(|e| *e += votes)
                    .or_insert(votes);
            }
        }
    }

    // Determine top candidates and map them to precincts
    for (precinct, candidates) in &vote_aggregation {
        let top_candidate = candidates.iter()
            .max_by_key(|(_, votes)| *votes)
            .map(|(candidate, _)| candidate.clone())
            .unwrap_or_default();

        candidate_to_precincts.entry(top_candidate)
            .or_insert_with(Vec::new)
            .push(precinct.clone());
    }

    // Build the graph with nodes and edges
    for (_candidate, precincts) in candidate_to_precincts {
        for precinct in &precincts {
            let node_index = *index_map.entry(precinct.clone()).or_insert_with(|| graph.add_node(precinct.clone()));

            for other_precinct in &precincts {
                if precinct != other_precinct {
                    let other_node_index = *index_map.entry(other_precinct.clone()).or_insert_with(|| graph.add_node(other_precinct.clone()));
                    graph.add_edge(node_index, other_node_index, 1);
                }
            }
        }
    }

    Ok((graph, vote_aggregation))
}
#[cfg(test)]
mod tests {
    use super::*;

    // Helper Function to Keep Record
    fn create_record(precinct: &str, candidate: &str, votes: &str) -> ElectionRecord {
        ElectionRecord {
            precinct: Some(precinct.to_string()),
            candidate: Some(candidate.to_string()),
            votes: Some(votes.to_string()),
        }
    }

    #[test]
    fn test_build_graph() {
        // Create a vector of ElectionRecords
        let records = vec![
            create_record("Precinct1", "CandidateA", "100"),
            create_record("Precinct1", "CandidateB", "150"),
            create_record("Precinct2", "CandidateA", "200"),
            create_record("Precinct3", "CandidateB", "250"),
        ];
        let result = build_graph(&records).expect("Failed to build graph");
        let (graph, vote_aggregation) = result;

        // Check graph properties
        assert_eq!(graph.node_count(), 3, "Graph should have 3 nodes");
        assert_eq!(graph.edge_count(), 2, "Graph should have 2 edges");

        // Check vote aggregation
        assert_eq!(vote_aggregation.get("Precinct1").unwrap().get("CandidateA").unwrap(), &100);
        assert_eq!(vote_aggregation.get("Precinct1").unwrap().get("CandidateB").unwrap(), &150);
        assert_eq!(vote_aggregation.get("Precinct2").unwrap().get("CandidateA").unwrap(), &200);
        assert_eq!(vote_aggregation.get("Precinct3").unwrap().get("CandidateB").unwrap(), &250);

        // Check more properties if needed
    }
}