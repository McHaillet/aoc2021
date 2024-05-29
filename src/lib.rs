use std::env;
use std::fs;

pub fn read_input() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    println!("Input file: {}", file_path);
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    contents
        .split('\n')
        .map(|v: &str| v.to_string())
        .collect()
}
