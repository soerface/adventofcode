use std::collections::HashMap;
use std::fs::read_to_string;

fn to_components(draw: &str) -> (u32, &str) {
    let draw = draw.trim();
    let components: Vec<&str> = draw.split_whitespace().collect();
    let n: u32 = components[0].parse().unwrap();
    let category = components[1];
    (n, category)
}

pub fn game_is_possible(line: &str) -> bool {
    let draws: Vec<&str> = line.split([',', ';']).collect();
    for draw in draws {
        let (n, category) = to_components(draw);
        if category == "red" && n > 12 {
            return false
        }
        if category == "green" && n > 13 {
            return false
        }
        if category == "blue" && n > 14 {
            return false
        }
    }
    true
}

pub fn cube_power(line: &str) -> u32 {
    let mut max_per_category: HashMap<&str, u32> = HashMap::new();
    for draw in line.split([',', ';']) {
        let (n, category) = to_components(draw);
        let current = max_per_category.get(category).copied().unwrap_or(0);
        if n > current {
            max_per_category.insert(category, n);
        }
    }
    max_per_category.get("red").copied().unwrap_or(0) *
    max_per_category.get("blue").copied().unwrap_or(0) *
    max_per_category.get("green").copied().unwrap_or(0)
}

pub fn stage1(file_path: &str) -> u32 {
    read_to_string(file_path).unwrap().lines().enumerate().map(|(i, line)| {
        if game_is_possible(line.split_once(':').unwrap().1) {
            let game_id = i as u32 + 1;
            game_id
        } else {
            0
        }
    }).sum()
}

pub fn stage2(file_path: &str) -> u32 {
    read_to_string(file_path).unwrap().lines().map(|line| {
        cube_power(line.split_once(':').unwrap().1)
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage1() {
        assert_eq!(stage1("inputs/day02/example.txt"), 8);
        assert_eq!(stage1("inputs/day02/input.txt"), 2593);
    }

    #[test]
    fn test_stage2() {
        assert_eq!(stage2("inputs/day02/example.txt"), 2286);
        assert_eq!(stage2("inputs/day02/input.txt"), 54699);
    }
}