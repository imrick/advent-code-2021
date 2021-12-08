use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
pub struct DigitSequence {
    part1: Vec<String>,
    part2: Vec<String>,
}

fn main() {
    let mut digits_part1: HashMap<u8, u16> = HashMap::new();
    let sequenses = read_data("./input-full.txt");
    for sequence in sequenses.clone() {
        digits_part1 = extract_unique_digits_count(digits_part1, sequence.part2);
    }
    println!(
        "result part 1 is {}",
        digits_part1.iter().map(|d| d.1).sum::<u16>()
    );

    let mut decoded_sequences: Vec<String> = Vec::new();
    for sequence in sequenses.clone() {
        decoded_sequences.push(decode_sequence(
            sequence.part2,
            extract_digits_pattern(sequence.part1),
        ));
    }
    println!(
        "Result part2 is {}",
        decoded_sequences
            .iter()
            .map(|s| s.parse::<u32>().unwrap())
            .sum::<u32>()
    );
}

pub fn read_data(path: &str) -> Vec<DigitSequence> {
    let mut sequenses = Vec::new();
    for line in read_lines(path).unwrap() {
        let digits_parts: Vec<&str> = line.split(" | ").collect();
        let part1: Vec<String> = digits_parts[0].split(' ').map(|s| s.to_string()).collect();
        let part2: Vec<String> = digits_parts[1].split(' ').map(|s| s.to_string()).collect();

        sequenses.push(DigitSequence { part1, part2 });
    }
    return sequenses;
}

pub fn read_lines(path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;
    io::BufReader::new(file).lines().collect()
}

/**
 * include unique pattern first
 * then use unique pattern to deduce others by matching segments
 */
pub fn extract_digits_pattern(sequence: Vec<String>) -> HashMap<u8, String> {
    let mut digits_patterns: HashMap<u8, String> = HashMap::new();

    for packet in sequence.clone() {
        if packet.len() == 2 {
            digits_patterns.insert(1, packet);
        } else if packet.len() == 4 {
            digits_patterns.insert(4, packet);
        } else if packet.len() == 3 {
            digits_patterns.insert(7, packet);
        } else if packet.len() == 7 {
            digits_patterns.insert(8, packet);
        }
    }

    for packet in sequence.clone() {
        if packet.len() == 6 {
            if check_pattern(packet.clone(), digits_patterns[&1].clone(), 2)
                && check_pattern(packet.clone(), digits_patterns[&7].clone(), 3)
                && check_pattern(packet.clone(), digits_patterns[&4].clone(), 3)
            {
                digits_patterns.insert(0, packet);
            } else if check_pattern(packet.clone(), digits_patterns[&1].clone(), 1)
                && check_pattern(packet.clone(), digits_patterns[&7].clone(), 2)
                && check_pattern(packet.clone(), digits_patterns[&4].clone(), 3)
            {
                digits_patterns.insert(6, packet);
            } else if check_pattern(packet.clone(), digits_patterns[&7].clone(), 3)
                && check_pattern(packet.clone(), digits_patterns[&4].clone(), 4)
            {
                digits_patterns.insert(9, packet);
            }
        }
    }
    for packet in sequence.clone() {
        if packet.len() == 5 {
            if check_pattern(packet.clone(), digits_patterns[&6].clone(), 5) {
                digits_patterns.insert(5, packet);
            } else if check_pattern(packet.clone(), digits_patterns[&1].clone(), 2)
                && check_pattern(packet.clone(), digits_patterns[&7].clone(), 3)
                && check_pattern(packet.clone(), digits_patterns[&4].clone(), 3)
            {
                digits_patterns.insert(3, packet);
            } else if check_pattern(packet.clone(), digits_patterns[&1].clone(), 1)
                && check_pattern(packet.clone(), digits_patterns[&7].clone(), 2)
                && check_pattern(packet.clone(), digits_patterns[&4].clone(), 2)
                && check_pattern(packet.clone(), digits_patterns[&6].clone(), 4)
            {
                digits_patterns.insert(2, packet);
            }
        }
    }
    digits_patterns
}

pub fn decode_sequence(sequence: Vec<String>, digits_patterns: HashMap<u8, String>) -> String {
    let mut decoded_sequence: String = String::from("");
    for packet in sequence {
        let digit = digits_patterns
            .iter()
            .find(|p| {
                packet.len() == p.1.len()
                    && check_pattern(packet.clone(), p.1.to_string(), packet.len() as u8)
            })
            .unwrap();
        decoded_sequence.push_str(&digit.0.to_string());
    }
    decoded_sequence
}

pub fn extract_unique_digits_count(
    mut digits: HashMap<u8, u16>,
    sequence: Vec<String>,
) -> HashMap<u8, u16> {
    for packet in sequence {
        if packet.len() == 2 {
            digits = upsert_digit_count(digits, 1);
        } else if packet.len() == 4 {
            digits = upsert_digit_count(digits, 4);
        } else if packet.len() == 3 {
            digits = upsert_digit_count(digits, 7);
        } else if packet.len() == 7 {
            digits = upsert_digit_count(digits, 8);
        }
    }
    digits
}

pub fn upsert_digit_count(mut map: HashMap<u8, u16>, key: u8) -> HashMap<u8, u16> {
    if map.contains_key(&key) {
        (*map.get_mut(&key).unwrap()) += 1;
    } else {
        map.insert(key, 1);
    }
    map
}

pub fn check_pattern(subjet: String, pattern: String, expected: u8) -> bool {
    let nb_match = pattern
        .chars()
        .collect::<Vec<char>>()
        .into_iter()
        .fold(0, |mut result, c| {
            if subjet.contains(c) {
                result += 1
            }
            result
        });
    expected == nb_match
}
