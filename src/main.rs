#[macro_use] extern crate clap;
extern crate serde_json;
extern crate rayon;

use std::io::{BufRead, BufReader};
use std::process;
use std::iter::Iterator;
use clap::App;
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

fn main() {
    let cli_cnf = load_yaml!("cli.yml");
    let matched = App::from_yaml(cli_cnf).get_matches();

    let input = match matched.value_of("input") {
        Some(input) => String::from(input),
        None => {
            println!("--input <GeoJSON> arg required");
            process::exit(1);
        }
    };

    let mut indexer = Indexer::new(input);
    let mut docs = Vec::with_capacity(10000);

    for _ in 0..10000 {
        match indexer.next() {
            Some(doc) => { docs.push(doc); },
            None => { break; }
        };
    }

    let _i: Vec<i64> =  docs.par_iter().map(|doc| {
        println!("{:?}", &doc);    
        1
    }).collect();
}
