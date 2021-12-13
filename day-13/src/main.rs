use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let (mut dots, instructions) = read_data("./input-full.txt");
    for instruction in instructions {
        let (axe, pos) = instruction;
        let mut new_dots: Vec<[u16; 2]> = dots
            .clone()
            .into_iter()
            .filter(|d| if axe == "x" { d[0] < pos } else { d[1] < pos })
            .collect();
        let dots_to_move: Vec<[u16; 2]> = dots
            .clone()
            .into_iter()
            .filter(|d| if axe == "x" { d[0] > pos } else { d[1] > pos })
            .collect();

        let dots_moved: Vec<[u16; 2]> = dots_to_move
            .clone()
            .into_iter()
            .map(|d| {
                if axe == "x" {
                    [pos - (d[0] - pos), d[1]]
                } else {
                    [d[0], pos - (d[1] - pos)]
                }
            })
            .collect();

        for dot_moved in dots_moved {
            if new_dots
                .iter()
                .find(|nd| nd[0] == dot_moved[0] && nd[1] == dot_moved[1])
                .is_none()
            {
                new_dots.push(dot_moved);
            }
        }

        dots = new_dots;
    }

    let max_x = dots
        .clone()
        .into_iter()
        .fold(0, |r, d| if d[0] > r { d[0] } else { r });
    let max_y = dots
        .clone()
        .into_iter()
        .fold(0, |r, d| if d[1] > r { d[1] } else { r });

    (0..max_y + 1).for_each(|posy| {
        let mut line: String = String::from("");
        (0..max_x + 1).for_each(|posx| {
            if dots
                .iter()
                .find(|d| d[0] == posx && d[1] == posy)
                .is_some()
            {
                line.push('#');
            } else {
                line.push('.');
            }
        });
        println!("{:?}", line);
    });
}

pub fn read_data(path: &str) -> (Vec<[u16; 2]>, Vec<(String, u16)>) {
    let mut dots: Vec<[u16; 2]> = Vec::new();
    let mut fold_instructions: Vec<(String, u16)> = Vec::new();
    for line in read_lines(path).unwrap() {
        if line.contains(",") {
            let numbers = line
                .split(",")
                .map(|s| s.parse::<u16>().unwrap())
                .collect::<Vec<u16>>();
            dots.push([numbers[0], numbers[1]])
        } else if line.contains("fold along") {
            let instruction_parts = line
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let instruction_pos = instruction_parts[2]
                .split("=")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            fold_instructions.push((
                instruction_pos[0].clone(),
                instruction_pos[1].parse::<u16>().unwrap(),
            ));
        }
    }
    (dots, fold_instructions)
}

pub fn read_lines(path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;
    io::BufReader::new(file).lines().collect()
}
