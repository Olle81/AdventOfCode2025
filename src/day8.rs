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

fn group(neighbours: Vec<Measurement<'_>>) -> usize {
    let sorted: Vec<Measurement> = neighbours
        .into_iter()
        .sorted_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
        .collect();

    // for g in sorted.iter() {
    //     println!("{:?}", g);
    // }

    let mut groups: Vec<Vec<&Point>> = vec![];

    let print = |grp: &Vec<Vec<&Point>>| {
        // for g in grp.iter() {
        //     println!("{:?}", g);
        // }
        // println!();
    };

    for pair in sorted.iter().take(1000) {
        let (point_a, point_b) = (pair.point_a, pair.point_b);

        let has_a = groups
            .iter()
            .position(|group| group.iter().any(|p| p.index == point_a.index));
        let has_b = groups
            .iter()
            .position(|group| group.iter().any(|p| p.index == point_b.index));

        match (has_a, has_b) {
            (Some(idx_a), None) => {
                //print!("Adding {} to group\n", point_b.index);
                groups[idx_a].push(point_b);
                print(&groups);
            }
            (None, Some(idx_b)) => {
                //print!("Adding {} to group\n", point_a.index);
                groups[idx_b].push(point_a);
                print(&groups);
            }
            (None, None) => {
                // print!(
                //     "Creating new group with {} and {}\n",
                //     point_a.index, point_b.index
                // );
                groups.push(vec![point_a, point_b]);
                print(&groups);
            }
            (Some(idx_a), Some(idx_b)) => {
                if idx_a == idx_b {
                    // print!(
                    //     "Both {} and {} are already in the same group\n",
                    //     point_a.index, point_b.index
                    // );
                    // println!();
                } else {
                    // print!(
                    //     "Merging groups of {} and {}\n",
                    //     point_a.index, point_b.index
                    // );
                    let mut group_b = groups[idx_b].clone();
                    groups[idx_a].append(&mut group_b);
                    groups.remove(idx_b);
                    print(&groups);
                }
            }
        }
    }

    groups
        .iter()
        .map(|g| g.len())
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .reduce(|acc, x| acc * x)
        .unwrap()
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

    let measures = measure(&input);
    let result = group(measures);

    println!("{:?}", result);
}
