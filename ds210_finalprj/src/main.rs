use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::{HashMap, BTreeMap};
use csv::{Reader, ReaderBuilder};
use serde::{Deserialize, de::{self, Deserializer}};
use std::error::Error;
use std::fs::File;

fn deserialize_option_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(Some(opt.unwrap_or_else(|| "MISSING".to_string())))
}

#[derive(Debug, Deserialize)]
struct ElectionRecord {
    #[serde(deserialize_with = "deserialize_option_string")]
    precinct: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string")]
    candidate: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string")]
    votes: Option<String>,
}

fn load_data(file_path: &str) -> Result<Vec<ElectionRecord>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut data = Vec::new();
    for result in rdr.deserialize() {
        let record: ElectionRecord = result?;
        data.push(record);
    }
    Ok(data)
}

// Function to find the top 10 candidates by total votes
fn top_ten_candidates(vote_aggregation: &HashMap<String, BTreeMap<String, u32>>) -> Vec<(String, u32)> {
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



fn main() {
    let result = load_data("Precinct_Data.csv");
    match result {
        Ok(records) => {
            let mut graph = UnGraph::<String, u32>::new_undirected();
            let mut index_map: HashMap<String, NodeIndex> = HashMap::new();
            let mut vote_aggregation: HashMap<String, BTreeMap<String, u32>> = HashMap::new();
            let mut candidate_to_precincts: HashMap<String, Vec<String>> = HashMap::new();

            // Aggregate Votes
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


            for (candidate, precincts) in candidate_to_precincts {
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
            let top_candidates = top_ten_candidates(&vote_aggregation);
            println!("Top 10 Candidates:");
            for (candidate, total_votes) in top_candidates {
                println!("{}: {}", candidate, total_votes);
            }
            println!("Graph initialized with {} nodes and {} edges.", graph.node_count(), graph.edge_count());
        },
        Err(e) => println!("Failed to load data: {}", e),
    }
}