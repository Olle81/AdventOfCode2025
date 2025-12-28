use geo::algorithm::Area;
use geo::{BooleanOps, Contains, Coord, LineString, Polygon, Rect};
use itertools::Itertools;
use std::fs;
use std::io::{self, Read};

#[derive(Debug, Clone, Copy)]
struct Point {
    index: usize,
    x: i64,
    y: i64,
}

impl Point {
    fn to_coord(&self) -> Coord<f64> {
        Coord {
            x: self.x as f64,
            y: self.y as f64,
        }
    }
}

fn print_polygon(polygon: &Polygon<f64>) {
    for coord in polygon.exterior().points() {
        println!("{:.0} {:.0},", coord.x(), coord.y());
    }
    println!();
}

fn compute_area(p1: &Point, p2: &Point) -> i64 {
    (p1.x - p2.x + 1).abs() * (p1.y - p2.y + 1).abs()
}

fn create_rectangles(points: &Vec<Point>) -> Vec<(&Point, &Point)> {
    points
        .iter()
        .flat_map(|p1| {
            points
                .iter()
                .filter(|p2| p1.index != p2.index)
                .map(move |p2| (p1, p2))
        })
        .collect()
}

fn create_polygon(points: &Vec<Point>) -> Polygon<f64> {
    Polygon::new(
        points
            .iter()
            .map(|p| Coord {
                x: p.x as f64,
                y: p.y as f64,
            })
            .collect::<LineString<_>>(),
        vec![],
    )
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

fn solve_part_2(points: &Vec<Point>) -> i64 {
    let polygon = create_polygon(points);

    let rectangles = create_rectangles(points)
        .iter()
        .map(|(p1, p2)| RectWithArea {
            p1,
            p2,
            area: compute_area(p1, p2),
        })
        .sorted_by(|a, b| Ord::cmp(&b.area, &a.area))
        .collect::<Vec<_>>();

    println!("Number of rects = {}", rectangles.len());

    for (index, rect) in rectangles.iter().enumerate() {
        let r = Rect::new(rect.p1.to_coord(), rect.p2.to_coord());
        let rect_polygon = Polygon::from(r);

        //println!("Checking rectangle {:?}", rect);

        print!("\rChecking rectangle {}/{}", index + 1, rectangles.len());

        let intersection = polygon.intersection(&rect_polygon);
        let threshold = 0.0001 * rect.area as f64;
        let contains = intersection.iter().count() == 1
            && (rect.area as f64 - intersection.iter().next().unwrap().unsigned_area()).abs()
                < threshold;

        if contains {
            println!("Rectangle contains polygon {:?})", rect);

            print_polygon(&rect_polygon);
            for poly in intersection {
                print_polygon(&poly);
                println!("Area: {}", poly.unsigned_area());
            }

            return rect.area;
        }
    }

    0
}

fn find_bounding_polygon(points: &Vec<Point>) -> Vec<Point> {
    let start_point = *points.iter().min_by(|a, b| a.x.cmp(&b.x)).unwrap();
    let mut point = start_point;
    let mut bounding_polygon: Vec<Point> = Vec::new();
    let mut points_left = points.clone();

    loop {
        // println!("At point {:?}", point);
        // let mut buffer = [0; 1];
        // io::stdin().read_exact(&mut buffer).unwrap();

        bounding_polygon.push(point);
        points_left.retain(|p| p.index != point.index);

        let point_above = points_left
            .iter()
            .filter(|p| p.x == point.x && p.y < point.y)
            .min_by(|a, b| a.y.cmp(&b.y))
            .copied();

        if let Some(p) = point_above {
            point = p;
            continue;
        }

        let point_right = points_left
            .iter()
            .filter(|p| p.y == point.y && p.x > point.x)
            .max_by(|a, b| a.x.cmp(&b.x))
            .copied();

        if let Some(p) = point_right {
            point = p;
            continue;
        }

        let point_below = points_left
            .iter()
            .filter(|p| p.x == point.x && p.y > point.y)
            .max_by(|a, b| a.y.cmp(&b.y))
            .copied();

        if let Some(p) = point_below {
            point = p;
            continue;
        }

        let point_left = points_left
            .iter()
            .filter(|p| p.y == point.y && p.x < point.x)
            .min_by(|a, b| a.x.cmp(&b.x))
            .copied();

        if let Some(p) = point_left {
            point = p;
            continue;
        }

        break;
    }

    bounding_polygon
}

pub fn solve() {
    let input: Vec<Point> = fs::read_to_string("InputDay9.txt")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let coords: Vec<i64> = line.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
            Point {
                index: index,
                x: coords[0],
                y: coords[1],
            }
        })
        .collect();

    let result_part_1 = solve_part_1(&input);
    let result_part_2 = solve_part_2(&input);

    println!("Day 9 - Part 1: {}", result_part_1);
    println!("Day 9 - Part 2: {}", result_part_2);
}
