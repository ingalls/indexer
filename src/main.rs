#[macro_use] extern crate clap;
extern crate rayon;
extern crate indexer;

use indexer::Indexer;
use std::process;
use std::iter::Iterator;
use clap::App;
use rayon::prelude::*;

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
