use std::{clone, fs};

struct Result {
    matrix: Vec<Vec<char>>,
    number_of_splits: usize,
}

fn solve_part_1(matrix: &Vec<Vec<char>>) -> Result {
    let mut result: Vec<Vec<char>> = Vec::new();
    result.push(matrix.first().unwrap().clone());

    let width = matrix[0].len();
    let mut number_of_splits = 0;

    for row in matrix.iter().skip(1) {
        let previous = result.last().unwrap();
        let mut next = row.clone();
        for col in 0..width {
            match previous[col] {
                '|' | 'S' => match row[col] {
                    '.' => next[col] = '|',
                    '^' => {
                        number_of_splits += 1;
                        next[col - 1] = '|';
                        next[col + 1] = '|';
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        result.push(next);
    }

    Result {
        matrix: result,
        number_of_splits,
    }
}

pub fn solve() {
    let input: Vec<Vec<char>> = fs::read_to_string("InputDay7.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let result = solve_part_1(&input);

    for row in result.matrix {
        let line: String = row.iter().collect();
        println!("{line}");
    }

    println!("Number of splits: {:?}", result.number_of_splits);
}
