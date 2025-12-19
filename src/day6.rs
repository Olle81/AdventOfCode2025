use std::fs;

#[derive(Debug)]
struct Operation {
    func: fn(i64, i64) -> i64,
    initial_value: i64,
    sign: char,
}

#[derive(Debug)]
struct Problem {
    matrix: Vec<Vec<char>>,
    operation: Operation,
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

fn get_operation(op_char: char) -> Operation {
    match op_char {
        '+' => Operation {
            func: |a: i64, b: i64| a + b,
            initial_value: 0,
            sign: op_char,
        },
        '*' => Operation {
            func: |a: i64, b: i64| a * b,
            initial_value: 1,
            sign: op_char,
        },
        _ => panic!("Unknown operation"),
    }
}

fn solve_part_1(input: &str) {
    let lines: Vec<Vec<&str>> = input
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
        .map(|x| get_operation(x.chars().next().unwrap()))
        .collect();

    let results = compute(&numbers, &operations);
    let result = results.iter().sum::<i64>();

    println!("{:?}", results);
    println!("{:?}", result);
}

fn solve_problem(problem: &Problem) -> i64 {
    let mut result = problem.operation.initial_value;

    println!(
        "Problem: {:?}, operation: {:?}",
        problem.matrix, problem.operation.sign
    );

    for col in 0..problem.matrix[0].len() {
        let string_value = problem
            .matrix
            .iter()
            .map(|row| row[col])
            .filter(|c| *c != ' ')
            .collect::<String>();
        let value = string_value.parse::<i64>().unwrap();
        result = (problem.operation.func)(result, value);
    }
    result
}

fn solve_part_2(input: &str) {
    let problems = parse_problems(input);
    let result = problems
        .iter()
        .map(|problem| solve_problem(problem))
        .sum::<i64>();
    println!("{:?}", result);
}

fn parse_problems(input: &str) -> Vec<Problem> {
    let matrix = input
        .lines()
        .map(|line| line.to_string() + " ")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut next_problem_column = 0;
    let mut problems: Vec<Problem> = Vec::new();

    let columns = matrix.iter().map(|row| row.len()).max().unwrap();

    for col in 0..columns {
        let number_of_blank_chars = matrix.iter().filter(|row| row[col] == ' ').count();

        if number_of_blank_chars == matrix.len() {
            let problem_matrix: Vec<Vec<char>> = matrix[0..matrix.len() - 1]
                .iter()
                .map(|row| row[next_problem_column..col].to_vec())
                .collect();

            let operation_symbol = matrix[matrix.len() - 1][next_problem_column..col]
                .iter()
                .filter(|c| *c != &' ')
                .collect::<Vec<&char>>()[0];

            problems.push(Problem {
                matrix: problem_matrix,
                operation: get_operation(*operation_symbol),
            });

            next_problem_column = col + 1;
        }
    }

    problems
}

pub fn solve() {
    let input = fs::read_to_string("InputDay6.txt").unwrap();
    println!("Part 1:");
    solve_part_1(&input);
    println!("Part 2:");
    solve_part_2(&input);
}
