use std::{fs, io};

#[derive(Debug, Clone)]
enum Move {
    Left(i32),
    Right(i32),
}

struct KeyLock {
    position: i32,
    code: i32,
}

impl KeyLock {
    fn new() -> Self {
        KeyLock {
            position: 50,
            code: 0,
        }
    }

    fn turn(&mut self, mv: &Move) {
        match mv {
            Move::Left(dist) => self.update_code(-dist),
            Move::Right(dist) => self.update_code(*dist),
        }
    }

    fn turn2(&mut self, mv: &Move) {
        let (dist, direction) = match mv {
            Move::Left(dist) => (dist, -1),
            Move::Right(dist) => (dist, 1),
        };

        for _ in 0..*dist {
            self.position += direction;
            if self.position < 0 {
                self.position = 99;
            } else if self.position > 99 {
                self.position = 0;
            }

            if self.position == 0 {
                self.code += 1;
            }
        }
    }

    fn update_code(&mut self, dist: i32) {
        let y = self.position + dist % 100;
        let revolutions = dist.abs() / 100;
        self.code += revolutions
            + (if (y < 0 || y > 100) && self.position != 0 {
                1
            } else {
                0
            });
        self.position = y.rem_euclid(100);

        if self.position == 0 {
            self.code += 1;
        }
    }
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

fn compute_code_part1(moves: &[Move]) -> i32 {
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

fn compute_code_part2(moves: &[Move]) -> i32 {
    let mut key_lock = KeyLock::new();
    let mut key_lock2 = KeyLock::new();

    for mv in moves {
        key_lock.turn(mv);
        key_lock2.turn2(mv);
        println!(
            "After move {:?}: position1 = {}, code1 = {}, position2 = {}, code2 = {}",
            mv, key_lock.position, key_lock.code, key_lock2.position, key_lock2.code
        );

        if key_lock.code != key_lock2.code {
            let mut buf = String::new();
            let _apa = io::stdin().read_line(&mut buf);
        }
    }

    key_lock.code
}

pub fn solve() {
    let moves: Vec<Move> = fs::read_to_string("InputDay1.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(to_move) // make each slice into a string
        .collect(); // gather them together into a vector

    let code_part1 = compute_code_part1(&moves);
    let code_part2 = compute_code_part2(&moves);

    println!("Part 1 answer = {}", code_part1);
    println!("Part 2 answer = {}", code_part2);
}
