use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;
use std::iter::FromIterator;
use std::cmp::max;

use itertools::Itertools;

#[derive(Debug)]
enum Instruction {
    Add(i128, i128, usize),
    Eq(i128, i128, usize),
    Hlt,
    Inp(usize),
    Jnz(i128, usize),
    Jz(i128, usize),
    Lt(i128, i128, usize),
    Mul(i128, i128, usize),
    Out(i128)
}

fn opcode_decode(pc: usize, memory: &[i128]) -> Instruction {
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

#[derive(Clone)]
struct IntCodeVM {
    pc: usize,
    mem: Vec<i128>,
    inputs: VecDeque<i128>
}

impl IntCodeVM {
    fn new(starting_memory: &[i128], inputbuffer: &[i128]) -> IntCodeVM {
        IntCodeVM {pc: 0, mem: starting_memory.to_vec(), inputs: VecDeque::from_iter(inputbuffer.iter().copied())}
    }
    fn run(&mut self) -> Option<i128> {
        use Instruction::*;
        loop {
            let opcode = opcode_decode(self.pc, &self.mem);
            match opcode {
                Add(a, b, out) => { self.mem[out] = a + b; self.pc += 4; }
                Mul(a, b, out) => { self.mem[out] = a * b; self.pc += 4; }
                Inp(out) => { self.mem[out] = self.inputs.pop_back().unwrap(); self.pc += 2; }
                Out(a) => { self.pc += 2; return Some(a) }
                Jnz(a, b) => self.pc = if a != 0 { b } else { self.pc + 3 },
                Jz(a, b) => self.pc = if a == 0 { b } else { self.pc + 3 },
                Lt(a, b, out) => { self.mem[out] = if a < b { 1 } else { 0 }; self.pc += 4 }
                Eq(a, b, out) => { self.mem[out] = if a == b { 1 } else { 0 }; self.pc += 4; }
                Hlt => return None
            }
        }
    }
    fn feed_input(&mut self, i: i128) {
        self.inputs.push_front(i);
    }
}

fn part1(starting_memory: &[i128]) -> i128 {
    let mut part1 = 0;
    for p in (0..=4).permutations(5) {
        let mut nextinput = 0;
        for &pn in p.iter() {
            let mut vm = IntCodeVM::new(&starting_memory, &[nextinput, pn]);
            nextinput = vm.run().unwrap();
        }
        part1 = max(part1, nextinput);
    }
    part1
}

fn part2(starting_memory: &[i128]) -> i128 {
    let mut out = 0;
    for p in (5..=9).permutations(5) {
        let mut vms = p.iter().map(|&n| IntCodeVM::new(&starting_memory, &[n])).collect::<Vec<_>>();
        let mut signal = 0;
        for i in (0..=4).cycle() {
            vms[i].feed_input(signal);
            match vms[i].run() {
                Some(output) => signal = output,
                None if i == 4 => break,
                None => ()
            }
        }
        out = max(out, signal);
    }
    out
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap().split(',').map(|n| n.parse().unwrap()).collect::<Vec<i128>>();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_example1() {
        let testinput = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let testinput = testinput.split(',').map(|n| n.parse().unwrap()).collect::<Vec<i128>>();
        assert_eq!(part1(&testinput), 65210);
    }

    #[test]
    fn test_p1_example2() {
        let testinput = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        let testinput = testinput.split(',').map(|n| n.parse().unwrap()).collect::<Vec<i128>>();
        assert_eq!(part1(&testinput), 54321);
    }

    #[test]
    fn test_p2_example1() {
        let testinput = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        let testinput = testinput.split(',').map(|n| n.parse().unwrap()).collect::<Vec<i128>>();
        assert_eq!(part2(&testinput), 139629729);
    }
    #[test]
    fn test_p2_example2() {
        let testinput = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
        let testinput = testinput.split(',').map(|n| n.parse().unwrap()).collect::<Vec<i128>>();
        assert_eq!(part2(&testinput), 18216);
    }
}