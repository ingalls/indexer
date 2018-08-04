extern crate geojson; 
extern crate serde_json;

const MAX_TEXT_SYNONYMS: u8 = 10;

use serde_json::Value;
use geojson::{GeoJson, Feature};
use std::io::{BufRead, BufReader};
use std::iter::Iterator;
use std::fs::File;
use std::str;
use std::process;

pub struct Doc();

impl Doc {
    pub fn is_valid(doc: &Feature) -> Result<(), String> {
        let id = match &doc.id {
            Some(Value::Number(id)) => {
                match id.as_u64() {
                    None => { return Err(String::from("DATA ERROR: doc must have intenger id")) },
                    Some(id) => id
                }
            },
            _ => { return Err(String::from("DATA ERROR: doc has no id")); },
        };

        if doc.geometry.is_none() {
            return Err(format!("DATA ERROR: doc has no geometry on id {}", &id));
        }

        match &doc.properties {
            None => { return Err(format!("DATA ERROR: doc has no properties on id {}", &id)); },
            Some(props) => {
                match props.get("carmen:text") {
                    None => { return Err(format!("DATA ERROR: doc has no carmen:text on id {}", &id)); },
                    Some(text) => {
                        match text.as_str() {
                            None => { return Err(format!("DATA ERROR: doc has non-string value for carmen:text on id {}", &id)); },
                            Some(text) => {
                                if props.contains_key("carmen:addressnumber") || props.contains_key("carmen:rangetype") {
                                    let syns: Vec<&str> = text.split(",").collect();

                                    if syns.len() > MAX_TEXT_SYNONYMS as usize {
                                        return Err(format!("DATA ERROR: doc has > {} synonyms on id {}", MAX_TEXT_SYNONYMS, &id));
                                    }
                                    
                                }
                            }
                        }
                    }
                } 

                if props.contains_key("carmen:geocoder_stack") && !props.get("carmen:geocoder_stack").unwrap().is_string() {
                    return Err(format!("DATA ERROR: doc has non-string value for carmen:geocoder_stack"));
                }

                /*
                if (doc.geometry.type === 'Polygon' || doc.geometry.type === 'MultiPolygon') {
                    // check for Polygons or Multipolygons with too many vertices
                    let vertices = 0;
                    let ringCount;
                    if (doc.geometry.type === 'Polygon') {
                        ringCount = doc.geometry.coordinates.length;
                        for (let i = 0; i < ringCount; i++) {
                            vertices += doc.geometry.coordinates[i].length;
                        }
                    } else {
                        const polygonCount = doc.geometry.coordinates.length;
                        for (let k = 0; k < polygonCount; k++) {
                            ringCount = doc.geometry.coordinates[k].length;
                            for (let j = 0; j < ringCount; j++) {
                                vertices += doc.geometry.coordinates[k][j].length;
                            }
                        }
                    }
                    if (vertices > 50000) {
                        throw Error('Polygons may not have more than 50k vertices. Simplify your polygons, or split the polygon into multiple parts on id:' + doc.id);
                */


            }
        };

        Ok(())
    }
}

pub struct Indexer {
    zoom: u8,
    stream: BufReader<File>
}

impl Indexer {
    pub fn new(path: String, zoom: u8) -> Self {
        if zoom > 14 || zoom < 1 {
            println!("INDEX ERROR: zoom must be greater than 0 less than 15--- zoom was {}", zoom);
            process::exit(1);
        }

        Indexer {
            zoom: zoom,
            stream: BufReader::new(File::open(path).unwrap()),
        }
    }

    pub fn process(doc: &Feature) -> i64 {
        match Doc::is_valid(&doc) {
            Err(err) => {
                /*
                 * TODO Don't panic within process as it can mess up other 
                 * threads - pass an error up to the batch controller
                 */
                println!("{}", &err);
                process::exit(1);
            }
            _ => ()
        };

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
            };

            data.clear();
        }
    }
}
