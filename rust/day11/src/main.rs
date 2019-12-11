use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

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
    input: int,
    relative_base: int
}

impl IntCodeVM {
    fn new(starting_memory: &[int], input: int) -> IntCodeVM {
        let mut mem = starting_memory.to_vec();
        mem.resize(mem.len() + 4096, 0); // YOLO
        IntCodeVM {
            pc: 0,
            mem,
            input,
            relative_base: 0
        }
    }

    fn next_output(&mut self) -> Option<int> {
        use Instruction::*;
        loop {
            match self.current_opcode_decode() {
                Add(a, b, out) => { self.mem[out] = a + b; self.pc += 4; }
                Mul(a, b, out) => { self.mem[out] = a * b; self.pc += 4; }
                Inp(out) => { self.mem[out] = self.input; self.pc += 2; }
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

    fn set_input(&mut self, i: int) {
        self.input = i;
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

#[derive(Debug)]
struct Robot {
    d: i8,
    x: i64,
    y: i64,
}

const UP: i8 = 0;
const RIGHT: i8 = 1;
const DOWN: i8 = 2;
const LEFT: i8 = 3;

fn modulo(a: i8, b: i8) -> i8 {
    ((a % b) + b) % b
}

impl Robot {
    fn new() -> Robot {
        Robot { d: UP, x: 0, y: 0, }
    }

    fn turn_and_move(&mut self, n: u8) {
        match n {
            0 => self.d  = modulo(self.d - 1, 4),
            1 => self.d  = modulo(self.d + 1, 4),
            e => panic!("wrong turn: {}", e)
        }
        match self.d {
            UP => self.y -= 1,
            RIGHT => self.x += 1,
            DOWN => self.y += 1,
            LEFT => self.x -= 1,
            e => panic!("wrong direction: {}", e)
        }
    }
}


fn paint(starting_memory: &[int], inp: int) -> HashMap<(i64, i64), u8> {
    let mut robot = Robot::new();
    let mut vm = IntCodeVM::new(&starting_memory, inp);
    let mut map: HashMap<(i64, i64), u8> = HashMap::new();
    map.insert((0,0), inp as u8);
    loop {
        if let Some(color) = vm.next_output() {
            assert!(color < 2);
            map.insert((robot.x, robot.y), color as u8);
        } else {
            break;
        }
        if let Some(turn) = vm.next_output() {
            robot.turn_and_move(turn as u8);
            vm.set_input(*map.get(&(robot.x, robot.y)).unwrap_or(&0) as int);
        } else {
            break;
        }
    }
    map
}

fn pretty_print(map: &HashMap<(i64, i64), u8>) {
    for y in -5..10 {
        for x in -10..80 {
            let c = match map.get(&(x, y)) {
                Some(1) => '█',
                _ => ' ',
            };
            print!("{}", c);
        }
        println!();
    }
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap().split(',').map(|n| n.parse().unwrap()).collect::<Vec<int>>();

    let part1 = paint(&input, 0).len();
    println!("Part 1: {}", part1);

    pretty_print(&paint(&input, 1));
}

