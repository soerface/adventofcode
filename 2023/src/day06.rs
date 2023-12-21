use std::fs::read_to_string;

struct Input {
    times: Vec<u64>,
    distances: Vec<u64>,
}

impl Input {
    fn from_file(file_path: &str, parse: fn(&str) -> Vec<u64>) -> Input {
        let input_str = read_to_string(file_path).unwrap();
        let mut lines = input_str.lines();
        let times = parse(lines.next().unwrap());
        let distances = parse(lines.next().unwrap());
        Input { times, distances }
    }
}

fn min_hold_time(time: u64, distance: u64) -> u64 {
    let res = 0.5 * ((time as f64) - ((time.pow(2) - 4 * distance) as f64).sqrt());
    let mut rounded = res.ceil();
    if rounded == res {
        rounded += 1.0;
    }
    rounded as u64
}

fn max_hold_time(time: u64, distance: u64) -> u64 {
    let res = 0.5 * ((time as f64) + ((time.pow(2) - 4 * distance) as f64).sqrt());
    let mut rounded = res.floor();
    if rounded == res {
        rounded -= 1.0;
    }
    rounded as u64
}

pub fn stage1(file_path: &str) -> u64 {
    fn parse(line: &str) -> Vec<u64> {
        line.split(":").nth(1).unwrap().trim().split_whitespace().map(|num| num.parse().unwrap()).collect()
    }

    let input = Input::from_file(file_path, parse);
    let mut results = vec![];
    for (time, distance) in input.times.iter().zip(input.distances.iter()) {
        let min_time = min_hold_time(*time, *distance);
        let max_time = max_hold_time(*time, *distance);
        let result = max_time - min_time + 1;
        results.push(result);
    }
    results.iter().product()
}

pub fn stage2(file_path: &str) -> u64 {
    fn parse(line: &str) -> Vec<u64> {
        let s = line.split(":").nth(1).unwrap().chars().filter(|c| !c.is_whitespace()).collect::<String>();
        let n = s.parse().unwrap();   // .map(|num| num.parse().unwrap()).collect()
        vec![n]
    }

    let input = Input::from_file(file_path, parse);
    let min_time = min_hold_time(input.times[0], input.distances[0]);
    let max_time = max_hold_time(input.times[0], input.distances[0]);
    max_time - min_time + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage1() {
        assert_eq!(stage1("inputs/day06/example.txt"), 288);
        assert_eq!(stage1("inputs/day06/input.txt"), 1084752);
    }

    #[test]
    fn test_stage2() {
        assert_eq!(stage2("inputs/day06/example.txt"), 71503);
        assert_eq!(stage2("inputs/day06/input.txt"), 28228952);
    }
}