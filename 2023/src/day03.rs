use std::collections::HashSet;
use std::fmt;
use std::fs::read_to_string;

#[derive(Clone, PartialEq)]
enum PosType {
    Empty,
    GearSymbol,
    OtherSymbol,
    Digit(usize)
}

impl PosType {
    fn is_symbol(&self) -> bool {
        match self {
            PosType::GearSymbol => true,
            PosType::OtherSymbol => true,
            _ => false
        }
    }
}

impl fmt::Debug for PosType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PosType::Empty => write!(f, "."),
            PosType::GearSymbol => write!(f, "*"),
            PosType::OtherSymbol => write!(f, "#"),
            PosType::Digit(n) => write!(f, "{}", n)
        }
    }
}

struct Matrix {
    data: [Vec<PosType>; 3]
}

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.data.iter() {
            for pos in row.iter() {
                write!(f, "{:?}", pos)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Matrix {
    fn new(line_length: usize) -> Self {
        Matrix {
            data: [
                vec![PosType::Empty; line_length + 2],
                vec![PosType::Empty; line_length + 2],
                vec![PosType::Empty; line_length + 2]
            ]
        }
    }

    fn write_line(&mut self, line: &str) -> Vec<u32> {
        let mut number_mapping = vec![];
        let mut current_number_idx = 0;
        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if number_mapping.len() == current_number_idx {
                    number_mapping.push(c.to_digit(10).unwrap() as u32);
                } else {
                    number_mapping[current_number_idx] = number_mapping[current_number_idx] * 10 + c.to_digit(10).unwrap() as u32;
                }
                self.data[2][i + 1] = PosType::Digit(current_number_idx);
            } else {
                if current_number_idx < number_mapping.len() {
                    current_number_idx += 1;
                }
                if c == '.' {
                    self.data[2][i + 1] = PosType::Empty;
                } else if c == '*' {
                    self.data[2][i + 1] = PosType::GearSymbol;
                } else {
                    self.data[2][i + 1] = PosType::OtherSymbol;
                }
            }
        }
        number_mapping
    }

    /// Roll the contents of the matrix one row up.
    /// The first row is discarded.
    /// The last row is filled with empty positions.
    fn roll(&mut self) {
        self.data[0] = self.data[1].clone();
        self.data[1] = self.data[2].clone();
        self.data[2] = vec![PosType::Empty; self.data[2].len()];
    }

    fn sum_valid_numbers(&self, number_values: &Vec<u32>) -> u32 {
        let mut result = 0;
        let mut not_yet_processed_idx = 0;
        for (idx, pos) in self.data[1].iter().enumerate() {
            if not_yet_processed_idx == number_values.len() {
                break;
            }
            if let PosType::Digit(n) = pos {
                if *n >= not_yet_processed_idx && self.has_symbol_as_neighbour(idx) {
                    result += number_values[*n] as u32;
                    not_yet_processed_idx = *n + 1;
                }
            }
        }
        result
    }

    fn has_symbol_as_neighbour(&self, idx: usize) -> bool {
        self.data[0][idx - 1].is_symbol() ||
        self.data[0][idx].is_symbol() ||
        self.data[0][idx + 1].is_symbol() ||
        self.data[1][idx - 1].is_symbol() ||
        self.data[1][idx + 1].is_symbol() ||
        self.data[2][idx - 1].is_symbol() ||
        self.data[2][idx].is_symbol() ||
        self.data[2][idx + 1].is_symbol()
    }

    fn sum_gear_ratios(&self, number_values: [&Vec<u32>; 3]) -> u32 {
        let mut result = 0;
        for (idx, pos) in self.data[1].iter().enumerate() {
            if let PosType::GearSymbol = pos {
                let neighbours = self.neighboured_numbers(idx, number_values);
                if neighbours.len() != 2 {
                    continue;
                }
                let a = neighbours.iter().next().unwrap();
                let b = neighbours.iter().last().unwrap();
                result += a * b;
            }
        };
        result
    }

    fn neighboured_numbers(&self, idx: usize, number_values: [&Vec<u32>; 3]) -> HashSet<u32> {
        let mut result = HashSet::new();
        for row_offset in -1..=1 {
            for col_offset in -1..=1 {
                if let PosType::Digit(n) = self.data[(row_offset + 1) as usize][(idx as i32 + col_offset) as usize] {
                    let value = number_values[(row_offset + 1) as usize][n];
                    result.insert(value);
                }
            }
        }
        result
    }
}

pub fn stage1(file_path: &str) -> u32 {
    let input_str = read_to_string(file_path).unwrap();
    let first_line = input_str.lines().next().unwrap();
    let line_length = first_line.len();

    let mut matrix = Matrix::new(line_length);
    let mut number_values = matrix.write_line(first_line);
    matrix.roll();

    let mut result = 0;
    for line in input_str.lines().skip(1) {
        let next_number_values = matrix.write_line(line);

        result += matrix.sum_valid_numbers(&number_values);

        matrix.roll();
        number_values = next_number_values;
    }
    result += matrix.sum_valid_numbers(&number_values);

    result
}

pub fn stage2(file_path: &str) -> u32 {
    let input_str = read_to_string(file_path).unwrap();
    let first_line = input_str.lines().next().unwrap();
    let line_length = first_line.len();

    let mut matrix = Matrix::new(line_length);
    let mut number_values = matrix.write_line(first_line);
    let mut last_number_values = vec![];
    matrix.roll();

    let mut result = 0;
    for line in input_str.lines().skip(1) {
        let next_number_values = matrix.write_line(line);

        result += matrix.sum_gear_ratios([
            &last_number_values,
            &number_values,
            &next_number_values
        ]);

        matrix.roll();
        last_number_values = number_values;
        number_values = next_number_values;
    }
    result += matrix.sum_gear_ratios([
        &last_number_values,
        &number_values,
        &vec![],
    ]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage1() {
        assert_eq!(stage1("inputs/day03/example.txt"), 4361);
        assert_eq!(stage1("inputs/day03/input.txt"), 521601);
    }

    #[test]
    fn test_stage2() {
        assert_eq!(stage2("inputs/day03/example.txt"), 467835);
        assert_eq!(stage2("inputs/day03/input.txt"), 80694070);
    }
}