extern crate geojson;

use geojson::{GeoJson, Feature};
use std::io::{BufRead, BufReader};
use std::iter::Iterator;
use std::fs::File;
use std::str;
use std::process;

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

    pub fn process(doc: &Feature) -> i64 {
        1
    }
}

impl Iterator for Indexer {
    type Item = Feature;

    fn next(&mut self) -> Option<Feature> {
        let mut data = Vec::new();

        loop {
            match self.stream.read_until(b'\n', &mut data).unwrap() {
                0 => { return None; },
                _ => {
                    //Skip empty lines
                    if data.len() == 1 as usize && data[0] == 10 as u8 { continue; }

                    let input: GeoJson = match str::from_utf8(&data) {
                        Ok(data) => match data.parse::<GeoJson>() {
                            Ok(input) => input,
                            Err(_) => panic!("Not Valid GeoJSON")
                        },
                        Err(_) => {
                            println!("Invalid UTF8 Data");
                            process::exit(1);
                        }
                    };

                    match input {
                        GeoJson::Feature(feat) => { return Some(feat) },
                        _ => {
                            println!("All Data must be GeoJSON Feature Types");
                            process::exit(1);
                        }
                    };
                }
            }

            data.clear();
        }
    }
}
