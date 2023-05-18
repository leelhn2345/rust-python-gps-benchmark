use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::time::Instant;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct RobotData {
    timestamp: f64,
    #[serde(alias = "GPS")]
    gps: Gps,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
struct Gps {
    latitude: f64,
    longitude: f64,
    altitude: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let now = Instant::now();

    // File::open("hello.json")?;

    let file_data = std::fs::read_to_string("sample.json")?;
    let data_set: Vec<RobotData> = serde_json::from_str(&file_data)?;

    // let mut gps_exists = BTreeSet::new();
    // let mut gps_exists = FxHashSet::default();
    let mut gps_exists = HashSet::new();
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
    // println!("{:#?}", data_set);

    // serde_json::to_writer_pretty(std::fs::File::create("sample-rust.json")?, &new_data)?;
    std::fs::write("sample-rust.json", serde_json::to_string_pretty(&new_data)?)?;
    let elapsed = now.elapsed().as_millis();

    println!("Elapsed: {} milliseconds", elapsed);

    let file_data = std::fs::read_to_string("sample-rust.json")?;
    let new_data: Vec<RobotData> = serde_json::from_str(&file_data)?;

    let second_now = Instant::now();
    let mut first_robot = vec![];
    let mut second_robot = vec![];
    let mut third_robot = vec![];

    let first_timestamp = 1681790361.965;
    let second_timestamp = 1681790993.761;

    for robot_data in new_data {
        if robot_data.timestamp <= first_timestamp {
            first_robot.push(robot_data);
            continue;
        }
        if robot_data.timestamp <= second_timestamp {
            second_robot.push(robot_data);
            continue;
        }
        third_robot.push(robot_data);
    }

    std::fs::write(
        "sample-1st.json",
        serde_json::to_string_pretty(&first_robot)?,
    )?;
    std::fs::write(
        "sample-2nd.json",
        serde_json::to_string_pretty(&second_robot)?,
    )?;
    std::fs::write(
        "sample-3rd.json",
        serde_json::to_string_pretty(&third_robot)?,
    )?;

    let elapsed = second_now.elapsed().as_millis();

    println!("Elapsed: {} milliseconds", elapsed);
    Ok(())
}
