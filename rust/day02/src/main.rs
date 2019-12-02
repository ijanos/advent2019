use std::io;
use std::io::prelude::*;

fn calc(a: usize, b: usize, mut memory: Vec<usize>) -> usize {
    let mut pc = 0;
    memory[1] = a;
    memory[2] = b;
    loop {
        match memory[pc] {
            1 => {
                let a = memory[pc + 1];
                let b = memory[pc + 2];
                let out = memory[pc + 3];
                memory[out] = memory[a] + memory[b];
                pc += 4;
            }
            2 => {
                let a = memory[pc + 1];
                let b = memory[pc + 2];
                let out = memory[pc + 3];
                memory[out] = memory[a] * memory[b];
                pc += 4;
            }
            99 => {
                break;
            }
            _ => panic!("unknown opcode")
        }
    }
    memory[0]
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap().split(',').map(|n| n.parse().unwrap()).collect::<Vec<usize>>();

    println!("Part 1: {}", calc(12, 2, input.to_vec()));

    'mainloop: for a in 0..=99 {
        for b in 0..=99 {
            if calc(a,b, input.to_vec()) == 19690720 {
                println!("Part 2: {}", 100 * a + b);
                break 'mainloop;
            }
        }
    }
}
