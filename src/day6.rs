use std::fs;

#[derive(Debug)]
struct Operation {
    func: fn(i64, i64) -> i64,
    initial_value: i64,
}

fn compute(numbers: &Vec<Vec<i64>>, operations: &Vec<Operation>) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();

    for col in 0..numbers[0].len() {
        let operation = &operations[col];
        let mut acc = operation.initial_value;

        for row in 0..numbers.len() {
            acc = (operation.func)(acc, numbers[row][col]);
        }

        result.push(acc);
    }

    result
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
