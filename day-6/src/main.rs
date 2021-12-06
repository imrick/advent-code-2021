use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    println!(
        "Result part 1 {}",
        running_grow(read_data("./input-full.txt"), 80)
    );
    println!(
        "Result part 2 {}",
        running_grow_v2(read_data("./input-full.txt"), 256)
    );
}

pub fn read_data(path: &str) -> Vec<u16> {
    let lines: Vec<String> = read_lines(path).unwrap();
    return lines[0]
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.to_string().parse::<u16>().unwrap())
        .collect();
}

pub fn read_lines(path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;
    io::BufReader::new(file).lines().collect()
}

pub fn running_grow(mut lanternfishes: Vec<u16>, days: u16) -> usize {
    for _n in 0..days {
        let mut grown_lanternfishes = Vec::new();
        for lanternfishe in lanternfishes {
            if lanternfishe == 0 {
                grown_lanternfishes.push(6);
                grown_lanternfishes.push(8);
            } else {
                grown_lanternfishes.push(lanternfishe - 1);
            }
        }
        lanternfishes = grown_lanternfishes;
    }
    lanternfishes.len()
}

pub fn running_grow_v2(lanternfishes: Vec<u16>, days: usize) -> usize {
    let mut timers_count = HashMap::new();

    (0..9).for_each(|timer| {
        timers_count.insert(timer, 0);
    });

    (0..lanternfishes.len()).for_each(|i| {
        *timers_count.get_mut(&lanternfishes[i]).unwrap() += 1;
    });

    (0..days).for_each(|_day| {
        let new_born_count = timers_count[&0];
        (0..9).for_each(|timer| {
            let new_timer_count;
            if timer < 6 || timer == 7 {
                new_timer_count = timers_count[&(timer + 1)];
            } else if timer == 6 {
                new_timer_count = timers_count[&(timer + 1)] + new_born_count
            } else {
                new_timer_count = new_born_count;
            }
            *timers_count.get_mut(&timer).unwrap() = new_timer_count;
        })
    });

    timers_count.iter().map(|n| n.1).sum::<usize>()
}
