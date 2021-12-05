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
    print_summary(gamma_rate_bits, epsilon_rate_bits);
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
    if n1 > n0 {
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

fn print_summary(gamma_rate_bits: String, epsilon_rate_bits: String) {
    let gamma_rate = isize::from_str_radix(gamma_rate_bits.as_str(), 2).unwrap();
    let epsilon_rate = isize::from_str_radix(epsilon_rate_bits.as_str(), 2).unwrap();

    println!("bin gamma rate {}", gamma_rate_bits);
    println!("int gamma rate {}", gamma_rate);
    println!("bin epsilon rate {}", epsilon_rate_bits);
    println!("int epsilon rate {}", epsilon_rate);
    println!("Result {}", gamma_rate * epsilon_rate);
}
