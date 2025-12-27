use std::fs;

#[derive(Debug, Clone, Copy)]
struct Point {
    index: usize,
    x: i64,
    y: i64,
}

fn compute_area(p1: &Point, p2: &Point) -> i64 {
    (p1.x - p2.x + 1).abs() * (p1.y - p2.y + 1).abs()
}

fn compute_largest_rectangle_area(points: &Vec<Point>) -> i64 {
    points
        .iter()
        .map(|p1| {
            points
                .iter()
                .filter(|p2| p1.index != p2.index)
                .map(|p2| compute_area(p1, p2))
        })
        .flatten()
        .max()
        .unwrap()
}

pub fn solve() {
    let input: Vec<Point> = fs::read_to_string("InputDay9.txt")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let coords: Vec<i64> = line.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
            Point {
                index,
                x: coords[0],
                y: coords[1],
            }
        })
        .collect();

    let result = compute_largest_rectangle_area(&input);

    println!("Day 9 - Part 1: {}", result);
}
