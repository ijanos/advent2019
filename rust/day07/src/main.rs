use std::io;
use std::io::prelude::*;

use itertools::Itertools;

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

struct IntCodeVM {
    pc: usize,
    mem: Vec<isize>,
    inputs: Vec<isize>
}

impl IntCodeVM {
    fn new(starting_memory: &[isize], inputbuffer: &[isize]) -> IntCodeVM {
        IntCodeVM {pc: 0, mem: starting_memory.to_vec(), inputs: inputbuffer.to_vec()}
    }
    fn run(&mut self) -> Option<isize> {
        use Instruction::*;
        loop {
            let opcode = opcode_decode(self.pc, &self.mem);
            match opcode {
                Add(a, b, out) => { self.mem[out] = a + b; self.pc += 4; }
                Mul(a, b, out) => { self.mem[out] = a * b; self.pc += 4; }
                Inp(out) => { self.mem[out] = self.inputs.pop().unwrap(); self.pc += 2; }
                Out(a) => { self.pc += 2; return Some(a) }
                Jnz(a, b) => self.pc = if a != 0 { b } else { self.pc + 3 },
                Jz(a, b) => self.pc = if a == 0 { b } else { self.pc + 3 },
                Lt(a, b, out) => { self.mem[out] = if a < b { 1 } else { 0 }; self.pc += 4 }
                Eq(a, b, out) => { self.mem[out] = if a == b { 1 } else { 0 }; self.pc += 4; }
                Hlt => return None
            }
        }
    }
    fn feedInput(&mut self, i: isize) {
        self.inputs.push(i);
    }
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap().split(',').map(|n| n.parse().unwrap()).collect::<Vec<isize>>();

    let mut part1 = 0;
    for p in (0..=4).permutations(5) {
        let mut nextinput = 0;
        for &pn in p.iter() {
            let mut vm = IntCodeVM::new(&input, &mut vec![nextinput, pn]);
            nextinput = vm.run().unwrap();
        }
        if nextinput > part1 {
            part1 = nextinput
        }
    }
    println!("Part 1: {}", part1);
}
