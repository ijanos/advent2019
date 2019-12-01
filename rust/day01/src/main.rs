use std::io;
use std::io::prelude::*;
use std::iter::successors;

fn main() {
    let stdin = io::stdin();
    let modules = stdin.lock().lines().map(|line| line.unwrap().parse().unwrap()).collect::<Vec<usize>>();

    let part1: usize = modules.iter().map(|m| m / 3 - 2 ).sum();
    println!("Part 1: {}", part1);

    let fuelalltheway = |mass: &usize| successors(Some(*mass), |m| (m / 3).checked_sub(2)).skip(1).sum::<usize>();
    let part2: usize = modules.iter().map(fuelalltheway).sum();
    println!("Part 2: {}", part2);
}
