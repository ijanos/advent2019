use std::io;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().map(|line| line.unwrap()).collect::<Vec<String>>();

    let mut orbits: HashMap<String, Vec<String>> = HashMap::new();
    for line in input {
        let line: Vec<_> = line.split(')').collect();
        let a = line[0].to_string();
        let b = line[1].to_string();
        orbits.entry(a).or_insert_with(Vec::new).push(b);
    }

    let keys = orbits.keys().collect::<HashSet<_>>();
    let values = orbits.values().flatten().collect::<HashSet<_>>();
    let root = keys.difference(&values).next().unwrap();

    let mut part1 = 0;
    let mut nodes: Vec<(&str, usize, Vec<&str>)>  = vec![(root, 0, vec![root])];
    let mut san: HashSet<String> = HashSet::new();
    let mut you: HashSet<String> = HashSet::new();

    while !nodes.is_empty() {
        let (current, depth, path) = nodes.pop().unwrap();
        part1 += depth;
        if let Some(list) = orbits.get(current) {
            for i in list {
                match i.as_ref() {
                    "YOU" => you = path.iter().map(|s| s.to_string()).collect(),
                    "SAN" => san = path.iter().map(|s| s.to_string()).collect(),
                    _ => ()
                };
                let mut newpath = path.clone();
                newpath.push(i);
                nodes.push((i, depth + 1, newpath));
            }
        }
    }
    println!("Part 1: {}", part1);

    let diff = you.symmetric_difference(&san).count();
    println!("Part 2: {}", diff);
}
