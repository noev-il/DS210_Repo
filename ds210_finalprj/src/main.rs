use csv::{Reader, ReaderBuilder};
use serde::{Deserialize, Deserializer};
use std::error::Error;
use std::fs::File;
use serde::de::{self, Visitor};
use std::fmt;


fn deserialize_option_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(Some(opt.unwrap_or_else(|| "MISSING".to_string())))
}// THIS FUNCTION REPLACES ALL MISSING VALUES WITH THE STRING "MISSING"

#[derive(Debug, Deserialize)]
struct ElectionRecord { // CREATES A STRUCT TO TO HOLD THE DATA
    #[serde(deserialize_with = "deserialize_option_string")]
    precinct: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string")]
    office: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string")]
    party_detailed: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string")]
    party_simplified: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string")]
    mode: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string")]
    votes: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string")]
    county_name: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string")]
    county_fips: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string")]
    jurisdiction_name: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string")]
    jurisdiction_fips: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string")]
    candidate: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string")]
    district: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string")]
    magnitude: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string")]
    dataverse: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string")]
    year: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string")]
    stage: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string")]
    state: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string")]
    special: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string")]
    writein: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string")]
    state_po: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string")]
    state_fips: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string")]
    state_cen: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string")]
    state_ic: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string")]
    date: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string")]
    readme_check: Option<String>,
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

fn main() {
    match load_data("Precinct_Data.csv") {
        Ok(records) => {
            println!("Data loaded successfully!");
            for record in records {
                println!("{:?}", record);
            }
        },
        Err(e) => println!("Failed to load data: {}", e),
    }
}
