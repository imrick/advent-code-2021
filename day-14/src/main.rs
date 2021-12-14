use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let (template, instructions) = read_data("./input-full.txt");
    let mut char_counter: HashMap<char, u64> = HashMap::new();
    let mut pairs_map: HashMap<String, u64> = HashMap::new();
    for rank in (0..template.clone().len() - 1).step_by(1) {
        register_pair(&mut pairs_map, template[rank..rank + 2].to_string(), 1);
    }

    for c in template.chars() {
        increase_char_counter(c, &mut char_counter, 1);
    }

    (0..40).for_each(|_step| {
        let mut new_pairs_map: HashMap<String, u64> = HashMap::new();
        for pair in pairs_map.clone().iter() {
            let mut matched = false;
            for instruction in &instructions {
                if pair.0.to_string() == instruction.0 {
                    matched = true;
                    register_pair(&mut new_pairs_map, instruction.2.clone(), pair.1.clone());
                    register_pair(&mut new_pairs_map, instruction.3.clone(), pair.1.clone());
                    increase_char_counter(instruction.1, &mut char_counter, pair.1.clone());
                    break;
                }
            }
            if !matched {
                new_pairs_map.insert(pair.0.to_string(), pair.1.clone());
            }
        }
        pairs_map = new_pairs_map;
    });
    let mut ordered_chars = char_counter.into_iter().map(|c| c.1).collect::<Vec<u64>>();
    ordered_chars.sort();
    println!("char_counter {:?} ", ordered_chars);
    println!(
        "Result is {:?} ",
        ordered_chars.last().unwrap() - ordered_chars.first().unwrap()
    );
}

pub fn register_pair(pairs_map: &mut HashMap<String, u64>, pair: String, count: u64) {
    if pairs_map.contains_key(&pair) {
        *pairs_map.get_mut(&pair).unwrap() += count;
    } else {
        pairs_map.insert(pair, count);
    }
}

pub fn increase_char_counter(c: char, char_counter: &mut HashMap<char, u64>, count: u64) {
    if char_counter.contains_key(&c) {
        *char_counter.get_mut(&c).unwrap() += count;
    } else {
        char_counter.insert(c, count);
    }
}

pub fn read_data(path: &str) -> (String, Vec<(String, char, String, String)>) {
    let lines = read_lines(path).unwrap();
    let template = lines[0].clone();
    let mut instructions: Vec<(String, char, String, String)> = Vec::new();
    for i in 2..lines.len() {
        let instruction_parts = lines[i]
            .clone()
            .split(" -> ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let pair_char: Vec<char> = instruction_parts[0].clone().chars().collect();
        instructions.push((
            instruction_parts[0].clone(),
            instruction_parts[1].clone().parse::<char>().unwrap(),
            format!("{}{}", pair_char[0], instruction_parts[1].clone()),
            format!("{}{}", instruction_parts[1].clone(), pair_char[1]),
        ));
    }
    (template, instructions)
}

pub fn read_lines(path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;
    io::BufReader::new(file).lines().collect()
}
