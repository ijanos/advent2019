use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn read_wire(wire: &Vec<String>) -> HashMap<(isize, isize), (u8, usize)> {
    let mut map = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    let mut steps = 0;
    for i in wire {
        let mut chars = i.chars();
        let d = chars.next().unwrap();
        let n: isize = chars.collect::<String>().parse().unwrap();
        let (w, h) = match d {
            'U' => (0, 1),
            'D' => (0, -1),
            'R' => (1, 0),
            'L' => (-1, 0),
            _ => unimplemented!()
        };
        for _ in 0..n {
            map.insert((x, y), (1, steps));
            x += w;
            y += h;
            steps += 1;
        };
    }
    map
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().map(|line| line.unwrap().split( ',').map(|l| l.to_owned()).collect::<Vec<String>>()).collect::<Vec<_>>();

    let wire1 = read_wire(&input[0]);
    let wire2 = read_wire(&input[1]);

    let part1 = wire2.keys().filter_map(|&(x, y)| {
        match wire1.get(&(x, y)) {
            Some(_) if x != 0 && y != 0 => Some(x.abs() + y.abs()),
            _ => None
        }
    }).min().unwrap();
    println!("Part 1: {}", part1);

    let part2 = wire2.iter().filter_map(|(&(x, y), (_, w1steps))| {
        match wire1.get(&(x, y)) {
            Some((_, w2steps)) if x != 0 && y != 0 => Some(w1steps + w2steps),
            _ => None
        }
    }).min().unwrap();
    println!("Part 2: {}", part2);
}
