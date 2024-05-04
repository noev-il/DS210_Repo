use csv::ReaderBuilder;
use serde::{Deserialize, Deserializer};
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct ElectionRecord {
    #[serde(deserialize_with = "deserialize_option_string")]
    pub precinct: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string")]
    pub candidate: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string")]
    pub votes: Option<String>,
}

pub fn deserialize_option_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(Some(opt.unwrap_or_else(|| "MISSING".to_string())))
}

pub fn load_data(file_path: &str) -> Result<Vec<ElectionRecord>, Box<dyn Error>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_data() {
        let result = load_data("Precinct_Data.csv");
        assert!(result.is_ok());
        let records = result.unwrap();
        assert_eq!(records.len(), 836424);  // Length of the CSV File is 836424 (MINUES THE HEADER)
    }
}