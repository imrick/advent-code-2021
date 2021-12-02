use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct SubPosition {
    horizontal: i32,
    depth: i32,
}

#[derive(Debug)]
struct Instruction {
    direction: String,
    velocity: i32,
}

fn main() {
    let mut position = SubPosition {
        horizontal: 0,
        depth: 0,
    };
    let mut current_aim: i32 = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(input) = line {
                let mut input_iter: Vec<String> =
                    input.split_whitespace().map(str::to_string).collect();
                let instruction = Instruction {
                    direction: input_iter.remove(0),
                    velocity: input_iter.remove(0).parse::<i32>().unwrap(),
                };
                match instruction.direction.as_str() {
                    "forward" => {
                        position.horizontal += instruction.velocity;
                        position.depth += current_aim * instruction.velocity;
                    }
                    "down" => {
                        current_aim += instruction.velocity
                    }
                    "up" => {
                        current_aim -= instruction.velocity
                    }
                    unknown => {
                        panic!("Unknown direction {}", unknown);
                    }
                }
            }
        }
    }
    println!(
        "Result is {} for {:?}",
        position.depth * position.horizontal,
        position
    )
}

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
