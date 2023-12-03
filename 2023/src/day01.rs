use std::fs::read_to_string;

pub fn stage1(file_path: &str) -> u32 {
    read_to_string(file_path).unwrap().lines().map(|line| {
        let nums: Vec<u32> = line.chars().filter_map(|c | c.to_digit(10)).collect();
        let first = nums.first().unwrap();
        let last = nums.last().unwrap();
        first * 10 + last
    }).sum()
}

pub fn stage2(file_path: &str) -> u32 {
    read_to_string(file_path).unwrap().lines().map(|line| {
        let nums = custom_to_digits(line);
        let first = nums.first().unwrap();
        let last = nums.last().unwrap();
        first * 10 + last
    }).sum()
}

fn custom_to_digits(line: &str) -> Vec<u32> {
    let numbers = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    line.chars().enumerate().filter_map(|(i, c)| {
        match c.to_digit(10) {
            None => {
                for (pos, number) in numbers.iter().enumerate() {
                    if line[i..].starts_with(number) {
                        return Some(pos as u32);
                    }
                }
                None
            },
            x => x
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage1() {
        assert_eq!(stage1("inputs/day01/example1.txt"), 142);
        assert_eq!(stage1("inputs/day01/input.txt"), 54634);
    }

    #[test]
    fn test_stage2() {
        assert_eq!(stage2("inputs/day01/example2.txt"), 281);
        assert_eq!(stage2("inputs/day01/input.txt"), 53855);
    }
}