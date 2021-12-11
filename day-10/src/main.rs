use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
pub struct Chunk {
    childs: Vec<Chunk>,
    opening_char: String,
    closing_char: String,
    corrupted: bool,
    line_corrupted: bool,
    illegal_char: String,
    incomplete: bool,
}

#[derive(Debug, Clone)]
pub struct ChunkLine {
    valid_delimiters: Vec<String>,
    fisrt_illegal_char: String,
    missing_close_delimiters: Vec<String>,
}

// TODO TU + refacto
// part 1 test 26397
// part 2 test 288957
// part 1 full 321237
// part 2 full 2360030859
fn main() {
    let lines = read_lines("./input-test.txt").unwrap();
    let mut chunks_lines: Vec<ChunkLine> = Vec::new();

    for line in lines {
        let mut chunk_line = ChunkLine {
            valid_delimiters: Vec::new(),
            fisrt_illegal_char: String::from(""),
            missing_close_delimiters: Vec::new(),
        };
        for c in line.chars() {
            match match_delimiter(c.to_string(), 0) {
                Some(_) => chunk_line.valid_delimiters.push(c.to_string()),
                None => match match_delimiter(c.to_string(), 1) {
                    Some(delimiters) => {
                        if delimiters[0].clone()
                            == chunk_line.valid_delimiters.last().unwrap().clone()
                        {
                            chunk_line
                                .valid_delimiters
                                .remove(chunk_line.valid_delimiters.len() - 1);
                        } else {
                            chunk_line.fisrt_illegal_char = c.to_string();
                            break;
                        }
                    }
                    None => panic!("Unknown delimiter {}", c),
                },
            }
        }
        chunks_lines.push(chunk_line);
    }

    println!(
        "Result part 1: {}",
        chunks_lines
            .iter()
            .filter(|cl| cl.fisrt_illegal_char != "")
            .map(|cl| cl.fisrt_illegal_char.clone())
            .map(|c| get_delimiters_scores()[&c])
            .sum::<i64>()
    );

    let uncompleted_lines = chunks_lines
        .into_iter()
        .filter(|cl| cl.fisrt_illegal_char == "")
        .collect::<Vec<ChunkLine>>();

    let mut missing_scores: Vec<i64> = Vec::new();
    for mut line in uncompleted_lines {
        for c in line
            .valid_delimiters
            .into_iter()
            .rev()
            .collect::<Vec<String>>()
        {
            line.missing_close_delimiters
                .push(match_delimiter(c.to_string(), 0).unwrap()[1].clone());
        }
        missing_scores.push(
            line.missing_close_delimiters
                .into_iter()
                .fold(0, |score, c| {
                    (score * 5) + get_missing_delimiters_scores()[&c]
                }),
        );
    }

    missing_scores.sort();
    println!(
        "Result part2 : {:?}",
        missing_scores[missing_scores.len() / 2]
    );
}
pub fn read_lines(path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;
    io::BufReader::new(file).lines().collect()
}

pub fn match_delimiter(c: String, pos: usize) -> Option<[String; 2]> {
    get_delimiters().into_iter().find(|d| d[pos] == c)
}

pub fn get_delimiters() -> Vec<[String; 2]> {
    vec![
        [String::from("("), String::from(")")],
        [String::from("{"), String::from("}")],
        [String::from("["), String::from("]")],
        [String::from("<"), String::from(">")],
    ]
}

pub fn get_delimiters_scores() -> HashMap<String, i64> {
    HashMap::from([
        (String::from(")"), 3),
        (String::from("]"), 57),
        (String::from("}"), 1197),
        (String::from(">"), 25137),
    ])
}

pub fn get_missing_delimiters_scores() -> HashMap<String, i64> {
    HashMap::from([
        (String::from(")"), 1),
        (String::from("]"), 2),
        (String::from("}"), 3),
        (String::from(">"), 4),
    ])
}
