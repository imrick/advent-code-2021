use std::fs::File;
use std::io::{self, BufRead};

struct FrequentsBits {
    most_frequent: char,
    least_frequent: char,
}

fn main() {
    let lines: Vec<Vec<char>> = read_lines("./input-full.txt")
        .unwrap()
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut gamma_rate_bits = String::from("");
    let mut epsilon_rate_bits = String::from("");
    let ranks = lines.clone()[0].len();
    for i in 0..ranks {
        let frequents_bits = frequents_at_rank(lines.clone(), i);
        gamma_rate_bits.push(frequents_bits.most_frequent);
        epsilon_rate_bits.push(frequents_bits.least_frequent);
    }

    let o2_rate_bits = find_bits_by_frequency_type(lines.clone(), ranks, |fc| fc.most_frequent);
    let co2_rate_bits = find_bits_by_frequency_type(lines.clone(), ranks, |fc| fc.least_frequent);

    print_result(gamma_rate_bits, epsilon_rate_bits, o2_rate_bits, co2_rate_bits);
}

pub fn read_lines(path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;
    io::BufReader::new(file).lines().collect()
}

fn frequents_at_rank(lines: Vec<Vec<char>>, rank: usize) -> FrequentsBits {
    let mut n0 = 0;
    let mut n1 = 0;
    for line in lines {
        n1 += if line[rank] == '1' { 1 } else { 0 };
        n0 += if line[rank] == '0' { 1 } else { 0 };
    }
    if n1 >= n0 {
        FrequentsBits {
            most_frequent: '1',
            least_frequent: '0',
        }
    } else {
        FrequentsBits {
            most_frequent: '0',
            least_frequent: '1',
        }
    }
}

fn print_result(gamma_rate_bits: String, epsilon_rate_bits: String, o2_rate_bits: String, co2_rate_bits: String) {

    let gamma_rate = parse_str_bits_into_int(&gamma_rate_bits);
    let epsilon_rate = parse_str_bits_into_int(&epsilon_rate_bits);
    let o2_rate = parse_str_bits_into_int(&o2_rate_bits);
    let co2_rate = parse_str_bits_into_int(&co2_rate_bits);

    println!("Gamma rate:  {} -> {}", gamma_rate_bits, gamma_rate);
    println!("Epsilon rate:  {} -> {}", epsilon_rate_bits, epsilon_rate);
    println!("O2 rate:  {} -> {}", o2_rate_bits, o2_rate);
    println!("CO2:  {} -> {}", co2_rate_bits, co2_rate);

    println!("Power consumption is {}, life support is {}", gamma_rate * epsilon_rate, o2_rate * co2_rate);
}

fn parse_str_bits_into_int(str_bits: &str) -> isize {
    isize::from_str_radix(str_bits, 2).unwrap()
}

fn extract_matching_lines_at_rank(
    lines: Vec<Vec<char>>,
    matching_char: char,
    rank: usize,
) -> Vec<Vec<char>> {
    let mut matching_lines: Vec<Vec<char>> = Vec::new();
    for line in lines {
        if line[rank] == matching_char {
            matching_lines.push(line.clone())
        }
    }
    return matching_lines;
}

fn find_bits_by_frequency_type(
    lines: Vec<Vec<char>>,
    ranks: usize,
    frequency_type_selector: fn(FrequentsBits) -> char,
) -> String {
    let mut co2_lines = lines.clone();
    let mut co2_rate_bits = String::from("");
    for i in 0..ranks {
        co2_lines = extract_matching_lines_at_rank(
            co2_lines.clone(),
            frequency_type_selector(frequents_at_rank(co2_lines.clone(), i)),
            i,
        );
        if co2_lines.clone().len() <= 1 {
            co2_rate_bits = co2_lines.into_iter().flat_map(|c| c).collect();
            break;
        }
    }
    return co2_rate_bits;
}
