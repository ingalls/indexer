#[macro_use] extern crate clap;
extern crate serde_json;

use std::io::{BufRead, BufReader};
use clap::App;

fn main() {
    let cli_cnf = load_yaml!("cli.yml");
    let matched = App::from_yaml(cli_cnf).get_matches();

}

fn distribute_input(mut stream: TcpStream) -> Result<(), Error> {
    let mut data = Vec::new();
    let mut stream = BufReader::new(stream);

    loop {
        data.clear();

        let bytes_read = stream.read_until(b'\n', &mut data)?;

        if bytes_read == 0 { return Ok(()); } //End of stream

        let input = serde_json::from_slice(&data)?;

        println!("{}", input);
    }
}
