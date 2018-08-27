use std::fs::File;
use std::io::prelude::*;

pub fn read_file(fileName: &str) -> Result<String, &'static str> {
    let mut file = match File::open(fileName) {
        Err(e) => { return Err("IO Error") },
        Ok(f) => f
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(e) => { return Err("IO Error") },
        Ok(_) => {}
    };

    Ok(contents)
}
