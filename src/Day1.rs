use std::fs;

#[derive(Debug, Clone)]
enum Move {
    Left(i32),
    Right(i32),
}

fn to_move(str: &str) -> Move {
    let (direction, distance_str) = str.split_at(1);
    let distance = distance_str.parse().unwrap();

    match direction {
        "L" => Move::Left(distance),
        "R" => Move::Right(distance),
        _ => panic!("Invalid direction"),
    }
}

fn compute_code(moves: &[Move]) -> i32 {
    let mut x = 50;
    let mut code = 0;
    for mv in moves {
        match mv {
            Move::Left(dist) => x = (x - dist) % 100,
            Move::Right(dist) => x = (x + dist) % 100,
        }

        if x == 0 {
            code += 1;
        }
    }

    code
}

pub fn solve() {
    let moves: Vec<Move> = fs::read_to_string("InputDay1.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(to_move) // make each slice into a string
        .collect(); // gather them together into a vector

    let code = compute_code(&moves);

    print!("Code = {}", code);
}
