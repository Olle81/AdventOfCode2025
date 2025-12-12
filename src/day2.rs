use std::fs;

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

fn get_range(str: &str) -> Range {
    let parts: Vec<&str> = str.split("-").collect();
    let start = parts[0].trim().parse::<u64>().unwrap();
    let end = parts[1].trim().parse::<u64>().unwrap();
    Range { start, end }
}

fn count_digits(n: u64) -> u32 {
    if n == 0 {
        return 1;
    }

    let mut count = 0;
    let mut num = n;
    while num > 0 {
        num /= 10;
        count += 1;
    }
    count
}

fn get_repeating_patterns(number_of_digits: u32, divisor: u32) -> Vec<u64> {
    if (number_of_digits % divisor) != 0 {
        return vec![];
    }

    let group_length = number_of_digits / divisor;

    let multipliers: Vec<u64> = (0..divisor)
        .map(|i| u64::pow(10, group_length * i))
        .collect();

    let start = u64::pow(10, group_length - 1);
    let end = u64::pow(10, group_length);
    return (start..end)
        .map(|group| multipliers.iter().map(|m| group * m).sum())
        .collect();
}

fn get_invalid_codes_part_2(ranges: &Vec<Range>) -> Vec<u64> {
    let primes: Vec<u32> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23];

    let min = ranges.iter().map(|r| r.start).min().unwrap();
    let max = ranges.iter().map(|r| r.end).max().unwrap();

    let min_digits = count_digits(min);
    let max_digits = count_digits(max);

    let divisors: Vec<u32> = primes.into_iter().filter(|&p| p <= max_digits).collect();

    let mut codes: Vec<u64> = (min_digits..=max_digits)
        .flat_map(|digits| {
            divisors
                .iter()
                .flat_map(move |&d| get_repeating_patterns(digits, d))
        })
        .collect();

    codes.sort();
    codes.dedup();

    let invalid_codes: Vec<u64> = ranges
        .iter()
        .flat_map(|range| {
            codes
                .iter()
                .filter(move |&&code| code >= range.start && code <= range.end)
                .cloned()
        })
        .collect();

    invalid_codes
}

fn get_invalid_codes_part_1(range: &Range) -> u64 {
    let mut number_of_digits = count_digits(range.start);

    let start: u64 = if (number_of_digits % 2) != 0 {
        number_of_digits += 1;
        u64::pow(10, number_of_digits - 1)
    } else {
        range.start
    };

    let mut multiplier = u64::pow(10, number_of_digits / 2);
    let mut first_half = start / multiplier;
    let mut result: u64 = 0;

    loop {
        let test_code = first_half * multiplier + first_half;

        first_half += 1;

        if test_code < range.start {
            continue;
        }

        if test_code > range.end {
            break;
        }

        result += test_code;

        if first_half >= multiplier {
            multiplier *= 10;
        }
    }

    result
}

pub fn solve() {
    let content = fs::read_to_string("InputDay2.txt").unwrap();

    let binding = content
        .trim()
        .replace("\r\n", "")
        .replace("\n", "")
        .replace("\r", "");
    let ranges: Vec<Range> = binding
        .split(",")
        .filter(|s| !s.is_empty())
        .map(get_range)
        .collect();

    let result_part_one: u64 = ranges
        .iter()
        .map(|range| get_invalid_codes_part_1(&range))
        .sum();

    let invalid_codes_part_2: Vec<u64> = get_invalid_codes_part_2(&ranges);

    println!("Result part 1: {:?}", result_part_one);
    println!("Invalid codes part 2: {:?}", invalid_codes_part_2);
    println!(
        "Generated {} invalid codes for part 2",
        invalid_codes_part_2.len()
    );
    println!(
        "Result part 2: {:?}",
        invalid_codes_part_2.iter().sum::<u64>()
    );
}
