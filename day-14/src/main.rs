use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let (template, instructions) = read_data("./input-full.txt");
    let mut formula = template.clone();

    (0..40).for_each(|_step| {
        let mut insertions: Vec<(usize, char)> = Vec::new();
        println!("Start step {:?}", _step);
        
        for rank in (0..formula.len() - 1).step_by(1) {
            for instruction in instructions.clone() {
                // println!("instruction {:?}", instruction.clone().0);
                // println!("couple {:?}", formula[rank..rank+1].to_string());
                if formula[rank..rank + 2].to_string() == instruction.0 {
                    insertions.push((rank + 1, instruction.1));
                }
            }
        }
        // println!("insertions {:?}", insertions.clone());
        let mut rank_offset: usize = 0;
        for insertion in insertions {
            formula.insert(insertion.0 + rank_offset, insertion.1);
            rank_offset += 1;
        }
        println!("End step {:?}", _step);
    });
    // println!("template {:?} formula {:?}", template, formula);
    
    let mut char_counter: HashMap<char, u16> = HashMap::new();
    for c in formula.chars() {
        if char_counter.contains_key(&c) {
            *char_counter.get_mut(&c).unwrap() += 1;
        } else {
            char_counter.insert(c, 1);
        }
    }
    let mut ordered_chars = char_counter.into_iter().map(|c| c.1).collect::<Vec<u16>>();
    ordered_chars.sort();
    println!("char_counter {:?} ", ordered_chars);
    println!("Result is {:?} ", ordered_chars.last().unwrap() - ordered_chars.first().unwrap());
}

pub fn read_data(path: &str) -> (String, Vec<(String, char)>) {
    let lines = read_lines(path).unwrap();
    let template = lines[0].clone();
    let mut instructions: Vec<(String, char)> = Vec::new();
    for i in 2..lines.len() {
        let instruction_parts = lines[i]
            .clone()
            .split(" -> ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        instructions.push((
            instruction_parts[0].clone(),
            instruction_parts[1].clone().parse::<char>().unwrap(),
        ));
    }
    (template, instructions)
}

pub fn read_lines(path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;
    io::BufReader::new(file).lines().collect()
}
