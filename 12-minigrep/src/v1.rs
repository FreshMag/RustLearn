
use std::fs;

pub fn v1() {
    let args: Vec<String> = std::env::args().collect();

    // dbg!(args);

    let query = &args[1];
    let file_path = &args[2];

    println!("Query: {}\nFilename: {}", query, file_path);

    let content = fs::read_to_string(file_path).expect("Should have been able to read that file");

    println!("Content:\n{}", content)
}
