use std::fs;

#[derive(Debug, Clone)]
struct Range {
    start: i64,
    end: i64,
}

fn combine(range1: &Range, range2: &Range) -> Option<Range> {
    if range1.end < range2.start || range2.end < range1.start {
        return None;
    }

    let start = std::cmp::min(range1.start, range2.start);
    let end = std::cmp::max(range1.end, range2.end);

    return Some(Range { start, end });
}

fn combine_all(ranges: &mut Vec<Range>) {
    if ranges.iter().count() < 2 {
        return;
    }

    let mut combination_found: Option<(Range, usize)> = None;

    for n in 0..ranges.iter().count() - 1 {
        match combine(&ranges[n], &ranges[n + 1]) {
            Some(combined) => {
                combination_found = Some((combined, n));
                break;
            }
            None => {}
        }
    }

    match combination_found {
        Some((combined_range, index)) => {
            ranges.remove(index + 1);
            ranges.remove(index);
            ranges.insert(index, combined_range);
            combine_all(ranges);
        }
        None => {}
    }
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

    let values: Vec<i64> = lines
        .iter()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    combine_all(&mut ranges);
    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let result: usize = values
        .into_iter()
        .filter(|v| ranges.iter().any(|r| r.start <= *v && r.end >= *v))
        .count();

    println!("Result: {}", result);
}
