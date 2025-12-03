use std::env;
mod Day1;

fn main() {
    let days = vec![Day1::solve];
    let args: Vec<String> = env::args().collect();
    let day = days[args[1].parse::<usize>().unwrap() - 1];
    day();
}
