#[macro_use] extern crate clap;
extern crate serde_json;

use std::io::{BufRead, BufReader};
use clap::App;
use std::fs::File;
use serde_json::Value;

fn main() {
    let cli_cnf = load_yaml!("cli.yml");
    let matched = App::from_yaml(cli_cnf).get_matches();

    let input = String::from(matched.value_of("input").unwrap());

    match distribute_input(&input) {
        Err(err) => {
            println!("{}", err);
        },
        _ => {
            println!("OK");
        }
    };
}

fn distribute_input(input_path: &String) -> std::io::Result<()> {
    let mut data = Vec::new();

    let file = File::open(input_path)?;
    let mut stream = BufReader::new(file);

    println!("HERE");

    loop {
        data.clear();

        let bytes_read = stream.read_until(b'\n', &mut data)?;

        if bytes_read == 0 { return Ok(()); } //End of stream

        let input: Value = match serde_json::from_slice(&data) {
            Ok(input) => input,
            Err(_) => panic!("Not Valid JSON")
        };

        println!("{:?}", input);
    }
}
