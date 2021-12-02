use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    part_one()?;
    println!("================================================================================");
    part_two()
}

fn part_two() -> std::io::Result<()> {
    let reader = BufReader::new(File::open("input.txt")?);
    let mut depth= 0u32;
    let mut distance = 0u32;
    let mut aim = 0u32;
    for line in reader.lines().map(|l| l.unwrap()) {
        match Move::from(&line) {
            Move::DOWN(value) => aim += value,
            Move::FORWARD(value) => {
                distance += value;
                depth += aim * value;
            },
            Move::UP(value) => aim -= value,
        }
    }
    println!("Depth: {}", depth);
    println!("Distance: {}", distance);
    println!("Aim: {}", aim);
    println!("Multiply: {}" ,depth * distance);
    Ok(())
}

fn part_one() -> std::io::Result<()> {
    let reader = BufReader::new(File::open("input.txt")?);
    let mut depth= 0u32;
    let mut distance = 0u32;
    for line in reader.lines().map(|l| l.unwrap()) {
        match Move::from(&line) {
            Move::DOWN(value) => depth += value,
            Move::FORWARD(value) => distance += value,
            Move::UP(value) => depth -= value,
        }
    }
    println!("Depth: {}", depth);
    println!("Distance: {}", distance);
    println!("Multiply: {}" ,depth * distance);
    Ok(())
}

enum Move {
    DOWN(u32),
    FORWARD(u32),
    UP(u32),
}

impl Move {
    fn from(input: &str) -> Self {
        let input_parts :Vec<&str> = input.split(" ").collect();
        let (direction, distance) = (input_parts[0], input_parts[1]);
        match direction {
            "down" => Move::DOWN(distance.parse::<u32>().unwrap()),
            "forward" => Move::FORWARD(distance.parse::<u32>().unwrap()),
            "up" => Move::UP(distance.parse::<u32>().unwrap()),
            other => panic!("Direction not known: {}", other),
        }
    }
}
