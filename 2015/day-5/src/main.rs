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
    let mut nice_counter = 0;
    for line in reader.lines().map(|l| l.unwrap()) {
        let mut validators :Vec<Box<dyn StringValidation>> = Vec::new();
        validators.push(Box::new(VowelValidator::new()));
        validators.push(Box::new(SameLetterValidator::new()));
        validators.push(Box::new(NoConsecutiveValidator::new()));
        for c in line.chars() {
            for v in &mut validators {
                v.as_mut().add(c);
            }
        }
        // validators.iter().all(|v| v.is_valid()).then(|| nice_counter += 1);
        let mut all_valid = true;
        if !validators[0].is_valid() {
            println!("Line {} not valid for VowelValidator", line);
            all_valid = false;
        }
        if !validators[1].is_valid() {
            println!("Line {} not valid for SameLetterValidator", line);
            all_valid = false;
        }
        if !validators[2].is_valid() {
            println!("Line {} not valid for NoConsecutiveValidator", line);
            all_valid = false;
        }
        if all_valid {
            println!("valid: {}", line);
            nice_counter += 1;
        }
    }
    println!("Nice count: {}", nice_counter);
    Ok(())
}

fn part_two() -> std::io::Result<()> {
    let reader = BufReader::new(File::open("input.txt")?);
    for line in reader.lines().map(|l| l.unwrap()) {

    }
    Ok(())
}

trait StringValidation {
    fn add(&mut self, letter: char);
    fn is_valid(&self) -> bool;
}

struct VowelValidator {
    check: bool,
    valid: bool,
    vowel_count: u8,
    vowels: Vec<char>,
    expected_vowel_count: u8,
}

impl VowelValidator {
    fn new() -> Self {
        VowelValidator {
            check: true,
            valid: false,
            vowel_count: 0,
            vowels: vec!['a', 'e', 'i', 'o', 'u'],
            expected_vowel_count: 3,
        }
    }
}

impl StringValidation for VowelValidator {
    fn add(&mut self, letter: char) {
        if self.check {
            if self.vowels.contains(&letter) {
                self.vowel_count += 1;
            }
            if self.vowel_count == self.expected_vowel_count {
                self.valid = true;
                self.check = false;
            }
        }
    }

    fn is_valid(&self) -> bool {
        self.valid
    }
}

struct SameLetterValidator {
    check: bool,
    valid: bool,
    previous_char: Option<char>,
}

impl SameLetterValidator {
    fn new() -> Self {
        SameLetterValidator {
            check: true,
            valid: false,
            previous_char: None,
        }
    }
}

impl StringValidation for SameLetterValidator {
    fn add(&mut self, letter: char) {
        if self.check {
            if self.previous_char == Some(letter) {
                self.valid = true;
                self.check = false;
            }
            self.previous_char = Some(letter);
        }
    }

    fn is_valid(&self) -> bool {
        self.valid
    }
}

struct NoConsecutiveValidator {
    check: bool,
    valid: bool,
    previous_char_code: u8,
}

impl NoConsecutiveValidator {
    fn new() -> Self {
        NoConsecutiveValidator {
            check: true,
            valid: true,
            previous_char_code: 0,
        }
    }
}

impl StringValidation for NoConsecutiveValidator {
    fn add(&mut self, letter: char) {
        if self.check {
            if letter as u8 - 1 == self.previous_char_code {
                self.valid = false;
                self.check = false;
            }
            self.previous_char_code = letter as u8;
        }
    }

    fn is_valid(&self) -> bool {
        self.valid
    }
}

#[cfg(test)]
mod tests {
    use crate::{NoConsecutiveValidator, SameLetterValidator, StringValidation, VowelValidator};

    #[test]
    fn test_same_letter_validator() {
        let mut validator = SameLetterValidator::new();
        assert!(!validator.is_valid());
        validator.add('a');
        validator.add('a');
        assert!(validator.is_valid());
    }

    #[test]
    fn test_no_consecutive_validator() {
        let mut validator = NoConsecutiveValidator::new();
        assert!(validator.is_valid());
        validator.add('a');
        validator.add('b');
        assert!(!validator.is_valid());
    }

    #[test]
    fn test_vowel_validator() {
        let mut validator = VowelValidator::new();
        assert!(!validator.is_valid());
        validator.add('a');
        validator.add('b');
        validator.add('u');
        validator.add('b');
        validator.add('e');
        assert!(validator.is_valid());

    }
}
