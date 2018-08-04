extern crate serde_json;

use std::io::{BufRead, BufReader};
use std::iter::Iterator;
use std::fs::File;
use std::process;
use serde_json::Value;

pub struct Indexer {
    zoom: u8,
    stream: BufReader<File>
}

impl Indexer {
    pub fn new(path: String, zoom: u8) -> Self {
        if zoom > 14 || zoom < 1 {
            println!("zoom must be greater than 0 less than 15--- zoom was {}", zoom);
            process::exit(1);
        }

        Indexer {
            zoom: zoom,
            stream: BufReader::new(File::open(path).unwrap()),
        }
    }

    pub fn process(doc: &Value) -> i64 {
        1
    }
}

impl Iterator for Indexer {
    type Item = Value;

    fn next(&mut self) -> Option<Value> {
        let mut data = Vec::new();

        loop {
            match self.stream.read_until(b'\n', &mut data).unwrap() {
                0 => { return None; },
                _ => {
                    //Skip empty lines
                    if data.len() == 1 as usize && data[0] == 10 as u8 { continue; }

                    let input: Value = match serde_json::from_slice(&data) {
                        Ok(input) => input,
                        Err(_) => panic!("Not Valid JSON")
                    };

                    return Some(input)
                }
                
            }
        }
    }
}
