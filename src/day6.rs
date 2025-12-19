use std::fs;

#[derive(Debug)]
struct Operation {
    func: fn(i64, i64) -> i64,
    initial_value: i64,
}

fn compute(numbers: &[Vec<i64>], operations: &[Operation]) -> Vec<i64> {
    operations
        .iter()
        .enumerate()
        .map(|(col, operation)| {
            numbers
                .iter()
                .map(|row| row[col])
                .fold(operation.initial_value, operation.func)
        })
        .collect()
}

pub fn solve() {
    let binding = fs::read_to_string("InputDay6.txt").unwrap();
    let lines: Vec<Vec<&str>> = binding
        .lines()
        .map(|line| line.split(' ').filter(|x| *x != "").collect())
        .collect();

    let numbers: Vec<Vec<i64>> = lines[..lines.len() - 1]
        .iter()
        .map(|line| line.iter().map(|x| x.parse::<i64>().unwrap()).collect())
        .collect();

    let operations: Vec<Operation> = lines
        .last()
        .unwrap()
        .iter()
        .map(|op| {
            if *op == "+" {
                Operation {
                    func: |a: i64, b: i64| a + b,
                    initial_value: 0,
                }
            } else {
                Operation {
                    func: |a: i64, b: i64| a * b,
                    initial_value: 1,
                }
            }
        })
        .collect();

    let results = compute(&numbers, &operations);
    let result = results.iter().sum::<i64>();

    println!("{:?}", results);
    println!("{:?}", result);
}
