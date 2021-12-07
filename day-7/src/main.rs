use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let crabs: Vec<i32> = read_data("./input-full.txt");
    let mut fuel_dispense: HashMap<usize, i32> = HashMap::new();
    
    (0..crabs.len()).for_each(|crab_idx| {
        println!("Compute cost at pos {}", crab_idx);
        fuel_dispense.insert(
            crab_idx,
            crabs
                .iter()
                .map(|c| (c - crab_idx as i32).abs())
                .map(|steps| (1..steps + 1).fold(0, |cost, i| cost + i))
                .sum(),
        );
    });

    let mut list_expenses = fuel_dispense.iter().map(|a| a.1).collect::<Vec<_>>();
    list_expenses.sort();
    println!("Lowest cost {:?}", list_expenses[0]);
}

pub fn read_data(path: &str) -> Vec<i32> {
    let lines: Vec<String> = read_lines(path).unwrap();
    return lines[0]
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();
}

pub fn read_lines(path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;
    io::BufReader::new(file).lines().collect()
}
