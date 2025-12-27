use {itertools::Itertools, std::fs};

#[derive(Debug)]
struct Point {
    index: usize,
    x: i64,
    y: i64,
    z: i64,
}

fn distance(a: &Point, b: &Point) -> f64 {
    let dx = (a.x - b.x) as f64;
    let dy = (a.y - b.y) as f64;
    let dz = (a.z - b.z) as f64;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

#[derive(Debug)]
struct Measurement<'a> {
    point_a: &'a Point,
    point_b: &'a Point,
    distance: f64,
}

struct GroupResult<'a> {
    groups: Vec<Vec<&'a Point>>,
    last_connection_points: (&'a Point, &'a Point),
}

fn group<'a>(
    measurements: &'a Vec<Measurement<'a>>,
    connections: usize,
    number_of_boxes: usize,
) -> GroupResult<'a> {
    let sorted: Vec<&Measurement> = measurements
        .into_iter()
        .sorted_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
        .collect();

    let mut groups: Vec<Vec<&Point>> = vec![];

    let mut last_connected_pair: Option<(&Point, &Point)> = None;

    for pair in sorted.iter().take(connections) {
        let (point_a, point_b) = (pair.point_a, pair.point_b);

        let has_a = groups
            .iter()
            .position(|group| group.iter().any(|p| p.index == point_a.index));
        let has_b = groups
            .iter()
            .position(|group| group.iter().any(|p| p.index == point_b.index));

        match (has_a, has_b) {
            (Some(idx_a), None) => {
                groups[idx_a].push(point_b);
            }
            (None, Some(idx_b)) => {
                groups[idx_b].push(point_a);
            }
            (None, None) => {
                groups.push(vec![point_a, point_b]);
            }
            (Some(idx_a), Some(idx_b)) => {
                if idx_a != idx_b {
                    let mut group_b = groups[idx_b].clone();
                    groups[idx_a].append(&mut group_b);
                    groups.remove(idx_b);
                }
            }
        }

        last_connected_pair = Some((point_a, point_b));

        if groups.len() == 1 && groups[0].len() == number_of_boxes {
            break;
        }
    }

    GroupResult {
        groups: groups,
        last_connection_points: last_connected_pair.unwrap(),
    }
}

fn measure(junction_boxes: &Vec<Point>) -> Vec<Measurement<'_>> {
    junction_boxes
        .iter()
        .map(|b1| {
            junction_boxes.iter().map(|b2| Measurement {
                point_a: b1,
                point_b: b2,
                distance: distance(b1, b2),
            })
        })
        .flatten()
        .filter(|m| m.point_a.index != m.point_b.index)
        .unique_by(|m| {
            let (min, max) = if m.point_a.index < m.point_b.index {
                (m.point_a.index, m.point_b.index)
            } else {
                (m.point_b.index, m.point_a.index)
            };
            (min, max)
        })
        .collect()
}

fn solve_part_1(boxes: &Vec<Point>, connections: usize) -> usize {
    let measures = measure(&boxes);
    let result = group(&measures, connections, boxes.len());
    result
        .groups
        .iter()
        .map(|g| g.len())
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .reduce(|acc, x| acc * x)
        .unwrap()
}

fn solve_part_2(boxes: &Vec<Point>) -> i64 {
    let measures = measure(&boxes);
    let groups = group(&measures, measures.len(), boxes.len());
    let (p1, p2) = groups.last_connection_points;
    p1.x * p2.x
}

pub fn solve() {
    let input: Vec<Point> = fs::read_to_string("InputDay8.txt")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let coords: Vec<i64> = line.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
            Point {
                index,
                x: coords[0],
                y: coords[1],
                z: coords[2],
            }
        })
        .collect();

    let result_part_1 = solve_part_1(&input, 1000);
    println!("Result part 1: {:?}", result_part_1);

    let result_part_2 = solve_part_2(&input);
    println!("Result part 2: {:?}", result_part_2);
}
