use std::env;
mod Day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let days = vec![
        Day1::solve,
        day2::solve,
        day3::solve,
        day4::solve,
        day5::solve,
    ];
    let args: Vec<String> = env::args().collect();
    let day = days[args[1].parse::<usize>().unwrap() - 1];

    let start = std::time::Instant::now();
    day();
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
