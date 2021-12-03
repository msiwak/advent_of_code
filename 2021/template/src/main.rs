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
    for line in reader.lines().map(|l| l.unwrap()) {

    }
    Ok(())
}

fn part_two() -> std::io::Result<()> {
    let reader = BufReader::new(File::open("input.txt")?);
    for line in reader.lines().map(|l| l.unwrap()) {

    }
    Ok(())
}
