use std::fs;

fn get_joltage_2(battery: Vec<i8>, battery_length: usize) -> u64 {
    let mut index_of_largest: usize = 0;
    let mut result = 0;

    for digit in 0..battery_length {
        let end = battery.len() - battery_length + digit + 1;

        for index in index_of_largest..end {
            if battery[index] > battery[index_of_largest] {
                index_of_largest = index;
            }
        }

        result = result * 10 + battery[index_of_largest] as u64;

        index_of_largest += 1;
    }

    result
}

pub fn get_result(battery_length: usize) -> u64 {
    let joltage: u64 = fs::read_to_string("InputDay3.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c as i8 - '0' as i8)
                .collect::<Vec<i8>>()
        })
        .map(|v| get_joltage_2(v, battery_length))
        .map(|v| v as u64)
        .sum();

    return joltage;
}

pub fn solve() {
    println!("Result 1: {:?}", get_result(2));
    println!("Result 2: {:?}", get_result(12));
}
