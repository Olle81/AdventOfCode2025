use std::fs;

#[derive(Debug, Clone)]
struct Range {
    start: u64,
    end: u64,
}

fn combine(range1: &Range, range2: &Range) -> Option<Range> {
    if range1.end < range2.start || range2.end < range1.start {
        return None;
    }

    let start = std::cmp::min(range1.start, range2.start);
    let end = std::cmp::max(range1.end, range2.end);

    return Some(Range { start, end });
}

fn combine_all(sorted_ranges: &Vec<Range>) -> Vec<Range> {
    let mut result = Vec::new();

    let mut current_range = sorted_ranges[0].clone();

    for range in sorted_ranges.iter().skip(1) {
        match combine(&current_range, range) {
            Some(combined) => {
                current_range = combined;
            }
            None => {
                result.push(current_range.clone());
                current_range = range.clone();
            }
        }
    }

    result.push(current_range.clone());

    result
}

pub fn solve() {
    let binding = fs::read_to_string("InputDay5.txt").unwrap();

    let lines: Vec<&str> = binding.lines().collect();

    let mut ranges: Vec<Range> = lines
        .iter()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split("-").collect::<Vec<&str>>())
        .map(|s| Range {
            start: s[0].parse().unwrap(),
            end: s[1].parse().unwrap(),
        })
        .collect();

    let values: Vec<u64> = lines
        .iter()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    ranges.sort_by(|a, b| a.start.cmp(&b.start));
    let combined_ranges = combine_all(&ranges);

    let result_part_1: usize = values
        .into_iter()
        .filter(|v| combined_ranges.iter().any(|r| r.start <= *v && r.end >= *v))
        .count();

    let result_part_2: u64 = combined_ranges.iter().map(|r| r.end - r.start + 1).sum();

    println!("Result part 1: {}", result_part_1);
    println!("Result part 2: {:?}", result_part_2);
}
