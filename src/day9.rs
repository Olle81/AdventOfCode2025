use itertools::Itertools;
use std::fs;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

fn compute_area(p1: &Point, p2: &Point) -> i64 {
    ((p1.x - p2.x).abs() + 1) * ((p1.y - p2.y).abs() + 1)
}

fn create_rectangles(points: &Vec<Point>) -> Vec<(&Point, &Point)> {
    points
        .iter()
        .flat_map(|p1| {
            points
                .iter()
                .filter(|p2| p1.x != p2.x && p1.y != p2.y)
                .map(move |p2| (p1, p2))
        })
        .collect()
}

fn are_intersecting(v: &VerticalLine, h: &HorizontalLine) -> bool {
    if h.y <= v.y1 || h.y >= v.y2 {
        return false;
    }
    h.x1 < v.x && h.x2 > v.x
}

fn solve_part_1(points: &Vec<Point>) -> i64 {
    create_rectangles(points)
        .iter()
        .map(|(p1, p2)| compute_area(p1, p2))
        .max()
        .unwrap()
}

#[derive(Debug)]
struct RectWithArea<'a> {
    p1: &'a Point,
    p2: &'a Point,
    area: i64,
}

fn split_by_orientation(lines: &Vec<Line>) -> (Vec<VerticalLine>, Vec<HorizontalLine>) {
    lines.into_iter().fold(
        (Vec::new(), Vec::new()),
        |(mut verticals, mut horizontals), line| {
            match line {
                Line::Vertical(v) => verticals.push(v.clone()),
                Line::Horizontal(h) => horizontals.push(h.clone()),
            }
            (verticals, horizontals)
        },
    )
}

fn print_points(points: &Vec<Point>) {
    for point in points {
        println!("{},{}", point.x, point.y);
    }
}

fn solve_part_2(points: &Vec<Point>) -> i64 {
    let rectangles = create_rectangles(points)
        .iter()
        .map(|(p1, p2)| RectWithArea {
            p1,
            p2,
            area: compute_area(p1, p2),
        })
        .sorted_by(|a, b| b.area.cmp(&a.area))
        .collect::<Vec<RectWithArea>>();

    println!("Total rectangles to check: {}", rectangles.len());

    let polygon = Polygon::new(points);

    for (index, rectangle) in rectangles.iter().enumerate() {
        print!(
            "\rChecking rectangle {}/{} with area {}...",
            index + 1,
            rectangles.len(),
            rectangle.area
        );

        let is_inside = polygon.contains_rect(&rectangle);

        if is_inside {
            println!("Found rectangle");
            let points = get_points(rectangle);
            print_points(&points);
            println!();
            return rectangle.area;
        }
    }

    0
}

#[derive(Debug, Clone)]
struct VerticalLine {
    x: i64,
    y1: i64,
    y2: i64,
}

#[derive(Debug, Clone)]
struct HorizontalLine {
    y: i64,
    x1: i64,
    x2: i64,
}

struct Polygon {
    vertical_lines: Vec<VerticalLine>,
    horizontal_lines: Vec<HorizontalLine>,
}

impl Polygon {
    pub fn new(points: &Vec<Point>) -> Self {
        let lines = create_lines(points);
        let (vertical_lines, horizontal_lines) = split_by_orientation(&lines);

        Polygon {
            vertical_lines,
            horizontal_lines,
        }
    }

    pub fn contains_point(&self, point: &Point) -> bool {
        let on_horizontal_line = self
            .horizontal_lines
            .iter()
            .filter(|line| line.x1 <= point.x && line.x2 >= point.x)
            .any(|line| line.y == point.y);

        if on_horizontal_line {
            return true;
        }

        let vertical_lines = self
            .vertical_lines
            .iter()
            .filter(|line| line.y1 <= point.y && line.y2 > point.y)
            .collect::<Vec<&VerticalLine>>();

        if vertical_lines.iter().any(|line| line.x == point.x) {
            return true;
        }

        let intersections_left = vertical_lines
            .iter()
            .filter(|line| line.x < point.x)
            .count();

        let intersections_right = vertical_lines
            .iter()
            .filter(|line| line.x > point.x)
            .count();

        let is_outside = intersections_left % 2 == 0 || intersections_right % 2 == 0;

        !is_outside
    }

