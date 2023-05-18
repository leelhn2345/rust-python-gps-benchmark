use serde::{Deserialize, Serialize};

use rustc_hash::FxHashSet;
use std::collections::BTreeSet;
use std::io::BufReader;
use std::time::Instant;
use std::{error::Error, fs::File};

#[derive(Serialize, Deserialize)]
struct RobotData {
    timestamp: f64,
    #[serde(alias = "GPS")]
    gps: Gps,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
struct Gps {
    latitude: f64,
    longitude: f64,
    altitude: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let now = Instant::now();
    let data_set: Vec<RobotData> =
        serde_json::from_reader(BufReader::new(File::open("sample.json")?))?;

    let mut gps_exists = BTreeSet::new();
    let mut new_data = vec![];
    for data in data_set {
        let gps = data.gps;
        let gps_tuple = (gps.latitude.to_string(), gps.longitude.to_string());
        if gps_exists.contains(&gps_tuple) {
            continue;
        } else {
            new_data.push(data);
            gps_exists.insert(gps_tuple);
        }
    }

    serde_json::to_writer_pretty(File::create("sample-rust.json")?, &new_data)?;

    let elapsed = now.elapsed().as_millis();

    println!("Elapsed: {} milliseconds", elapsed);
    Ok(())
}
