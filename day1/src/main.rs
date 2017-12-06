use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "input";
    let mut f = File::open(filename).expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("error during reading the file");
    let contents: Vec<u32> = contents.chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    let mut sum = 0;

    for i in 0..contents.len() {
        if contents[i] == contents[(i + contents.len() / 2) % contents.len()] {
            sum += contents[i];
        }
    }
    println!("{}", sum);
}