    pub fn contains_rect(&self, rect: &RectWithArea) -> bool {
        let points = get_points(rect);

        let points_inside = points.iter().all(|point| self.contains_point(point));

        let lines_intersecting = create_lines_from_rect(rect)
            .iter()
            .any(|line| self.intersects(line));

        points_inside && !lines_intersecting
    }

    pub fn intersects(&self, line: &Line) -> bool {
        match line {
            Line::Vertical(v) => self.horizontal_lines.iter().any(|h| are_intersecting(v, h)),
            Line::Horizontal(h) => self.vertical_lines.iter().any(|v| are_intersecting(v, h)),
        }
    }
}

#[derive(Debug, Clone)]
enum Line {
    Vertical(VerticalLine),
    Horizontal(HorizontalLine),
}

fn create_line(p1: &Point, p2: &Point) -> Line {
    if p1.x == p2.x {
        let (y1, y2) = if p1.y < p2.y {
            (p1.y, p2.y)
        } else {
            (p2.y, p1.y)
        };

        Line::Vertical(VerticalLine {
            x: p1.x,
            y1: y1,
            y2: y2,
        })
    } else {
        let (x1, x2) = if p1.x < p2.x {
            (p1.x, p2.x)
        } else {
            (p2.x, p1.x)
        };

        Line::Horizontal(HorizontalLine {
            y: p1.y,
            x1: x1,
            x2: x2,
        })
    }
}

fn get_points(rect: &RectWithArea) -> Vec<Point> {
    let p1 = rect.p1;
    let p2 = rect.p2;

    vec![
        Point {
            x: p1.x.min(p2.x),
            y: p1.y.min(p2.y),
        },
        Point {
            x: p1.x.max(p2.x),
            y: p1.y.min(p2.y),
        },
        Point {
            x: p1.x.max(p2.x),
            y: p1.y.max(p2.y),
        },
        Point {
            x: p1.x.min(p2.x),
            y: p1.y.max(p2.y),
        },
    ]
}

fn create_lines_from_rect(rect: &RectWithArea) -> Vec<Line> {
    let points = get_points(rect);
    create_lines(&points)
}

fn create_lines(points: &Vec<Point>) -> Vec<Line> {
    let mut lines: Vec<Line> = points
        .windows(2)
        .map(|window| create_line(&window[0], &window[1]))
        .collect();
    lines.push(create_line(&points[points.len() - 1], &points[0]));
    lines
}

fn read_input() -> Vec<Point> {
    fs::read_to_string("InputDay9.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let coords: Vec<i64> = line.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
            Point {
                x: coords[0],
                y: coords[1],
            }
        })
        .collect()
}

pub fn solve() {
    let input: Vec<Point> = read_input();

    let result_part_1 = solve_part_1(&input);
    println!("Day 9 - Part 1: {}", result_part_1);

    let result_part_2 = solve_part_2(&input);
    println!("Day 9 - Part 2: {}", result_part_2);
}

#[cfg(test)]
impl<'a> RectWithArea<'a> {
    pub fn new(p1: &'a Point, p2: &'a Point) -> Self {
        let area = compute_area(p1, p2);
        RectWithArea { p1, p2, area }
    }
}

#[cfg(test)]
macro_rules! points {
    ($(($x:expr, $y:expr)),* $(,)?) => {
        vec![
            $(
                Point { x: $x, y: $y }
            ),*
        ]
    };
}

#[cfg(test)]
mod tests {

    mod polygon {
        use crate::day9::read_input;

