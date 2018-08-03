#[macro_use] extern crate clap;
extern crate serde_json;

use std::io::{BufRead, BufReader};
use clap::App;

fn main() {
    let cli_cnf = load_yaml!("cli.yml");
    let matched = App::from_yaml(cli_cnf).get_matches();
}
