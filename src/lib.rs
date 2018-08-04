extern crate serde_json;
extern crate rayon;

use std::io::{BufRead, BufReader};
use std::process;
use std::iter::Iterator;
use std::fs::File;
use serde_json::Value;
use rayon::prelude::*;

pub struct Indexer {
    stream: BufReader<File>
}

impl Indexer {
    pub fn new(path: String) -> Self {
        Indexer {
            stream: BufReader::new(File::open(path).unwrap()),
        }
    }
}

impl Iterator for Indexer {
    type Item = Value;

    fn next(&mut self) -> Option<Value> {
        let mut data = Vec::new();

        match self.stream.read_until(b'\n', &mut data).unwrap() {
            0 => None,
            _ => {
                let input: Value = match serde_json::from_slice(&data) {
                    Ok(input) => input,
                    Err(_) => panic!("Not Valid JSON")
                };

                Some(input)
            }
            
        }
    }
}
