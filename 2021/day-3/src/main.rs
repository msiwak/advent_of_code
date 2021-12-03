use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    println!("=================================== Part One ===================================");
    part_one()?;
    println!("=================================== Part Two ===================================");
    part_two()?;
    Ok(())
}

fn part_one() -> std::io::Result<()> {
    // Setup
    let reader = BufReader::new(File::open("input.txt")?);
    let mut lines = reader.lines();
    let first_line = lines.next().unwrap().unwrap();
    let line_length= first_line.len();
    let mut bits_counters :Vec<u32> = vec![0; line_length];
    // Gathering data
    count_line_bits(&first_line, &mut bits_counters);
    let mut line_count = 1;
    for line in lines.map(|l| l.unwrap()) {
        line_count += 1;
        count_line_bits(&line, &mut bits_counters);
    }
    // Data analysis
    let mut gamma_rate_string = String::with_capacity(line_length);
    let mut epsilon_rate_string = String::with_capacity(line_length);
    for counter in bits_counters {
        match counter {
            c if c > line_count / 2 => {
                gamma_rate_string.push('1');
                epsilon_rate_string.push('0');
            },
            _ => {
                gamma_rate_string.push('0');
                epsilon_rate_string.push('1');
            }
        }
    }
    println!("gamma_rate_string: {}", gamma_rate_string);
    println!("epsilon_rate_string: {}", epsilon_rate_string);
    let gamma_rate = usize::from_str_radix(&gamma_rate_string, 2).unwrap();
    let epsilon_rate = usize::from_str_radix(&epsilon_rate_string, 2).unwrap();
    println!("gamma_rate: {}", gamma_rate);
    println!("epsilon_rate: {}", epsilon_rate);
    println!("power: {}", gamma_rate * epsilon_rate);

    let mut test = String::from("a");
    let char = '1';
    test.push(char);

    Ok(())
}

fn part_two() -> std::io::Result<()> {
    let reader = BufReader::new(File::open("input.txt")?);
    let mut lines = reader.lines();
    let first_line = lines.next().unwrap().unwrap();
    let line_length= first_line.len();
    let mut oxygen_bits:Vec<Vec<char>> = Vec::new();
    let mut co2_bits:Vec<Vec<char>> = Vec::new();
    oxygen_bits.push(first_line.chars().collect());
    co2_bits.push(first_line.chars().collect());
    for line in lines.map(|l| l.unwrap()) {
        oxygen_bits.push(line.chars().collect());
        co2_bits.push(line.chars().collect());
    }
    let mut oxygen_value = 0;
    let mut co2_value= 0;
    for i in 0..line_length {
        if oxygen_bits.len() > 1 {
            calculate_2(&mut oxygen_bits, i, true);
            // calculate(&mut oxygen_bits, i, '1', '0');
            if oxygen_bits.len() == 1 {
                oxygen_value = get_number_from_vec(oxygen_bits.get(0).unwrap());
            }
        }
        if co2_bits.len() > 1 {
            calculate_2(&mut co2_bits, i, false);
            if co2_bits.len() == 1 {
                co2_value = get_number_from_vec(co2_bits.get(0).unwrap());
            }
        }
    }
    println!("Oxygen: {}", oxygen_value);
    println!("CO2: {}", co2_value);
    println!("Life support rating: {}", oxygen_value * co2_value);
    Ok(())
}

fn calculate_2(list: &mut Vec<Vec<char>>, index: usize, is_oxy: bool) {
    let ones_count = count_ones(list, index);
    if is_oxy {
        match ones_count {
            ones if ones * 2 >= list.len() => list.retain(|v| v[index] == '1'),
            _ => list.retain(|v| v[index] == '0'),
        }
    } else {
        match ones_count {
            ones if ones * 2 >= list.len() => list.retain(|v| v[index] == '0'),
            _ => list.retain(|v| v[index] == '1'),
        }
    }
}

fn get_number_from_vec(vec: &[char]) -> usize {
    let mut str = String::new();
    vec.iter().for_each(|c| str.push(*c));
    usize::from_str_radix(&str, 2).unwrap()
}

fn count_line_bits(line: &str, bits_counters: &mut Vec<u32>) {
    line.chars().enumerate().into_iter().for_each(|(index, ch)| bits_counters[index] += ch.to_digit(2).unwrap());
}

fn count_ones(list: &[Vec<char>], index: usize) -> usize {
    list.iter().filter(|v| v[index] == '1').count()
}

#[cfg(test)]
mod tests {
    use crate::{calculate, count_bits, get_number_from_vec};

    #[test]
    fn count_bits_test() {
        let vec_1 = vec!['0', '1', '0', '1'];
        let vec_2 = vec!['1', '1', '0', '1'];
        let vec_3 = vec!['1', '1', '1', '1'];
        let vec_4 = vec!['0', '1', '0', '0'];
        let vec_all = vec![vec_1, vec_2, vec_3, vec_4];
        assert_eq!(2, count_bits(&vec_all, 0, '1'));
        assert_eq!(2, count_bits(&vec_all, 0, '0'));
        assert_eq!(4, count_bits(&vec_all, 1, '1'));
        assert_eq!(0, count_bits(&vec_all, 1, '0'));
        assert_eq!(1, count_bits(&vec_all, 2, '1'));
        assert_eq!(3, count_bits(&vec_all, 2, '0'));
        assert_eq!(3, count_bits(&vec_all, 3, '1'));
        assert_eq!(1, count_bits(&vec_all, 3, '0'));
    }

    #[test]
    fn get_number_from_vec_test() {
        let vec_1 = vec!['0', '1', '0', '1'];
        let vec_2 = vec!['1', '1', '0', '1'];
        let vec_3 = vec!['1', '1', '1', '1'];
        let vec_4 = vec!['0', '1', '0', '0'];
        assert_eq!(5, get_number_from_vec(&vec_1));
        assert_eq!(13, get_number_from_vec(&vec_2));
        assert_eq!(15, get_number_from_vec(&vec_3));
        assert_eq!(4, get_number_from_vec(&vec_4));
    }

    #[test]
    fn calculate_test() {
        let vec_1a = vec!['0', '1', '0', '1'];
        let vec_2a = vec!['1', '1', '0', '1'];
        let vec_3a = vec!['1', '1', '1', '1'];
        let vec_4a = vec!['0', '1', '0', '0'];
        let mut vec_alla = vec![vec_1a, vec_2a, vec_3a, vec_4a];
        calculate(&mut vec_alla, 0, '1', '0');
        assert_eq!(2, vec_alla.len());
        calculate(&mut vec_alla, 1, '1', '0');
        assert_eq!(2, vec_alla.len());
        calculate(&mut vec_alla, 2, '1', '0');
        assert_eq!(1, vec_alla.len());
        let vec_1 = vec!['0', '1', '0', '1'];
        let vec_2 = vec!['1', '1', '0', '1'];
        let vec_3 = vec!['1', '1', '1', '1'];
        let vec_4 = vec!['0', '1', '0', '0'];
        let mut vec_all = vec![vec_1, vec_2, vec_3, vec_4];
        calculate(&mut vec_all, 0, '0', '1');
        assert_eq!(2, vec_all.len());
        calculate(&mut vec_all, 1, '0', '1');
        assert_eq!(2, vec_all.len());
        calculate(&mut vec_all, 2, '0', '1');
        assert_eq!(2, vec_all.len());
        calculate(&mut vec_all, 3, '0', '1');
        assert_eq!(1, vec_all.len());
    }
}
