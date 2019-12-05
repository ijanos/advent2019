use std::io;
use std::io::prelude::*;

#[derive(Debug)]
enum Instruction {
    Add(isize, isize, usize),
    Mul(isize, isize, usize),
    Inp(usize),
    Out(isize),
    Hlt,
    Jtr(isize, usize),
    Jfl(isize, usize),
    Lt(isize, isize, usize),
    Eq(isize, isize, usize)
}

fn opcode_decode(pc: usize, memory: &Vec<isize>) -> Instruction {
    match memory[pc] {
        1 => {
            let a = memory[pc + 1] as usize;
            let b = memory[pc + 2] as usize;
            let out = memory[pc + 3] as usize;
            Instruction::Add(memory[a], memory[b], out)
        }
        2 => {
            let a = memory[pc + 1] as usize;
            let b = memory[pc + 2] as usize;
            let out = memory[pc + 3] as usize;
            Instruction::Mul(memory[a], memory[b], out)
        }
        3 => {
            let a = memory[pc + 1] as usize;
            Instruction::Inp(a)
        }
        4 => {
            let a = memory[pc + 1] as usize;
            Instruction::Out(memory[a])
        }
        99 => Instruction::Hlt,
        1102 => {
            let a = memory[pc + 1];
            let b = memory[pc + 2];
            let out = memory[pc + 3] as usize;
            Instruction::Mul(a, b, out)
        }
        104 => {
            let a = memory[pc + 1];
            Instruction::Out(a)
        }
        101 => {
            let a = memory[pc + 1];
            let b = memory[pc + 2] as usize;
            let out = memory[pc + 3] as usize;
            Instruction::Add(a, memory[b], out)
        }
        102 => {
            let a = memory[pc + 1];
            let b = memory[pc + 2] as usize;
            let out = memory[pc + 3] as usize;
            Instruction::Mul(a, memory[b], out)
        }
        1001 => {
            let a = memory[pc + 1] as usize;
            let b = memory[pc + 2];
            let out = memory[pc + 3] as usize;
            Instruction::Add(memory[a], b, out)
        }
        1002 => {
            let a = memory[pc + 1] as usize;
            let b = memory[pc + 2];
            let out = memory[pc + 3] as usize;
            Instruction::Mul(memory[a], b, out)
        }
        1101 => {
            let a = memory[pc + 1];
            let b = memory[pc + 2];
            let out = memory[pc + 3] as usize;
            Instruction::Add(a, b, out)
        }
        5 => {
            let a = memory[pc + 1] as usize;
            let b = memory[pc + 2] as usize;
            Instruction::Jtr(memory[a], memory[b] as usize)
        }
        1105 => {
            let a = memory[pc + 1];
            let b = memory[pc + 2] as usize;
            Instruction::Jtr(a, b)
        }
        1005 => {
            let a = memory[pc + 1] as usize;
            let b = memory[pc + 2] as usize;
            Instruction::Jtr(memory[a], b)
        }
        1106 => {
            let a = memory[pc + 1];
            let b = memory[pc + 2] as usize;
            Instruction::Jfl(a, b)
        }
        1006 => {
            let a = memory[pc + 1] as usize;
            let b = memory[pc + 2] as usize;
            Instruction::Jfl(memory[a], b)
        }
        105 => {
            let a = memory[pc + 1];
            let b = memory[pc + 2] as usize;
            Instruction::Jtr(a, memory[b] as usize)
        }
        106 => {
            let a = memory[pc + 1];
            let b = memory[pc + 2] as usize;
            Instruction::Jfl(a, memory[b] as usize)
        }
        7 => {
            let a = memory[pc + 1] as usize;
            let b = memory[pc + 2] as usize;
            let out = memory[pc + 3] as usize;
            Instruction::Lt(memory[a], memory[b], out)
        }
        108 => {
            let a = memory[pc + 1];
            let b = memory[pc + 2] as usize;
            let out = memory[pc + 3] as usize;
            Instruction::Eq(a, memory[b], out)
        }
        1108 => {
            let a = memory[pc + 1];
            let b = memory[pc + 2];
            let out = memory[pc + 3] as usize;
            Instruction::Eq(a, b, out)
        }
        107 => {
            let a = memory[pc + 1];
            let b = memory[pc + 2] as usize;
            let out = memory[pc + 3] as usize;
            Instruction::Lt(a, memory[b], out)
        }
        1107 => {
            let a = memory[pc + 1];
            let b = memory[pc + 2];
            let out = memory[pc + 3] as usize;
            Instruction::Lt(a, b, out)
        }
        8 => {
            let a = memory[pc + 1] as usize;
            let b = memory[pc + 2] as usize;
            let out = memory[pc + 3] as usize;
            Instruction::Eq(memory[a], memory[b], out)
        }
        1007 => {
            let a = memory[pc + 1] as usize;
            let b = memory[pc + 2];
            let out = memory[pc + 3] as usize;
            Instruction::Lt(memory[a], b, out)
        }
        1008 => {
            let a = memory[pc + 1] as usize;
            let b = memory[pc + 2];
            let out = memory[pc + 3] as usize;
            Instruction::Eq(memory[a], b, out)
        }
        code => panic!("unknown opcode {}", code)
    }
}

fn calc(starting_memory: &[isize]) {
    let mut pc = 0;
    let mut memory: Vec<isize> = starting_memory.to_vec();
    loop {
        use Instruction::*;
        let opcode = opcode_decode(pc, &memory);
        match opcode {
            Add(a, b, out) => {
                memory[out] = a + b;
                pc += 4;
            }
            Mul(a, b, out) => {
                memory[out] = a * b;
                pc += 4;
            }
            Inp(out) => {
                memory[out] = 5; // part 2
                pc += 2;
            }
            Out(a) => {
                println!("OUT: {}", a);
                pc += 2;
            }
            Jtr(a, b) => {
                if a != 0 {
                    pc = b;
                } else {
                    pc += 3;
                }
            }
            Jfl(a, b) => {
                if a == 0 {
                    pc = b
                } else {
                    pc += 3;
                }
            }
            Lt(a, b, out) => {
                if a < b {
                    memory[out] = 1
                } else {
                    memory[out] = 0
                }
                pc += 4;
            }
            Eq(a, b, out) => {
                if a == b {
                    memory[out] = 1
                } else {
                    memory[out] = 0
                }
                pc += 4;
            }
            Hlt => {
                break;
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap().split(',').map(|n| n.parse().unwrap()).collect::<Vec<isize>>();

    calc(&input);

}