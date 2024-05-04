mod data_loading;
mod graph_construction;
mod analysis;

fn main() {
    let result = data_loading::load_data("Precinct_Data.csv");
    match result {
        Ok(records) => {
            match graph_construction::build_graph(&records) {
                Ok((graph, vote_aggregation)) => {
                    analysis::print_summarized_degree_distribution(&graph);
                    let top_candidates = analysis::top_ten_candidates(&vote_aggregation);
                    println!("Top 10 Candidates:");
                    for (candidate, total_votes) in top_candidates {
                        println!("{}: {}", candidate, total_votes);
                    }
                    println!("Graph initialized with {} nodes and {} edges.", graph.node_count(), graph.edge_count());
                },
                Err(e) => println!("Error building the graph: {}", e),
            }
        },
        Err(e) => println!("Failed to load data: {}", e),
    }
}
