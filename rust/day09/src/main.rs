use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;
use std::iter::FromIterator;

type int = i64;

#[derive(Debug)]
enum Instruction {
    Add(int, int, usize),
    Eq(int, int, usize),
    Hlt,
    Inp(usize),
    Jnz(int, usize),
    Jz(int, usize),
    Lt(int, int, usize),
    Mul(int, int, usize),
    Out(int),
    Rbo(int),
}

#[derive(Clone)]
struct IntCodeVM {
    pc: usize,
    mem: Vec<int>,
    inputs: VecDeque<int>,
    relative_base: int
}

impl IntCodeVM {
    fn new(starting_memory: &[int], inputbuffer: &[int]) -> IntCodeVM {
        let mut mem = starting_memory.to_vec();
        mem.resize(mem.len() + 4096, 0); // YOLO
        IntCodeVM {
            pc: 0,
            mem,
            inputs: VecDeque::from_iter(inputbuffer.iter().copied()),
            relative_base: 0
        }
    }

    fn next_output(&mut self) -> Option<int> {
        use Instruction::*;
        loop {
            match self.current_opcode_decode() {
                Add(a, b, out) => { self.mem[out] = a + b; self.pc += 4; }
                Mul(a, b, out) => { self.mem[out] = a * b; self.pc += 4; }
                Inp(out) => { self.mem[out] = self.inputs.pop_back().unwrap(); self.pc += 2; }
                Out(a) => { self.pc += 2; return Some(a) }
                Jnz(a, b) => self.pc = if a != 0 { b } else { self.pc + 3 },
                Jz(a, b) => self.pc = if a == 0 { b } else { self.pc + 3 },
                Lt(a, b, out) => { self.mem[out] = if a < b { 1 } else { 0 }; self.pc += 4 }
                Eq(a, b, out) => { self.mem[out] = if a == b { 1 } else { 0 }; self.pc += 4; }
                Rbo(a) => { self.relative_base += a; self.pc += 2 }
                Hlt => return None
            }
        }
    }

    fn collect_output(&mut self) -> Vec<int> {
        let mut out = Vec::new();
        while let Some(n) = self.next_output() {
            out.push(n)
        }
        out
    }

    fn feed_input(&mut self, i: int) {
        self.inputs.push_front(i);
    }

    #[inline(always)]
    fn current_opcode_decode(&self) -> Instruction {
        let param = |n: usize| {
            let p = self.mem[self.pc + n];
            match self.mem[self.pc] / (10 * 10_u32.pow(n as u32) as int) % 10 {
                1 => p,
                2 => self.mem[(self.relative_base + p) as usize],
                _ => self.mem[p as usize]
            }
        };

        let dest = |n: usize| {
            let p = self.mem[self.pc + n];
            match self.mem[self.pc] / (10 * 10_u32.pow(n as u32) as int) % 10 {
                1 => panic!("immidiate output paramter"),
                2 => (self.relative_base + p) as usize,
                _ => p as usize
            }
        };

        use Instruction::*;
        match self.mem[self.pc] % 100 {
            1 => Add(param(1), param(2), dest(3)),
            2 => Mul(param(1), param(2), dest(3)),
            3 => Inp(dest(1)),
            4 => Out(param(1)),
            5 => Jnz(param(1), param(2) as usize),
            6 => Jz(param(1), param(2) as usize),
            7 => Lt(param(1), param(2), dest(3)),
            8 => Eq(param(1), param(2), dest(3)),
            9 => Rbo(param(1)),
            99 => Hlt,
            code => panic!("unknown opcode {}", code)
        }
    }
}

fn part1(starting_memory: &[int]) -> int {
    let mut vm = IntCodeVM::new(&starting_memory, &[1]);
    *vm.collect_output().last().unwrap()
}

fn part2(starting_memory: &[int]) -> int {
    let mut vm = IntCodeVM::new(&starting_memory, &[2]);
    *vm.collect_output().last().unwrap()
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap().split(',').map(|n| n.parse().unwrap()).collect::<Vec<int>>();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_example1() {
        let testinput = "104,1125899906842624,99";
        let testinput = testinput.split(',').map(|n| n.parse().unwrap()).collect::<Vec<int>>();
        let mut vm = IntCodeVM::new(&testinput, &[]);
        assert_eq!(vm.next_output(), Some(1125899906842624));
    }
    #[test]
    fn test_p1_example2() {
        let testinput = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let testinput = testinput.split(',').map(|n| n.parse().unwrap()).collect::<Vec<int>>();
        let mut vm = IntCodeVM::new(&testinput, &[]);
        assert_eq!(vm.collect_output(), testinput);
    }
}