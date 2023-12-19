use std::cmp::Ordering;
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn within(&self, other: Range) -> bool {
        self.start >= other.start && self.end <= other.end
    }

    fn overlaps(&self, other: Range) -> bool {
        self.start <= other.end && self.end >= other.start
    }

    fn get_overlapping_range(&self, other: Range) -> Option<Range> {
        if self.overlaps(other) {
            Some(Range {
                start: self.start.max(other.start),
                end: self.end.min(other.end),
            })
        } else {
            None
        }
    }

    fn get_non_overlapping_ranges(&self, other: Range) -> Vec<Range> {
        let mut ranges = vec![];
        if self.start < other.start {
            ranges.push(Range { start: self.start, end: other.start - 1 });
        }
        if self.end > other.end {
            ranges.push(Range { start: other.end, end: self.end });
        }
        ranges
    }
}

impl Eq for Range {}

impl PartialEq<Self> for Range {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl PartialOrd<Self> for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.start.partial_cmp(&other.start)
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

fn apply_map_rules(line: &str, cur_list: &mut Vec<i64>, next_list: &mut Vec<i64>) {
    let line_numbers: Vec<i64> = line.split_whitespace().map(|num| num.parse().unwrap()).collect();
    let destination = line_numbers[0];
    let source = line_numbers[1];
    let range_length = line_numbers[2];
    let diff = source - destination;

    let mut to_be_removed_indices = vec![];
    for (i, &n) in cur_list.iter().enumerate() {
        if n >= source &&  n <= source + range_length {
            next_list.push(n - diff);
            to_be_removed_indices.push(i);
        }
    }
    for i in to_be_removed_indices.iter().rev() {
        cur_list.remove(*i);
    }
}

fn apply_map_rules_on_ranges(lines: &str, cur_list: &mut Vec<Range>, next_list: &mut Vec<Range>) {
    let line_numbers: Vec<i64> = lines.split_whitespace().map(|num| num.parse().unwrap()).collect();
    let destination = line_numbers[0];
    let source = line_numbers[1];
    let range_length = line_numbers[2];
    let map_range = Range { start: source, end: source + range_length };
    let diff = source - destination;

    let mut to_be_removed_indices = vec![];
    let mut to_be_added = vec![];
    for (i, &r) in cur_list.iter().enumerate() {
        if r.within(map_range) {
            next_list.push(Range { start: r.start - diff, end: r.end - diff });
            to_be_removed_indices.push(i);
        } else if let Some(overlap) = r.get_overlapping_range(map_range) {
            next_list.push(Range { start: overlap.start - diff, end: overlap.end - diff });
            to_be_removed_indices.push(i);
            to_be_added.extend(r.get_non_overlapping_ranges(map_range));
        }
    }
    for i in to_be_removed_indices.iter().rev() {
        cur_list.remove(*i);
    }
    cur_list.extend(to_be_added);
}

fn process_file<'a, T>(lines: impl Iterator<Item = &'a str>, cur_list: &mut Vec<T>, apply_map_fn: fn(&str, &mut Vec<T>, &mut Vec<T>)) -> T where T: Ord + Copy {
    let mut next_list = vec![];
    let lines: Vec<&str> = lines.collect();
    for line in lines.into_iter() {
        if line.len() == 0 || !line.chars().next().unwrap().is_digit(10) {
            for item in next_list.iter() {
                cur_list.push(*item);
            }
            next_list = vec![];
        } else {
            apply_map_fn(line, cur_list, &mut next_list);
        }
    }
    for item in next_list.iter() {
        cur_list.push(*item);
    }
    cur_list.iter().min().unwrap().clone()
}

pub fn stage1(file_path: &str) -> u32 {
    let input_str = read_to_string(file_path).unwrap();
    let first_line = input_str.lines().next().unwrap();
    let mut initial_list: Vec<i64> = first_line.replace("seeds: ", "").split_whitespace().map(|num| num.parse().unwrap()).collect();

    let lines = input_str.lines().skip(1);
    process_file(lines, &mut initial_list, apply_map_rules).clone() as u32
}

pub fn stage2(file_path: &str) -> u32 {
    let input_str = read_to_string(file_path).unwrap();
    let first_line = input_str.lines().next().unwrap();
    let mut initial_list: Vec<Range> = vec![];
    let nums: Vec<i64> = first_line.replace("seeds: ", "").split_whitespace().map(|num| num.parse().unwrap()).collect();
    for i in (0..nums.len()).step_by(2) {
        let n = nums[i];
        let range = nums[i + 1];
        initial_list.push(Range { start: n, end: n + range });
    }

    let lines = input_str.lines().skip(1);
    process_file(lines, &mut initial_list, apply_map_rules_on_ranges).clone().start as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage1() {
        assert_eq!(stage1("inputs/day05/example.txt"), 35);
        assert_eq!(stage1("inputs/day05/input.txt"), 382895070);
    }

    #[test]
    fn test_stage2() {
        assert_eq!(stage2("inputs/day05/example.txt"), 46);
        assert_eq!(stage2("inputs/day05/input.txt"), 17729182);
    }
}