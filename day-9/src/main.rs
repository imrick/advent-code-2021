use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
pub struct Bassin {
    points: Vec<Point>,
}

fn main() {
    let map = read_data("./input-full.txt");
    let max_x = map[0].len() - 1;
    let max_y = map.len() - 1;
    let mut lowest_points: Vec<i32> = Vec::new();
    let mut bassins: Vec<Bassin> = Vec::new();

    for y_pos in 0..map.len() {
        let mut prev_line = Vec::new();
        let mut next_line = Vec::new();
        if y_pos > 0 {
            prev_line = map[y_pos - 1].clone();
        }
        if y_pos + 1 < map.len() {
            next_line = map[y_pos + 1].clone();
        }
        let line = map[y_pos].clone();
        for x_pos in 0..map[0].len() {
            if has_lowest_value_on(
                x_pos,
                y_pos,
                max_x,
                max_y,
                line.clone(),
                prev_line.clone(),
                next_line.clone(),
            ) {
                lowest_points.push(line[x_pos]);
                bassins.push(Bassin {
                    points: vec![Point {
                        x: x_pos as i32,
                        y: y_pos as i32,
                    }],
                });
            }
        }
    }

    println!(
        "Total risk is {:?}",
        lowest_points.iter().map(|p| p + 1).sum::<i32>()
    );

    for i in 0..bassins.clone().len() {
        let lowest_point = bassins[i].clone().points[0].clone();
        bassins[i] = add_closes_bassin_points(
            bassins[i].clone(),
            lowest_point.x,
            lowest_point.y,
            map.clone(),
        )
    }

    let mut bassins_sizes = bassins
        .iter()
        .map(|b| b.points.len())
        .collect::<Vec<usize>>();
    bassins_sizes.sort();
    bassins_sizes.reverse();

    // println!("Bassins {:?}", bassins);
    println!(
        "Bassins sizes {:?}",
        bassins_sizes[0..3]
            .iter()
            .fold(0, |r, &nb| if r > 0 { r * nb } else { nb })
    );
}

pub fn read_data(path: &str) -> Vec<Vec<i32>> {
    let mut map = Vec::new();
    for line in read_lines(path).unwrap() {
        map.push(
            line.chars()
                .map(|n| n.to_string().parse::<i32>().unwrap())
                .collect(),
        );
    }
    return map;
}

pub fn read_lines(path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;
    io::BufReader::new(file).lines().collect()
}

pub fn has_lowest_value_on(
    x_pos: usize,
    y_pos: usize,
    max_x: usize,
    max_y: usize,
    line: Vec<i32>,
    prev_line: Vec<i32>,
    next_line: Vec<i32>,
) -> bool {
    let val = line[x_pos];
    let w_val = get_val_at_pos(x_pos as i32 > 0, line.clone(), x_pos as i32 - 1);
    let e_val = get_val_at_pos(x_pos < max_x, line.clone(), x_pos as i32 + 1);
    let n_val = get_val_at_pos(y_pos as i32 > 0, prev_line.clone(), x_pos as i32);
    let s_val = get_val_at_pos(y_pos < max_y, next_line.clone(), x_pos as i32);

    val < w_val && val < e_val && val < n_val && val < s_val
}

pub fn get_val_at_pos(test_pos: bool, data: Vec<i32>, pos_val: i32) -> i32 {
    if test_pos {
        data[pos_val as usize]
    } else {
        10
    }
}

pub fn add_closes_bassin_points(
    mut bassin: Bassin,
    xpos: i32,
    ypos: i32,
    map: Vec<Vec<i32>>,
) -> Bassin {
    bassin = add_bassin_point_at_pos(bassin, xpos - 1, ypos, map.clone());
    bassin = add_bassin_point_at_pos(bassin, xpos + 1, ypos, map.clone());
    bassin = add_bassin_point_at_pos(bassin, xpos, ypos - 1, map.clone());
    bassin = add_bassin_point_at_pos(bassin, xpos, ypos + 1, map.clone());
    bassin
}

pub fn add_bassin_point_at_pos(
    mut bassin: Bassin,
    xpos: i32,
    ypos: i32,
    map: Vec<Vec<i32>>,
) -> Bassin {
    let max_x = map[0].len() as i32 - 1;
    let max_y = map.len() as i32 - 1;
    let point_exist = xpos >= 0 && xpos <= max_x && ypos >= 0 && ypos <= max_y;
    let not_in_bassin = bassin
        .points
        .iter()
        .find(|p| p.x == xpos && p.y == ypos)
        .is_none();
    if not_in_bassin && point_exist && map[ypos as usize][xpos as usize] < 9 {
        bassin.points.push(Point { x: xpos, y: ypos });
        bassin = add_closes_bassin_points(bassin, xpos, ypos, map);
    }
    bassin
}
