use aoc2021::read_input;

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

fn part2(data: Vec<u32>) {
    let mut count: u32 = 0;

    for i in 0..data.len() - 3 {
        if data[i + 3] > data[i] {
            count += 1;
        }
    }
    println!("part2: {count}");
}

fn main() {
    let input = read_input();
    
    // parse input
    let data: Vec<u32> = input
        .iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    part1(data.clone());
    part2(data.clone());
}