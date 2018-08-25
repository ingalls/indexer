extern crate geojson; 
extern crate serde_json;

const MAX_TEXT_SYNONYMS: u8 = 10;
const MAX_VERTICES: u32 = 50000;

use serde_json::Value;
use geojson::{GeoJson, Feature};
use std::io::{BufRead, BufReader};
use std::iter::Iterator;
use std::fs::File;
use std::str;
use std::process;

pub struct Doc();

impl Doc {
    pub fn standardize(doc: &mut Feature) -> Result<(), String> {
        /*
        if (doc.geometry && (doc.geometry.type === 'Polygon' || doc.geometry.type === 'MultiPolygon')) {
            doc = rewind(doc);
        }

        // Requires MultiPolygons to be in proper winding order
        runChecks(doc, zoom);

        let tiles = [];
        if (doc.geometry.type === 'GeometryCollection' && !doc.properties['carmen:zxy']) {
            doc.properties['carmen:zxy'] = [];
            tiles = [];
            for (let feat_it = 0; feat_it < doc.geometry.geometries.length; feat_it++) {
                tiles = tiles.concat(cover.tiles(doc.geometry.geometries[feat_it], { min_zoom: zoom, max_zoom: zoom }));
            }
            tiles.forEach((tile) => {
                doc.properties['carmen:zxy'].push(tile[2] + '/' + tile[0] + '/' + tile[1]);
            });
        } else if (!doc.properties['carmen:zxy']) {
            tiles = cover.tiles(doc.geometry, { min_zoom: zoom, max_zoom: zoom });
            doc.properties['carmen:zxy'] = [];
            tiles.forEach((tile) => {
                doc.properties['carmen:zxy'].push(tile[2] + '/' + tile[0] + '/' + tile[1]);
            });
        } else {
            doc.properties['carmen:zxy'].forEach((tile) => {
                tile = tile.split('/');
                tiles.push([tile[1], tile[2], tile[0]]);
            });
        }

        // if an outlier is detected in address numbers for example in [1,2,3,5000], 5000 is considered an outlier, then drop interpolation for it
        if (doc.properties['carmen:addressnumber'] && doc.geometry.type === 'GeometryCollection') {
            if (isOutlierDetected(doc.properties['carmen:addressnumber'])) {
                const interpolationProperties = ['carmen:lfromhn','carmen:ltohn', 'carmen:parityr', 'carmen:rfromhn','carmen:rtohn', 'carmen:parityl'];

                // set interpolation properties values to null, for example: "carmen:parityr":[["O", "O" ,null ,null ,null], null] would become "carmen:parityr":[[null, null ,null ,null ,null], null]
                interpolationProperties.forEach((p) => {
                    if (doc.properties[p]) {
                        for (let i = 0; i < doc.properties[p].length; i++) {
                            if (doc.properties[p][i] != null) {
                                doc.properties[p][i] = doc.properties[p][i].fill(null);
                            }
                            doc.properties[p][i] = doc.properties[p][i];
                        }
                    }
                });
            }
        }

        if (!doc.properties['carmen:center'] || !verifyCenter(doc.properties['carmen:center'], tiles)) {
            console.warn('carmen:center did not fall within the provided geometry for %s (%s). Calculating new point on surface.',
                doc.id, doc.properties['carmen:text']);
            doc.properties['carmen:center'] = centroid(doc.geometry).geometry.coordinates;
            if (!verifyCenter(doc.properties['carmen:center'], tiles)) {
                throw Error('Invalid carmen:center provided, and unable to calculate corrected centroid. Verify validity of doc.geometry for doc id:' + doc.id);
            } else {
                console.warn('new: carmen:center: ', doc.properties['carmen:center']);
                console.warn('new: zxy:    ', doc.properties['carmen:zxy']);
            }
        }

        // Standardize all addresses to GeometryCollections
        doc = feature.addrTransform(doc);

        if (!doc.bbox && (doc.geometry.type === 'MultiPolygon' || doc.geometry.type === 'Polygon')) {
            const boundingBox = extent(doc.geometry);
            const bboxWidth = boundingBox[2] - boundingBox[0];
            if (bboxWidth < 180) {
                doc.bbox = boundingBox;
            } else {
                doc.bbox = bbox.crossAntimeridian(doc.geometry, boundingBox);
            }
        }

        // zxy must be set at this point
        if (!doc.properties['carmen:zxy']) {
            throw Error('doc.properties[\'carmen:zxy\'] undefined, failed indexing, doc id:' + doc.id);
        }

        // Limit carmen:zxy length to 10000 covers.
        // Stopgap: If covers exceed this amount drop covers furthest from
        // the center of this feature. This breaks forward geocode stacking
        // for any of the dropped covers.
        if (doc.properties['carmen:zxy'] && doc.properties['carmen:zxy'].length > 10000) {
            console.warn('carmen:zxy exceeded 10000, truncating to 10000 (doc id: %s, text: %s)', doc.id, doc.properties['carmen:text']);
            const centerCover = center2zxy(doc.properties['carmen:center'], zoom);
            const sortedCovers = doc.properties['carmen:zxy'].slice(0);
            sortedCovers.sort((a, b) => {
                a = a.split('/');
                b = b.split('/');
                const aDist = Math.sqrt(Math.pow(centerCover[1] - parseInt(a[1],10),2) + Math.pow(centerCover[2] - parseInt(a[2],10),2));
                const bDist = Math.sqrt(Math.pow(centerCover[1] - parseInt(b[1],10),2) + Math.pow(centerCover[2] - parseInt(b[2],10),2));
                return aDist - bDist;
            });
            doc.properties['carmen:zxy'] = sortedCovers.slice(0,10000);
        }
        return doc;
        */


        Ok(())
    }

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
                    return Err(format!("DATA ERROR: doc has non-string value for carmen:geocoder_stack on id {}", &id));
                }
            }
        };

        match &doc.geometry {
            None => { return Err(format!("DATA ERROR: doc has no geometry on id {}", &id)); },
            Some(geom) => {

                match &geom.value {
                    geojson::Value::Polygon(poly) => {
                        let mut verts: u32 = 0;
                        for vec in poly {
                            verts += vec.len() as u32;
                        }

                        if verts > MAX_VERTICES {
                            return Err(format!("DATA ERROR: doc exceeds {} vertices max on id {}", &MAX_VERTICES, &id));
                        }
                    },
                    geojson::Value::MultiPolygon(mpoly) => {
                        let mut verts: u32 = 0;
                        for vecs in mpoly {
                            for vec in vecs {
                                verts += vec.len() as u32;
                            }
                        }

                        if verts > MAX_VERTICES {
                            return Err(format!("DATA ERROR: doc exceeds {} vertices max on id {}", &MAX_VERTICES, &id));
                        }
                    },
                    _ => ()
                }
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

                    data.clear();

                    match input {
                        GeoJson::Feature(feat) => { return Some(feat) },
                        _ => {
                            println!("All Data must be GeoJSON Feature Types");
                            process::exit(1);
                        }
                    };
                }
            };
        }
    }
}
