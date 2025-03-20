use serde_json;
use std::io::BufReader;
use std::fs;
use rand::prelude::*;

const  WORDS_FILE_PATH: &str = "words.json";

pub fn get_random_word() -> Result<String, std::io::Error> {
  let json = get_json()?;
  let mut rng = rand::rng();
  let random_idx: usize = rng.random_range(1..=200);
  let words = match json["words"].as_array() {
    Some(w) => w,
    None => { return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Could not parse words.json file!")); }
  };
  Ok(words[random_idx].to_string().replace("\"", ""))
}

fn get_json() -> Result<serde_json::Value, std::io::Error> {
  let file = fs::File::open(WORDS_FILE_PATH);
  let reader = BufReader::new(file?);
  let json: serde_json::Value = serde_json::from_reader(reader).expect("Could not read words.json file!");
  Ok(json)
}