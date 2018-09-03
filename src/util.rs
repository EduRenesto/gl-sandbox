use std::fs::File;
use std::io::prelude::*;

pub fn read_file(file_name: &str) -> Result<String, &'static str> {
    let mut file = match File::open(file_name) {
        Err(_) => { return Err("IO Error") },
        Ok(f) => f
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(_) => { return Err("IO Error") },
        Ok(_) => {}
    };

    Ok(contents)
}
