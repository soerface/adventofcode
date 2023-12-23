use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::str::Lines;

fn build_mapping(lines: &mut Lines) -> HashMap<String, [String; 2]> {
    let mut mapping = HashMap::new();
    for line in lines {
        let mut parts = line.split(" = (");
        let from = parts.next().unwrap();
        let second_part_cleaned = parts.next().unwrap().replace(")", "");
        let to = second_part_cleaned.split(", ").collect::<Vec<&str>>();
        mapping.insert(from.to_string(), [to[0].to_string(), to[1].to_string()]);
    }
    mapping
}

pub fn stage1(file_path: &str) -> u32 {
    let input_str = read_to_string(file_path).unwrap();
    let mut lines = input_str.lines();
    let turns: Vec<usize> = lines.next().unwrap().chars().map(|c| if c == 'L' { 0 } else { 1 }).collect();
    // skip empty line
    lines.next();
    let mapping = build_mapping(&mut lines);
    let mut current = String::from("AAA");
    let mut iterations = 0;
    while current != "ZZZ" {
        let next = mapping.get(&current).unwrap()[turns[iterations % turns.len()]].clone();
        current = next;
        iterations += 1;
    };
    iterations as u32
}

pub fn stage2(file_path: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage1() {
        assert_eq!(stage1("inputs/day08/example1.txt"), 2);
        assert_eq!(stage1("inputs/day08/example2.txt"), 6);
        assert_eq!(stage1("inputs/day08/input.txt"), 12599);
    }

    #[test]
    fn test_stage2() {
        assert_eq!(stage2("inputs/day08/example3.txt"), 6);
        // assert_eq!(stage2("inputs/day08/input.txt"), 0);
    }
}