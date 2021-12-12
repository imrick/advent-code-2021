use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

static START_ID: &str = "start";
static END_ID: &str = "end";

#[derive(Debug, Clone)]
pub struct Cave {
    id: String,
    links: Vec<String>,
}

fn main() {
    let caves = read_data("./input-full.txt");
    let mut paths: Vec<Vec<String>> = Vec::new();
    visit_cave(&caves, START_ID, &vec![String::from(START_ID)], &mut paths);

    // Remove unvalid paths (better solution ?)
    paths = paths
        .into_iter()
        .filter(|p| p[0] == START_ID && p[p.len() - 1] == END_ID)
        .collect::<Vec<Vec<String>>>();

    // debug paths
    // for path in paths.clone() {
    //     println!("{:?}", path);
    // }
    println!("Nb path {}", paths.clone().len());
}

pub fn visit_cave(
    caves: &HashMap<String, Cave>,
    cave_id: &str,
    current_path: &Vec<String>,
    visited_paths: &mut Vec<Vec<String>>,
) {
    if cave_id != END_ID {
        let cave = caves.get(cave_id).unwrap();
        for link in cave.links.clone() {
            if !is_lower_case(&link)
                || current_path
                    .iter()
                    .find(|c| c.to_string() == link)
                    .is_none()
            {
                let mut new_path: Vec<String> = current_path.clone();
                new_path.push(link.clone());
                visit_cave(caves, &link, &new_path, visited_paths);
                visited_paths.push(new_path);
            }
        }
    }
}

pub fn read_data(path: &str) -> HashMap<String, Cave> {
    let mut caves: HashMap<String, Cave> = HashMap::new();
    for relation in read_lines(path).unwrap() {
        let ids = relation
            .split("-")
            .map(|id| id.to_string())
            .collect::<Vec<String>>();
        register_caves_relation(&ids[0], ids[1].clone(), &mut caves);
        register_caves_relation(&ids[1], ids[0].clone(), &mut caves);
    }
    caves
}

pub fn read_lines(path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;
    io::BufReader::new(file).lines().collect()
}

pub fn register_caves_relation(id: &str, relation: String, caves: &mut HashMap<String, Cave>) {
    if caves.contains_key(id) {
        caves
            .entry(id.to_string())
            .and_modify(|c| c.links.push(relation));
    } else {
        caves.insert(
            id.to_string(),
            Cave {
                id: id.to_string(),
                links: vec![relation],
            },
        );
    }
}

pub fn is_lower_case(test: &str) -> bool {
    let reg = Regex::new(r"[a-z]+").unwrap();
    reg.is_match(test)
}
