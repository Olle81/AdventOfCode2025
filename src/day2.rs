use std::fs;

// fn find_invalid_ids(first: i32, last: i32) -> Vec<i32> {

// }

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64,
}

fn get_range(str: &str) -> Range {
    let parts: Vec<&str> = str.split("-").collect();
    let start = parts[0].trim().parse::<i64>().unwrap();
    let end = parts[1].trim().parse::<i64>().unwrap();
    Range { start, end }
}

fn count_digits(n: i64) -> u32 {
    if n == 0 {
        return 1;
    }

    let mut count = 0;
    let mut num = n.abs();
    while num > 0 {
        num /= 10;
        count += 1;
    }
    count
}

fn get_invalid_codes(range: &Range) -> Vec<i64> {
    let mut number_of_digits = count_digits(range.start);

    let start = if (number_of_digits % 2) != 0 {
        number_of_digits += 1;
        i64::pow(10, number_of_digits - 1)
    } else {
        range.start
    };

    let mut half_number_of_digits = number_of_digits / 2;
    let mut m = i64::pow(10, half_number_of_digits);
    let mut first_half = start / m;
    let mut test_code = first_half * m + first_half;
    let mut result = vec![];

    while test_code <= range.end && test_code >= range.start {
        result.push(test_code);
        first_half += 1;
        if count_digits(first_half) > half_number_of_digits {
            half_number_of_digits += 1;
            m = i64::pow(10, half_number_of_digits);
        }
        test_code = first_half * m + first_half;
    }

    result
}

pub fn solve() {
    let content = fs::read_to_string("ExampleDay2.txt").unwrap();
    let codes: Vec<i64> = content
        .trim()
        .replace("\r\n", "")
        .replace("\n", "")
        .replace("\r", "")
        .split(",")
        .filter(|s| !s.is_empty())
        .map(get_range)
        .flat_map(|range| get_invalid_codes(&range))
        .collect();

    print!("Input: {:?}", codes);
}
