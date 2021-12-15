use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};

const DEFAULT_MAX_RISK: i16 = 10000;

fn main() {
    let map = read_data("./input-full.txt");
    let expanded_map = expand_map(&map, 5);
    println!("Result part 1 is : {:?}", compute_safer_path_risk(&map));
    println!("Result part 2 is : {:?}", compute_safer_path_risk(&expanded_map));
}

fn compute_safer_path_risk(map: &Vec<Vec<i16>>) -> i16 {
    let l_map = map[0].len();
    let h_map = map.len();
    let mut min_risk_map = vec![vec![DEFAULT_MAX_RISK; l_map]; h_map];
    let mut points_queue = BinaryHeap::from([(0, 0, 0)]);
    let dest = (l_map - 1, h_map - 1);

    let mut safer_path_risk = 0;
    while let Some((total_risk, x, y)) = points_queue.pop() {
        if (x, y) == dest {
            safer_path_risk = -total_risk;
            break;
        }

        for (nx, ny) in get_neighbours(x as i16, y as i16, l_map, h_map) {
            let total_neighbour_risk = -total_risk + map[nx][ny];
            if total_neighbour_risk < min_risk_map[nx][ny] {
                min_risk_map[nx][ny] = total_neighbour_risk;
                points_queue.push((-total_neighbour_risk, nx, ny));
            }
        }
    }
    safer_path_risk
}

pub fn get_neighbours(x: i16, y: i16, l_map: usize, h_map: usize) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();
    for (nx, ny) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
        if nx >= 0 && nx < l_map as i16 && ny >= 0 && ny < h_map as i16 {
            neighbours.push((nx as usize, ny as usize));
        }
    }
    neighbours
}

pub fn expand_map(map: &Vec<Vec<i16>>, expansion_factor: usize) -> Vec<Vec<i16>> {
    let l_map = map[0].len();
    let h_map = map.len();
    (0..(expansion_factor * h_map))
        .map(|y| {
            (0..(expansion_factor * l_map))
                .map(|x| {
                    let increase_x = (x / l_map) as i16;
                    let increase_y = (y / h_map) as i16;
                    let cost_ref = map[y % h_map][x % l_map];
                    let cost = cost_ref + increase_x + increase_y;

                    if cost < 10 {
                        cost
                    } else {
                        cost - 9
                    }
                })
                .collect::<Vec<i16>>()
        })
        .collect::<Vec<Vec<i16>>>()
}

pub fn read_lines(path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;
    io::BufReader::new(file).lines().collect()
}

pub fn read_data(path: &str) -> Vec<Vec<i16>> {
    read_lines(path)
        .unwrap()
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| (c as u8 - b'0') as i16)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn print_map(map: &Vec<Vec<i16>>) {
    for line in map {
        println!(
            "{:?}",
            line.iter()
                .fold(String::from(""), |s, c| format!("{}{}", s, c))
        );
    }
}
