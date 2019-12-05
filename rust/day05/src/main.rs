use std::io;
use std::io::prelude::*;

#[derive(Debug)]
enum Instruction {
    Add(isize, isize, usize),
    Eq(isize, isize, usize),
    Hlt,
    Inp(usize),
    Jnz(isize, usize),
    Jz(isize, usize),
    Lt(isize, isize, usize),
    Mul(isize, isize, usize),
    Out(isize)
}

fn opcode_decode(pc: usize, memory: &Vec<isize>) -> Instruction {
    let resolve_a = |a| if memory[pc] / 100 % 10 == 1 { a } else { memory[a as usize] };
    let resolve_b = |b| if memory[pc] / 1000 % 10 == 1 { b } else { memory[b as usize] };
    match memory[pc] % 100 {
        1 => {
            let a = memory[pc + 1];
            let b = memory[pc + 2];
            let out = memory[pc + 3] as usize;
            Instruction::Add(resolve_a(a), resolve_b(b), out)
        }
        2 => {
            let a = memory[pc + 1];
            let b = memory[pc + 2];
            let out = memory[pc + 3] as usize;
            Instruction::Mul(resolve_a(a), resolve_b(b), out)
        }
        3 => {
            let a = memory[pc + 1] as usize;
            Instruction::Inp(a)
        }
        4 => {
            let a = memory[pc + 1];
            Instruction::Out(resolve_a(a))
        }
        5 => {
            let a = memory[pc + 1];
            let b = memory[pc + 2];
            Instruction::Jnz(resolve_a(a), resolve_b(b) as usize)
        }
        6 => {
            let a = memory[pc + 1];
            let b = memory[pc + 2];
            Instruction::Jz(resolve_a(a), resolve_b(b) as usize)
        }
        7 => {
            let a = memory[pc + 1];
            let b = memory[pc + 2];
            let out = memory[pc + 3] as usize;
            Instruction::Lt(resolve_a(a), resolve_b(b), out)
        }
        8 => {
            let a = memory[pc + 1];
            let b = memory[pc + 2];
            let out = memory[pc + 3] as usize;
            Instruction::Eq(resolve_a(a), resolve_b(b), out)
        }
        99 => Instruction::Hlt,
        code => panic!("unknown opcode {}", code)
    }
}


fn run(starting_memory: &[isize], fixinput: isize) -> Vec<isize> {
    let mut pc = 0;
    let mut output = Vec::new();
    let mut memory: Vec<isize> = starting_memory.to_vec();
    loop {
        use Instruction::*;
        let opcode = opcode_decode(pc, &memory);
        match opcode {
            Add(a, b, out) => { memory[out] = a + b; pc += 4; }
            Mul(a, b, out) => { memory[out] = a * b; pc += 4; }
            Inp(out) => { memory[out] = fixinput; pc += 2; }
            Out(a) => { output.push(a); pc += 2; }
            Jnz(a, b) => pc = if a != 0 { b } else { pc + 3 },
            Jz(a, b) => pc = if a == 0 { b } else { pc + 3 },
            Lt(a, b, out) => { memory[out] = if a < b { 1 } else { 0 }; pc += 4 }
            Eq(a, b, out) => { memory[out] = if a == b { 1 } else { 0 }; pc += 4; }
            Hlt => break
        }
    }
    output
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap().split(',').map(|n| n.parse().unwrap()).collect::<Vec<isize>>();

    let part1 = run(&input, 1);
    println!("Part 1: {}", part1.last().unwrap());

    let part2 = run(&input, 5);
    println!("Part 2: {}", part2.last().unwrap());
}
