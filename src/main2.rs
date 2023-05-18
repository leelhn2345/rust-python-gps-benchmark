use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
    str::from_utf8,
    time::Instant,
};
fn main() -> Result<(), Box<dyn Error>> {
    let now = Instant::now();
    // let mut file = BufReader::new(File::open("sample-rust.txt")?);
    // let mut string = String::new();

    // file.read_to_string(&mut string)
    //     .expect("error reading to string");

    // let file = std::fs::read("sample-rust.txt")?;
    // let string = from_utf8(&file)?.to_string();

    let string = std::fs::read("sample-rust.txt")?;

    for line in string.lines() {
        println!("{}", line);
    }

    println!("elapsed time: {}", now.elapsed().as_millis());

    Ok(())
}
