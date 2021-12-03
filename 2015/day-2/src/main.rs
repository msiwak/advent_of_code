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
    let mut total_area = 0;
    for line in reader.lines().map(|l| l.unwrap()) {
        total_area += calculate_area(get_dimensions(&line));
    }
    println!("Total area: {}", total_area);
    Ok(())
}

fn part_two() -> std::io::Result<()> {
    let reader = BufReader::new(File::open("input.txt")?);
    let mut total_length = 0u32;
    for line in reader.lines().map(|l| l.unwrap()) {
        total_length += calculate_ribbon(get_dimensions(&line))
    }
    println!("Total length: {}", total_length);
    Ok(())
}

fn get_dimensions(input: &str) -> (u32, u32, u32) {
    let dim_vec :Vec<u32> = input.split('x').map(|d| d.parse::<u32>().unwrap()).collect();
    (dim_vec[0], dim_vec[1], dim_vec[2])
}

fn calculate_area((l, w, h): (u32, u32, u32)) -> u32 {
    let mut areas = Vec::with_capacity(3);
    areas.push(2 * l * w);
    areas.push(2 * w * h);
    areas.push(2 * h * l);
    areas.sort();
    areas.push(areas[0]/2);
    areas.iter().sum()
}

fn calculate_ribbon((l, w, h): (u32, u32, u32)) -> u32 {
    let mut tmp = [l, w, h];
    tmp.sort();
    2 * tmp[0] + 2 * tmp[1] + l * w * h
}
