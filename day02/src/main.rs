use std::convert::TryFrom;

#[derive(Debug)]
enum Intcode {
    Add,
    Mult,
    Halt,
}

#[derive(Debug)]
enum ErrorIntcode {
    InvalidOpcode,
}

impl TryFrom<i32> for Intcode {
    type Error = ErrorIntcode;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Intcode::Add),
            2 => Ok(Intcode::Mult),
            99 => Ok(Intcode::Halt),
            _ => Err(ErrorIntcode::InvalidOpcode),
        }
    }
}

fn read_mem() -> Vec<i32> {
    let text = std::fs::read_to_string("input").unwrap();
    let text = text.trim();
    text.split(',').map(|s| s.parse().unwrap()).collect()
}

fn run(noun: i32, verb: i32) -> i32 {
    let mut mem = read_mem();
    let mut pc = 0;
    mem[1] = noun;
    mem[2] = verb;
    loop {
        let op: Intcode = Intcode::try_from(mem[pc]).unwrap();
        match op {
            Intcode::Add => {
                let (p_a, p_b, p_c) = (
                    mem[pc + 1] as usize,
                    mem[pc + 2] as usize,
                    mem[pc + 3] as usize,
                );
                mem[p_c] = mem[p_a] + mem[p_b];
                pc += 4;
            }
            Intcode::Mult => {
                let (p_a, p_b, p_c) = (
                    mem[pc + 1] as usize,
                    mem[pc + 2] as usize,
                    mem[pc + 3] as usize,
                );
                mem[p_c] = mem[p_a] * mem[p_b];
                pc += 4;
            }
            Intcode::Halt => {
                break;
            }
        }
    }
    mem[0]
}

fn p1() {
    println!("Part 1: {}", run(12, 2));
}

fn p2() {
    let tgt = 19690720;
    let mut noun = 0;
    let mut verb = 0;

    loop {
        if run(noun, verb) == tgt {
            println!("Part 2: {}", 100 * noun + verb);
            break;
        } else if run(noun + 1, verb) < tgt {
            noun += 1;
        } else {
            verb += 1;
        }
    }
}

fn main() {
    p1();
    p2();
}