        #[test]
        fn should_contain_point_inside() {
            use crate::day9::{Point, Polygon};

            let polygon = Polygon::new(&points![(0, 0), (0, 5), (5, 5), (5, 0)]);

            let point = Point { x: 3, y: 3 };

            assert!(polygon.contains_point(&point));
        }

        #[test]
        fn should_not_contain_point_outside() {
            use crate::day9::{Point, Polygon};

            let polygon = Polygon::new(&points![(0, 0), (0, 5), (5, 5), (5, 0)]);

            let point = Point { x: 6, y: 3 };

            assert!(!polygon.contains_point(&point));
        }

        #[test]
        fn should_contain_point_on_edge() {
            use crate::day9::{Point, Polygon};

            let polygon = Polygon::new(&points![(0, 0), (0, 5), (5, 5), (5, 0)]);

            let point = Point { x: 0, y: 3 };

            assert!(polygon.contains_point(&point));
        }

        #[test]
        fn should_contain_point_on_vertex() {
            use crate::day9::{Point, Polygon};

            let polygon = Polygon::new(&points![(0, 0), (0, 5), (5, 5), (5, 0)]);

            let point = Point { x: 0, y: 0 };

            assert!(polygon.contains_point(&point));
        }

        #[test]
        fn rectangle_should_contain_itself() {
            use crate::day9::{Point, Polygon, RectWithArea};

            let p1 = Point { x: 1, y: 1 };
            let p2 = Point { x: 4, y: 4 };

            let rectangle = RectWithArea::new(&p1, &p2);

            let polygon = Polygon::new(&points![(1, 1), (4, 1), (4, 4), (1, 4),]);

            assert!(polygon.contains_rect(&rectangle));
        }

        #[test]
        fn rectangle_should_not_contain_larger_rectangle() {
            use crate::day9::{Point, Polygon, RectWithArea};

            let p1 = Point { x: 0, y: 0 };
            let p2 = Point { x: 5, y: 5 };

            let rectangle = RectWithArea::new(&p1, &p2);

            let polygon = Polygon::new(&points![(1, 1), (1, 4), (4, 4), (4, 1),]);

            assert!(!polygon.contains_rect(&rectangle));
        }

        #[test]
        fn should_contain_partially_collinear_rectangle() {
            use crate::day9::{Point, Polygon, RectWithArea};

            let polygon = Polygon::new(&points![(0, 0), (0, 3), (3, 3), (3, 5), (6, 5), (6, 0)]);

            let rect = RectWithArea::new(&Point { x: 0, y: 3 }, &Point { x: 6, y: 0 });

            assert!(polygon.contains_rect(&rect));
        }

        #[test]
        fn cancave_polygon_should_not_contain_rectangle_intersecting_concavity() {
            use crate::day9::{Point, Polygon, RectWithArea};

            let polygon = Polygon::new(&points![
                (0, 0),
                (0, 2),
                (2, 2),
                (2, 3),
                (0, 3),
                (0, 5),
                (5, 5),
                (5, 0)
            ]);

            let rect = RectWithArea::new(&Point { x: 1, y: 1 }, &Point { x: 4, y: 4 });

            assert!(!polygon.contains_rect(&rect));
        }

        #[test]
        fn bleh() {
            use crate::day9::{Point, Polygon, RectWithArea};

            let polygon = Polygon::new(&read_input());

            let rect =
                RectWithArea::new(&Point { x: 53638, y: 52482 }, &Point { x: 98403, y: 98146 });

            assert!(!polygon.contains_rect(&rect));
        }

        #[test]
        fn bleh2() {
            use crate::day9::{Point, Polygon, RectWithArea};

            let polygon = Polygon::new(&read_input());

            let rect =
                RectWithArea::new(&Point { x: 6141, y: 50025 }, &Point { x: 94870, y: 67703 });

            assert!(polygon.contains_rect(&rect));
        }
    }
}
