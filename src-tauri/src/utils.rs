use std::{collections::HashMap, fs::OpenOptions, io::BufReader};

type Sources = HashMap<String, u32>;

pub fn read_json() -> Sources {
  let sources_json = OpenOptions::new()
    .read(true)
    .open("./sources.json")
    .unwrap();
  let reader = BufReader::new(sources_json);
  let sources: Sources = serde_json::from_reader(reader).unwrap();
  println!("{:?}", sources);
  sources
}

#[test]
fn test_read_json() {
  let res = read_json();
}
