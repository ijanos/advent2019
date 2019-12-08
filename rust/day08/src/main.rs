use std::io;
use std::io::prelude::*;


const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const LAYER_SIZE: usize = WIDTH * HEIGHT;

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap().chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>();
    let layers = input.chunks(LAYER_SIZE).collect::<Vec<_>>();
    let (ones, twos) = layers.iter().min_by_key(|layer| layer.iter().filter(|&&p| p == 0).count()).unwrap().iter().fold((0, 0), |(ones, twos), p| {
        match p {
            1 => (ones + 1, twos),
            2 => (ones, twos + 1),
            _ => (ones, twos)
        }
    });
    println!("Part 1: {}", ones * twos);

    let screen = (0..LAYER_SIZE).map(|i| {
        layers.iter().map(|l| l[i]).find(|&px| px == 0 ||  px == 1).unwrap()
    }).collect::<Vec<_>>();

    println!("Part 2:");
    screen.chunks(WIDTH).for_each(|line| println!("{}", line.iter().map(|&p| if p == 0 {' '} else {'█'}).collect::<String>()));
}
