use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    println!("=================================== Part One ===================================");
    part_one()?;
    println!("=================================== Part Two ===================================");
    part_two()
}

fn part_one() -> std::io::Result<()> {
    let (random_numbers, boards) = read_input()?;
    Ok(())
}

fn part_two() -> std::io::Result<()> {
    let reader = BufReader::new(File::open("input.txt")?);
    for line in reader.lines().map(|l| l.unwrap()) {

    }
    Ok(())
}

struct Board {
    board: Vec<Vec<(u8, bool)>>,
    next_row: usize,
}

impl Board {
    fn new() -> Self {
        Board {
            board: vec![vec![(0, false); 5]; 5],
            next_row: 0,
        }
    }

    fn add_row(mut self, row: &[u8]) {
        row.iter().enumerate().for_each(|(i, v)| self.board[self.next_row][i] = (*v, false));
    }
}

fn read_input() -> std::io::Result<(Vec<u8>, Vec<Board>)> {
    let reader = BufReader::new(File::open("input.txt")?);
    let line =reader.lines().next().unwrap().unwrap();
    let numbers :Vec<u8> = line
        .split(' ').collect::<Vec<&str>>()
        .iter().map(|e| e.parse::<u8>().unwrap()).collect();
    Ok((numbers, Vec::new()))
}

#[cfg(test)]
mod tests {
    use crate::Board;

    #[test]
    fn board_add_test() {
        let board = Board::new();
        board.add_row(&[1u8, 2, 3, 4, 5]);
    }
}

