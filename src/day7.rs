use std::{collections::HashMap, fs};

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

fn get_number_of_paths(
    matrix: &Vec<Vec<char>>,
    x: usize,
    start_y: usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    let mut y = start_y;

    let mut get_paths = |x: usize, y: usize| -> usize {
        if let Some(cached) = memo.get(&(x, y)) {
            return *cached;
        }
        let paths = get_number_of_paths(matrix, x, y, memo);
        memo.insert((x, y), paths);
        paths
    };

    while y < matrix.len() - 1 {
        match matrix[y + 1][x] {
            '.' => y += 1,
            '^' => {
                // Split into two paths
                let left = get_paths(x - 1, y + 1);
                let right = get_paths(x + 1, y + 1);
                return left + right;
            }
            _ => panic!("Unexpected character"),
        }
    }

    1
}

fn solve_part_2(matrix: &Vec<Vec<char>>) -> usize {
    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();
    let start_x = matrix[0].iter().position(|c| *c == 'S').unwrap();
    get_number_of_paths(matrix, start_x, 0, &mut memo)
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

    let result_part_2 = solve_part_2(&input);

    println!("Result part 1: {:?}", result.number_of_splits);
    println!("Result part 2: {:?}", result_part_2);
}
