use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
pub struct Segment {
    x1: u16,
    y1: u16,
    x2: u16,
    y2: u16,
}

fn main() {
    let segments = read_data("./input-full.txt");
    let mut points: Vec<(u16, u16)> = Vec::new();
    let mut points_counters: HashMap<(u16, u16), u16> = HashMap::new();
    for segment in segments {
        points.extend(create_segment_points(segment));
    }

    for point in points {
        if points_counters.contains_key(&point) {
            *points_counters.get_mut(&point).unwrap() += 1;
        } else {
            points_counters.insert(point, 1);
        }
    }

    println!(
        "Result {:?}",
        points_counters.iter().filter(|pc| *pc.1 >= 2).count()
    );
}

pub fn read_lines(path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;
    io::BufReader::new(file).lines().collect()
}

pub fn read_data(path: &str) -> Vec<Segment> {
    let lines: Vec<String> = read_lines(path).unwrap();

    let mut segments = Vec::new();

    // TODO better parsing method ?
    for line in lines {
        let coordinates_parts: Vec<&str> = line.split(" -> ").collect();
        let coordinates_1: Vec<u16> = coordinates_parts[0]
            .split(',')
            .map(|s| s.to_string().parse::<u16>().unwrap())
            .collect();
        let coordinates_2: Vec<u16> = coordinates_parts[1]
            .split(',')
            .map(|s| s.to_string().parse::<u16>().unwrap())
            .collect();

        let x1 = coordinates_1[0];
        let y1 = coordinates_1[1];
        let x2 = coordinates_2[0];
        let y2 = coordinates_2[1];

        segments.push(Segment { x1, y1, x2, y2 })
    }

    return segments;
}

pub fn create_segment_points(segment: Segment) -> Vec<(u16, u16)> {
    let mut points: Vec<(u16, u16)> = vec![(segment.x1, segment.y1)];
    let mut x = segment.x1;
    let mut y = segment.y1;

    while x != segment.x2 || y != segment.y2 {
        x -= (x > segment.x2) as u16;
        x += (x < segment.x2) as u16;
        y -= (y > segment.y2) as u16;
        y += (y < segment.y2) as u16;
        points.push((x, y));
    }

    return points;
}
