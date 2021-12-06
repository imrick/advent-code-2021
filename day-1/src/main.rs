use std::iter::Iterator;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let measurements = read_data("./input.txt");
    let mut i: usize = 0;
    let mut increasement_counter: i16 = 0;
    while i + 4 <= measurements.len() {
        let val_a: i16 = measurements[i..i + 3].iter().sum();
        let val_b: i16 = measurements[i + 1..i + 4].iter().sum();
        if val_b > val_a {
            increasement_counter += 1;
        }
        i += 1;
    }

    println!("There is {} increasements", increasement_counter);

    pub fn read_data(path: &str) -> Vec<i16> {
        let lines: Vec<String> = read_lines(path).unwrap();
        return lines
            .iter()
            .map(|s| s.to_string().parse::<i16>().unwrap())
            .collect();
    }
    pub fn read_lines(path: &str) -> Result<Vec<String>, io::Error> {
        let file = File::open(path)?;
        io::BufReader::new(file).lines().collect()
    }
}
