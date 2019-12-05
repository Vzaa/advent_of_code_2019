use std::convert::{TryInto, TryFrom};

#[derive(Debug)]
enum Mode {
    Pos,
    Im,
}

#[derive(Debug)]
enum Intcode {
    Add(Mode, Mode, Mode),
    Mult(Mode, Mode, Mode),
    In(Mode),
    Out(Mode),
    Jit(Mode, Mode),
    Jif(Mode, Mode),
    Lt(Mode, Mode, Mode),
    Equ(Mode, Mode, Mode),
    Halt,
}

#[derive(Debug)]
enum ErrorIntcode {
    InvalidOpcode,
    InvalidMode,
}

impl TryFrom<i32> for Mode {
    type Error = ErrorIntcode;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Mode::Pos),
            1 => Ok(Mode::Im),
            _ => Err(ErrorIntcode::InvalidMode),
        }
    }
}

impl TryFrom<i32> for Intcode {
    type Error = ErrorIntcode;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let m1: Mode = ((value / 100) % 10).try_into()?;
        let m2: Mode = ((value / 1000) % 10).try_into()?;
        let m3: Mode = ((value / 10000) % 10).try_into()?;

        match value % 100 {
            1 => Ok(Intcode::Add(m1, m2, m3)),
            2 => Ok(Intcode::Mult(m1, m2, m3)),
            3 => Ok(Intcode::In(m1)),
            4 => Ok(Intcode::Out(m1)),
            5 => Ok(Intcode::Jit(m1, m2)),
            6 => Ok(Intcode::Jif(m1, m2)),
            7 => Ok(Intcode::Lt(m1, m2, m3)),
            8 => Ok(Intcode::Equ(m1, m2, m3)),
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

fn mem_get(mem: &[i32], addr: usize, m: Mode) -> i32 {
    match m {
        Mode::Pos => mem[mem[addr] as usize],
        Mode::Im => mem[addr],
    }
}

fn mem_set(mem: &mut [i32], addr: usize, m: Mode, v: i32) {
    match m {
        Mode::Pos => mem[mem[addr] as usize] = v,
        Mode::Im => mem[addr] = v,
    }
}

// We have a single input anyway, make it a fn argument
fn run(input: i32) -> i32 {
    let mut mem = read_mem();
    let mut pc = 0;
    loop {
        let op: Intcode = mem[pc].try_into().unwrap();
        match op {
            Intcode::Add(m1, m2, m3) => {
                let (p_a, p_b) = (
                    mem_get(&mem, pc + 1, m1),
                    mem_get(&mem, pc + 2, m2),
                );
                mem_set(&mut mem, pc + 3, m3, p_a + p_b);
                pc += 4;
            }
            Intcode::Mult(m1, m2, m3) => {
                let (p_a, p_b) = (
                    mem_get(&mem, pc + 1, m1),
                    mem_get(&mem, pc + 2, m2),
                );
                mem_set(&mut mem, pc + 3, m3, p_a * p_b);
                pc += 4;
            }
            Intcode::In(m1) => {
                mem_set(&mut mem, pc + 1, m1, input);
                pc += 2;
            }
            Intcode::Out(m1) => {
                let p_a = mem_get(&mem, pc + 1, m1);
                println!("Out: {}", p_a);
                pc += 2;
            }
            Intcode::Jit(m1, m2) => {
                let (p_a, p_b) = (
                    mem_get(&mem, pc + 1, m1),
                    mem_get(&mem, pc + 2, m2),
                );
                if p_a != 0 {
                    pc = p_b as usize;
                } else {
                    pc += 3;
                }
            }
            Intcode::Jif(m1, m2) => {
                let (p_a, p_b) = (
                    mem_get(&mem, pc + 1, m1),
                    mem_get(&mem, pc + 2, m2),
                );
                if p_a == 0 {
                    pc = p_b as usize;
                } else {
                    pc += 3;
                }
            }
            Intcode::Lt(m1, m2, m3) => {
                let (p_a, p_b) = (
                    mem_get(&mem, pc + 1, m1),
                    mem_get(&mem, pc + 2, m2),
                );
                mem_set(&mut mem, pc + 3, m3, (p_a < p_b) as i32);
                pc += 4;
            }
            Intcode::Equ(m1, m2, m3) => {
                let (p_a, p_b) = (
                    mem_get(&mem, pc + 1, m1),
                    mem_get(&mem, pc + 2, m2),
                );
                mem_set(&mut mem, pc + 3, m3, (p_a == p_b) as i32);
                pc += 4;
            }
            Intcode::Halt => {
                break;
            }
        }
    }
    mem[0]
}

fn main() {
    println!("Part 1:");
    run(1);
    println!("\nPart 2:");
    run(5);
}
