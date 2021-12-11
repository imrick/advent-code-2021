use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut map = read_data("./input-full.txt");
    let mut map2 = map.clone();
    let mut total_flash_count = 0;
    // part 1
    (1..101).for_each(|step| total_flash_count += run_step(step, &mut map));
    println!("total_flash_count part 1 {}", total_flash_count);
    // part 2
    let mut step: usize = 0;
    while !is_all_light_flashes(map2.clone()) {
        step += 1;
        run_step(step, &mut map2);
    }
    println!("number of steps part 2 : {}", step);
}

pub fn read_data(path: &str) -> Vec<Vec<i8>> {
    let mut map = Vec::new();
    for line in read_lines(path).unwrap() {
        map.push(
            line.chars()
                .map(|n| n.to_string().parse::<i8>().unwrap())
                .collect(),
        );
    }
    return map;
}

pub fn read_lines(path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;
    io::BufReader::new(file).lines().collect()
}

pub fn run_step(_step: usize, map: &mut Vec<Vec<i8>>) -> u16 {
    increase_points(map);
    let mut flash_count_step = 0;
    for y_pos in 0..map.len() {
        for x_pos in 0..map[0].len() {
            flash_count_step += flash_point(y_pos as i8, x_pos as i8, map);
        }
    }
    // println!("step {} flash_count_step {}", step, flash_count_step);
    // print_map(map.clone());
    flash_count_step
}

pub fn increase_points(map: &mut Vec<Vec<i8>>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            map[y][x] += 1;
        }
    }
}

pub fn flash_point(xpos: i8, ypos: i8, map: &mut Vec<Vec<i8>>) -> u16 {
    let mut flash_count = 0;
    if map[ypos as usize][xpos as usize] > 9 {
        flash_count += 1;
        map[ypos as usize][xpos as usize] = 0;
        flash_count += flash_point_at_pos(xpos, ypos - 1, map); // n
        flash_count += flash_point_at_pos(xpos + 1, ypos - 1, map); // ne
        flash_count += flash_point_at_pos(xpos + 1, ypos, map); // e
        flash_count += flash_point_at_pos(xpos + 1, ypos + 1, map); // se
        flash_count += flash_point_at_pos(xpos, ypos + 1, map); // s
        flash_count += flash_point_at_pos(xpos - 1, ypos + 1, map); // sw
        flash_count += flash_point_at_pos(xpos - 1, ypos, map); // w
        flash_count += flash_point_at_pos(xpos - 1, ypos - 1, map); // nw
    }
    flash_count
}

pub fn flash_point_at_pos(xpos: i8, ypos: i8, map: &mut Vec<Vec<i8>>) -> u16 {
    let mut flash_count = 0;
    let max_x = map[0].len() - 1;
    let max_y = map.len() - 1;
    let point_exist = xpos >= 0 && xpos <= max_x as i8 && ypos >= 0 && ypos <= max_y as i8;
    if point_exist && map[ypos as usize][xpos as usize] != 0 {
        map[ypos as usize][xpos as usize] += 1;
        flash_count += flash_point(xpos, ypos, map);
    }
    flash_count
}

pub fn print_map(map: Vec<Vec<i8>>) {
    for line in map {
        println!(
            "{}",
            line.iter()
                .fold(String::from(""), |l, p| l + &p.to_string())
        )
    }
}

pub fn is_all_light_flashes(map: Vec<Vec<i8>>) -> bool {
    map.iter().fold(0, |sum_lines, line| {
        sum_lines + line.iter().fold(0, |sum_line, p| sum_line + p)
    }) == 0
}
