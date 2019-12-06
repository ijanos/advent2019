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

    let mut part1 = 0;
    let mut nodes: Vec<(&str, usize, Vec<&str>)>  = vec![("COM", 0, vec!["COM"])];
    let mut san: HashSet<&str> = HashSet::new();
    let mut you: HashSet<&str> = HashSet::new();

    while !nodes.is_empty() {
        let (current, depth, ref path) = nodes.pop().unwrap();
        part1 += depth;
        if let Some(list) = orbits.get(current) {
            for i in list {
                match i.as_ref() {
                    "YOU" => you = path.iter().copied().collect::<HashSet<_>>(),
                    "SAN" => san = path.iter().copied().collect::<HashSet<_>>(),
                    _ => ()
                };
                let mut newpath = path.clone();
                newpath.push(i);
                nodes.push((i, depth + 1, newpath));
            }
        }
    }
    println!("Part 1: {}", part1);

    let part2 = you.symmetric_difference(&san).count();
    println!("Part 2: {}", part2);
}
