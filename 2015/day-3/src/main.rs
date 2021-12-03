use std::collections::HashSet;
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
    let mut position = (0, 0);
    let mut houses_set = HashSet::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        for c in line.chars() {
            match c {
                '>' => position.0 += 1,
                '<' => position.0 -= 1,
                '^' => position.1 += 1,
                _ => position.1 -= 1,
            }
            houses_set.insert(position);

        }
    }
    println!("Unique houses: {}", houses_set.len());
    Ok(())
}

fn part_two() -> std::io::Result<()> {
    let reader = BufReader::new(File::open("input.txt")?);
    let mut positions = [(0, 0), (0, 0)];
    let mut current_santa = 0;
    let mut houses_set = HashSet::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        for c in line.chars() {
            match c {
                '>' => positions[current_santa].0 += 1,
                '<' => positions[current_santa].0 -= 1,
                '^' => positions[current_santa].1 += 1,
                _ => positions[current_santa].1 -= 1,
            }
            houses_set.insert(positions[current_santa]);
            if current_santa == 0 {
                current_santa = 1;
            } else {
                current_santa = 0;
            }
        }
    }
    println!("Unique houses: {}", houses_set.len());
    Ok(())
}
