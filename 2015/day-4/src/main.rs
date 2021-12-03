use md5;

fn main() -> std::io::Result<()> {
    println!("=================================== Part One ===================================");
    part_one()?;
    println!("=================================== Part Two ===================================");
    part_two()
}

const INPUT :&str = "bgvyzdsv";

fn part_one() -> std::io::Result<()> {
    let mut counter = 0u64;
    loop {
        let hash = format!("{:x}", md5::compute(format!("{}{}", INPUT, counter)));
        if hash.starts_with("00000") {
            println!("Hash: {}, number: {}", hash, counter);
            break;
        }
        counter += 1;
    }
    Ok(())
}

fn part_two() -> std::io::Result<()> {
    let mut counter = 0u64;
    loop {
        let hash = format!("{:x}", md5::compute(format!("{}{}", INPUT, counter)));
        if hash.starts_with("000000") {
            println!("Hash: {}, number: {}", hash, counter);
            break;
        }
        counter += 1;
    }
    Ok(())
}
