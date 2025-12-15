use std::fs;

fn find_accessable_rolls(map: Vec<Vec<bool>>) -> i32 {
    let range: [i32; 3] = [-1, 0, 1];
    let rows = map.len() as i32;
    let cols = map[0].len() as i32;
    let mut result = 0;

    for y in 0..rows {
        for x in 0..cols {
            let mut sum = 0;

            if !map[y as usize][x as usize] {
                continue;
            }

            for dy in range {
                if y + dy < 0 || y + dy >= rows {
                    continue;
                }

                for dx in range {
                    if x + dx < 0 || x + dx >= cols {
                        continue;
                    }

                    if dy == 0 && dx == 0 {
                        continue;
                    }

                    if map[(y + dy) as usize][(x + dx) as usize] {
                        sum += 1;
                    }
                }
            }

            if sum < 4 {
                result += 1;
            }
        }
    }

    result
}

fn parse(input: &str) -> Vec<Vec<bool>> {
    return input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '@' { true } else { false })
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();
}

pub fn solve() {
    let input = fs::read_to_string("InputDay4.txt").unwrap();
    let map = parse(&input);
    let result = find_accessable_rolls(map);
    println!("Result: {}", result);
}
