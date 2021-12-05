use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
pub struct Board {
    numbers: Vec<u32>,
    solutions: Vec<Vec<u32>>,
    score: u32,
    wining_solution: Vec<u32>,
}

fn main() {
    let (drawned_numbers, boards) = read_data("./input-full.txt");

    let drawn_total = drawned_numbers.len();
    let board_total = boards.len();
    let mut winning_boards: Vec<Board> = Vec::new();
    let mut playing_board = boards.clone();

    for num_index in 5..drawn_total {
        let tested_numbers = drawned_numbers[0..num_index + 1].to_vec();
        let mut next_playing_boards: Vec<Board> = Vec::new();

        for n in 0..playing_board.len() {
            let matched_solution = playing_board[n].solutions.iter().find(|s| {
                return s
                    .iter()
                    .filter(|sn| tested_numbers.iter().find(|tn| tn == sn).is_some())
                    .collect::<Vec<_>>()
                    .len()
                    == 5;
            });

            if matched_solution.is_some() {
                playing_board[n].wining_solution = matched_solution.unwrap().to_vec();
                let sum: u32 = playing_board[n]
                    .numbers
                    .iter()
                    .filter(|nb| tested_numbers.iter().find(|wn| wn == nb).is_none())
                    .sum();
                playing_board[n].score = sum * tested_numbers[num_index];
                winning_boards.push(playing_board[n].clone());
            } else {
                next_playing_boards.push(playing_board[n].clone());
            }
        }

        playing_board = next_playing_boards.clone();

        if winning_boards.len() >= board_total {
            break;
        }
    }
    println!("Total boards {:?}", board_total);
    println!("Nb board to win {:?}", winning_boards.len());
    println!("First board to win {:?}", winning_boards[0]);
    println!("Last board to win {:?}", winning_boards[board_total - 1]);
}

pub fn read_data(path: &str) -> (Vec<u32>, Vec<Board>) {
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

pub fn extract_drawned_numbers(line: String) -> Vec<u32> {
    return line
        .split(',')
        .map(|s| s.to_string().parse::<u32>().unwrap())
        .collect();
}

pub fn extract_boards(boards_lines: Vec<String>) -> Vec<Board> {
    let mut boards: Vec<Board> = vec![];
    let boards_count = boards_lines.clone().len() / 6;

    for n in 0..boards_count {
        let board_start_line = n * 6 + 1;
        let board_end_line = board_start_line + 5;
        let board_lines: Vec<Vec<u32>> = boards_lines[board_start_line..board_end_line]
            .to_vec()
            .iter()
            .map(|s| {
                s.trim()
                    .split_whitespace()
                    .map(|s| s.to_string().parse::<u32>().unwrap())
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
            let mut row_solutions: Vec<u32> = Vec::new();
            let mut col_solutions: Vec<u32> = Vec::new();
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
