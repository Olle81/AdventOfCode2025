use std::fs;

fn get_voltage(battery: Vec<i8>) -> i8 {
    let mut first_max_digit_index: usize = 0;

    for index in 0..battery.len() - 1 {
        if battery[index] > battery[first_max_digit_index] {
            first_max_digit_index = index;
        }
    }

    let mut second_max_digit_index: usize = first_max_digit_index + 1;

    for index in second_max_digit_index..battery.len() {
        if battery[index] > battery[second_max_digit_index] {
            second_max_digit_index = index;
        }
    }

    battery[first_max_digit_index] * 10 + battery[second_max_digit_index]
}

pub fn solve() {
    let voltages: u32 = fs::read_to_string("InputDay3.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c as i8 - '0' as i8)
                .collect::<Vec<i8>>()
        })
        .map(get_voltage)
        .map(|v| v as u32)
        .sum();

    println!("Result: {:?}", voltages);
}
