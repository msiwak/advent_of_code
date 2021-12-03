use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    println!("=================================== Part One ===================================");
    part_one()?;
    println!("=================================== Part Two ===================================");
    part_two()
}

fn part_one() -> std::io::Result<()> {
    let reader = BufReader::new(File::open("input.txt")?);
    let mut floor = 0_i32;
    for line in reader.lines().map(|l| l.unwrap()) {
        for c in line.chars() {
            match c {
                '(' => floor += 1,
                _ => floor -= 1,
            }
        }
    }
    println!("Floor: {}", floor);
    Ok(())
}

fn part_two() -> std::io::Result<()> {
    let reader = BufReader::new(File::open("input.txt")?);
    let mut floor = 0_i32;
    for line in reader.lines().map(|l| l.unwrap()) {
        for (index, c) in line.chars().enumerate() {
            match c {
                '(' => floor += 1,
                _ => floor -= 1,
            }
            if floor == -1 {
                println!("Character index: {}", index);
                break;
            }
        }
    }
    Ok(())
}
