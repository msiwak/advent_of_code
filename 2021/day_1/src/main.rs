use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    part_one()?;
    part_two()
}

fn part_two() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut increase_count = 0;
    let mut decrease_count = 0;
    let mut sum = Sum::new(3);
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let current_depth = line.parse::<u32>().unwrap();
        let new_sum = sum.next_sum(current_depth);
        if sum.can_compare {
            match new_sum.sum() {
                sum_new if sum_new < sum.sum() => decrease_count += 1,
                sum_new if sum_new > sum.sum() => increase_count += 1,
                _ => (),
            }
        }
        sum = new_sum;
    }
    println!("Increased {} times", increase_count);
    println!("Decreased {} times", decrease_count);
    Ok(())
}

fn part_one() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut previous_depth = 0;
    let mut increase_count = 0;
    let mut decrease_count = 0;
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let current_depth = line.parse::<u32>().unwrap();
        if previous_depth != 0 {
            match current_depth {
                a if a < previous_depth => decrease_count += 1,
                a if a > previous_depth => increase_count += 1,
                _ => (),
            }
        }
        previous_depth = current_depth;
    }
    println!("Increased {} times", increase_count);
    println!("Decreased {} times", decrease_count);
    Ok(())
}

struct Sum {
    number_of_elements: usize,
    can_compare: bool,
    current_numbers: usize,
    numbers: Vec<u32>,
}

impl Sum {
    fn new(number_of_elements: usize) -> Self {
        Sum {
            current_numbers: 0,
            can_compare: false,
            number_of_elements,
            numbers: Vec::with_capacity(number_of_elements),
        }
    }

    fn sum(&self) -> u32 {
        self.numbers.iter().sum()
    }

    fn next_sum(&self, new_number: u32) -> Self {
        let mut new_numbers = Vec::with_capacity(self.number_of_elements);
        match self.current_numbers {
            cn if 0 < cn && cn < self.number_of_elements => {
                for i in 0..cn {
                    new_numbers.push(self.numbers[i]);
                }
            },
            cn if cn == self.number_of_elements => {
                for i in 1..cn {
                    new_numbers.push(self.numbers[i]);
                }
            }
            _ => (),
        }
        new_numbers.push(new_number);
        let new_current_number = match self.current_numbers {
            a if a < self.number_of_elements => self.current_numbers + 1,
            _ => self.current_numbers,
        };
        let new_can_compare = new_current_number == self.number_of_elements;

        Sum {
            number_of_elements: self.number_of_elements,
            can_compare: new_can_compare,
            current_numbers: new_current_number,
            numbers: new_numbers,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Sum;

    #[test]
    fn test_new() {
        let sum = Sum::new(3);
        assert_eq!(0, sum.current_numbers);
        assert_eq!(3, sum.number_of_elements);
        assert_eq!(false, sum.can_compare);
        assert_eq!(vec![0; 0], sum.numbers);
        assert_eq!(0, sum.sum());
    }

    #[test]
    fn test_first_num() {
        let sum = Sum::new(3);
        let new_sum = sum.next_sum(1);
        assert_eq!(1, new_sum.current_numbers);
        assert_eq!(3, new_sum.number_of_elements);
        assert_eq!(false, new_sum.can_compare);
        assert_eq!(vec![1; 1], new_sum.numbers);
        assert_eq!(1, new_sum.sum());
    }

    #[test]
    fn test_second_num() {
        let sum = Sum::new(3);
        let new_sum = sum.next_sum(1);
        let new_sum = new_sum.next_sum(2);
        assert_eq!(2, new_sum.current_numbers);
        assert_eq!(3, new_sum.number_of_elements);
        assert_eq!(false, new_sum.can_compare);
        assert_eq!(vec![1, 2], new_sum.numbers);
        assert_eq!(3, new_sum.sum());
    }

    #[test]
    fn test_third_num() {
        let sum = Sum::new(3);
        let new_sum = sum.next_sum(1);
        let new_sum = new_sum.next_sum(2);
        let new_sum = new_sum.next_sum(3);
        assert_eq!(3, new_sum.current_numbers);
        assert_eq!(3, new_sum.number_of_elements);
        assert_eq!(true, new_sum.can_compare);
        assert_eq!(vec![1, 2, 3], new_sum.numbers);
        assert_eq!(6, new_sum.sum());
    }

    #[test]
    fn test_fourth_num() {
        let sum = Sum::new(3);
        let new_sum = sum.next_sum(1);
        let new_sum = new_sum.next_sum(2);
        let new_sum = new_sum.next_sum(3);
        let new_sum = new_sum.next_sum(4);
        assert_eq!(3, new_sum.current_numbers);
        assert_eq!(3, new_sum.number_of_elements);
        assert_eq!(true, new_sum.can_compare);
        assert_eq!(vec![2, 3, 4], new_sum.numbers);
        assert_eq!(9, new_sum.sum());
    }
}
