use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use util::pixel::Pixel;

pub fn write_ppm(name: &str, width: usize, height: usize, pixels: &Vec<Pixel>) {
    let path = Path::new(name);
    let display = path.display();
    println!("{}", display);

    let output = format!("P6\n {} {}", width, height);

    // write out the data
    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(why) => panic!("Open file failed: {}", why),
    };

    match file.write_all(output.as_bytes()) {
        Ok(_)    => (),
        Err(why) => panic!("Cannot write file: {}", why),
    }
}