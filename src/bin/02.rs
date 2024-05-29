use aoc2021::read_input;

fn part1(data: Vec<String>) {
    let (mut depth, mut horizontal): (i32, i32) = (0, 0);
    for message in &data {
        let mut iter = message.splitn(2, ' ');
        let command = iter.next().unwrap();
        let size: i32 = iter.next().unwrap().parse()
            .expect("Could not parse 2nd part of line to an int");
        match command {
            "forward" => {
                horizontal += size;
            },
            "down" => {
                depth += size;
            },
            "up" => {
                depth -= size;
            },
            &_ => break,
        }
    }
    println!("part1: {}", depth * horizontal);
}

fn part2(data: Vec<String>) {
    let (
        mut depth, 
        mut horizontal, 
        mut aim
    ): (i32, i32, i32) = (0, 0, 0);
    
    for message in &data {
        let mut iter = message.splitn(2, ' ');
        let command = iter.next().unwrap();
        let size: i32 = iter.next().unwrap().parse()
            .expect("Could not parse 2nd part of line to an int");
        match command {
            "forward" => {
                horizontal += size;
                depth += aim * size;
            },
            "down" => {
                aim += size;
            },
            "up" => {
                aim -= size;
            },
            &_ => break,
        }
    }
    println!("part2: {}", depth * horizontal);
}

fn main() {
    let input: Vec<String> = read_input();

    part1(input.clone());
    part2(input.clone());
}