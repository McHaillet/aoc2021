use aoc2021::read_input;

fn part1(data: Vec<String>) {
    // count the bits first
    let mut c: [u32; 16] = [0; 16];
    for line in &data {
        for (i, bit) in line.chars().rev().enumerate() {
            if bit == '1' {
                c[i] += 1
            }
        }
    }
    // convert to decimal numbers
    let base: u32 = 2;
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    let length: usize = data.len();
    let n_bits: usize = data[0].chars().count();
    for i in 0..16 {
        if i >= n_bits {
            break
        }
        if c[i] * 2 > (length as u32) {
            gamma += base.pow(i as u32);
        } else {
            epsilon += base.pow(i as u32);
        }
    }
    // println!("{:#b}", gamma);
    // println!("{:#b}", epsilon);
    println!("part1: {}", gamma * epsilon)
}

fn main() {
    let input: Vec<String> = read_input();

    part1(input.clone());
    // part2(input.clone());
}