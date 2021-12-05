use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
pub struct Board {
    numbers: Vec<u16>,
    solutions: Vec<Vec<u16>>,
    score: u16,
    wining_solution: Vec<u16>,
}

fn main() {
    let (drawned_numbers, boards) = read_data("./input-full.txt");

    let drawn_total = drawned_numbers.len();
    let mut winning_boards: Vec<Board> = Vec::new();

    for n in 5..drawn_total {
        let tested_numbers = drawned_numbers[0..n + 1].to_vec();

        for mut board in boards.clone() {
            let matched_solution = board.solutions.iter().find(|s| {
                return s
                    .iter()
                    .filter(|sn| tested_numbers.iter().find(|tn| tn == sn).is_some())
                    .collect::<Vec<_>>()
                    .len()
                    == 5;
            });

            if matched_solution.is_some() {
                board.wining_solution = matched_solution.unwrap().to_vec();
                let sum: u16 = board
                    .numbers
                    .iter()
                    .filter(|nb| tested_numbers.iter().find(|wn| wn == nb).is_none())
                    .sum();
                board.score = sum * tested_numbers[n];
                winning_boards.push(board);
            }
        }

        if winning_boards.len() > 0 {
            break;
        }
    }
    println!("{:?}", winning_boards);
}

pub fn read_data(path: &str) -> (Vec<u16>, Vec<Board>) {
    let lines: Vec<String> = read_lines(path).unwrap();

    return (
        extract_drawned_numbers(lines[0].clone()),
        extract_boards(lines[1..].to_vec()),
    );
}

pub fn read_lines(path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;
    io::BufReader::new(file).lines().collect()
}

pub fn extract_drawned_numbers(line: String) -> Vec<u16> {
    return line
        .split(',')
        .map(|s| s.to_string().parse::<u16>().unwrap())
        .collect();
}

pub fn extract_boards(boards_lines: Vec<String>) -> Vec<Board> {
    let mut boards: Vec<Board> = vec![];
    let boards_count = boards_lines.clone().len() / 6;

    for n in 0..boards_count {
        let board_start_line = n * 6 + 1;
        let board_end_line = board_start_line + 5;
        let board_lines: Vec<Vec<u16>> = boards_lines[board_start_line..board_end_line]
            .to_vec()
            .iter()
            .map(|s| {
                s.trim()
                    .split_whitespace()
                    .map(|s| s.to_string().parse::<u16>().unwrap())
                    .collect()
            })
            .collect();

        let mut board = Board {
            numbers: Vec::new(),
            solutions: Vec::new(),
            score: 0,
            wining_solution: Vec::new(),
        };
        for h in 0..5 {
            let mut row_solutions: Vec<u16> = Vec::new();
            let mut col_solutions: Vec<u16> = Vec::new();
            for l in 0..5 {
                board.numbers.push(board_lines[h][l]);
                row_solutions.push(board_lines[h][l]);
                col_solutions.push(board_lines[l][h])
            }
            board.solutions.push(row_solutions);
            board.solutions.push(col_solutions);
        }
        boards.push(board)
    }

    return boards;
}
