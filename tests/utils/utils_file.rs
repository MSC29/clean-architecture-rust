use serde::de;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

pub fn read_from_file<T: de::DeserializeOwned>(path: &str) -> Result<T, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(serde_json::from_reader(reader).unwrap())
}
