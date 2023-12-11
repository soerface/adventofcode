use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn build_number_set(line: &str) -> HashSet<u32> {
    let mut number_set = HashSet::new();
    for number in line.split_whitespace() {
        number_set.insert(number.parse().unwrap());
    }
    number_set
}

pub fn stage1(file_path: &str) -> u32 {
    read_to_string(file_path).unwrap().lines().map(|line| {
        let parts = line.split_once(':').unwrap().1.split_once('|').unwrap();
        let first_set = build_number_set(parts.0);
        let second_set = build_number_set(parts.1);
        let common_items = first_set.intersection(&second_set).count();
        if common_items == 0 {
            0
        } else {
            2_u32.pow(common_items as u32 - 1)
        }
    }).sum()
}

fn count_cards(lines: String) -> u32 {
    let mut card_multiplier = HashMap::new();
    lines.lines().enumerate().map(|(lineno, line)| {
        let parts = line.split_once(':').unwrap().1.split_once('|').unwrap();
        let first_set = build_number_set(parts.0);
        let second_set = build_number_set(parts.1);
        let common_items = first_set.intersection(&second_set).count();
        let mut sum = 0;
        let num_cards = *card_multiplier.get(&lineno).unwrap_or(&1);
        for _ in 0..num_cards {
            for n in 0..common_items {
                let cur = card_multiplier.get(&(lineno + n + 1)).unwrap_or(&1);
                card_multiplier.insert(lineno + n + 1, cur + 1);
            }
            sum += 1;
        }
        sum
    }).sum()
}

pub fn stage2(file_path: &str) -> u32 {
    let lines = read_to_string(file_path).unwrap();
    count_cards(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage1() {
        assert_eq!(stage1("inputs/day04/example.txt"), 13);
        assert_eq!(stage1("inputs/day04/input.txt"), 23028);
    }

    #[test]
    fn test_stage2() {
        assert_eq!(stage2("inputs/day04/example.txt"), 30);
        assert_eq!(stage2("inputs/day04/input.txt"), 9236992);
    }
}