use std::io;
use std::io::prelude::*;

fn allfuel(mass: isize) -> isize {
    let mut all = 0;
    let mut mass = mass;
    loop {
        let additionalfuel = mass / 3 - 2;
        if additionalfuel > 0 {
            all += additionalfuel
        } else {
            break;
        }
        mass = additionalfuel;
    }
    all
}

fn main() {
    let stdin = io::stdin();
    let modules = stdin.lock().lines().map(|line| line.unwrap().parse().unwrap()).collect::<Vec<_>>();

    let part1: isize = modules.iter().map(|m| m / 3 - 2 ).sum();
    println!("Part 1: {}", part1);

    let part2: isize = modules.iter().map(|&m| allfuel(m)).sum();
    println!("Part 2: {}", part2);
}
