use std::env;
use std::fs;

fn read_input() -> Vec<String> {
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

fn part1(data: Vec<u32>) {
    let mut y: u32;
    y = data[0];
    let mut count: u32 = 0;

    for x in data[1..].iter() {
        if *x > y {
            count += 1;
        }
        y = *x;
    }
    println!("part1: {count}");
}

fn main() {
    let input = read_input();
    
    // parse input
    let data: Vec<u32> = input
        .iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    part1(data);
}