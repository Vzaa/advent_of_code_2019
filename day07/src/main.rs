use std::convert::{TryFrom, TryInto};

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

#[derive(Debug, Clone)]
struct Amp {
    mem: Vec<i32>,
    pc: usize,
}

impl Amp {
    fn new() -> Amp {
        Amp {
            mem: read_mem(),
            pc: 0,
        }
    }

    fn run(&mut self, mut input: Option<i32>, signal: i32) -> Option<i32> {
        loop {
            let op: Intcode = self.mem[self.pc].try_into().unwrap();
            match op {
                Intcode::Add(m1, m2, m3) => {
                    let (p_a, p_b) = (
                        mem_get(&self.mem, self.pc + 1, m1),
                        mem_get(&self.mem, self.pc + 2, m2),
                    );
                    mem_set(&mut self.mem, self.pc + 3, m3, p_a + p_b);
                    self.pc += 4;
                }
                Intcode::Mult(m1, m2, m3) => {
                    let (p_a, p_b) = (
                        mem_get(&self.mem, self.pc + 1, m1),
                        mem_get(&self.mem, self.pc + 2, m2),
                    );
                    mem_set(&mut self.mem, self.pc + 3, m3, p_a * p_b);
                    self.pc += 4;
                }
                Intcode::In(m1) => {
                    if let Some(i) = input.take() {
                        mem_set(&mut self.mem, self.pc + 1, m1, i);
                    } else {
                        mem_set(&mut self.mem, self.pc + 1, m1, signal);
                    }
                    self.pc += 2;
                }
                Intcode::Out(m1) => {
                    let p_a = mem_get(&self.mem, self.pc + 1, m1);
                    self.pc += 2;
                    return Some(p_a);
                }
                Intcode::Jit(m1, m2) => {
                    let (p_a, p_b) = (
                        mem_get(&self.mem, self.pc + 1, m1),
                        mem_get(&self.mem, self.pc + 2, m2),
                    );
                    if p_a != 0 {
                        self.pc = p_b as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                Intcode::Jif(m1, m2) => {
                    let (p_a, p_b) = (
                        mem_get(&self.mem, self.pc + 1, m1),
                        mem_get(&self.mem, self.pc + 2, m2),
                    );
                    if p_a == 0 {
                        self.pc = p_b as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                Intcode::Lt(m1, m2, m3) => {
                    let (p_a, p_b) = (
                        mem_get(&self.mem, self.pc + 1, m1),
                        mem_get(&self.mem, self.pc + 2, m2),
                    );
                    mem_set(&mut self.mem, self.pc + 3, m3, (p_a < p_b) as i32);
                    self.pc += 4;
                }
                Intcode::Equ(m1, m2, m3) => {
                    let (p_a, p_b) = (
                        mem_get(&self.mem, self.pc + 1, m1),
                        mem_get(&self.mem, self.pc + 2, m2),
                    );
                    mem_set(&mut self.mem, self.pc + 3, m3, (p_a == p_b) as i32);
                    self.pc += 4;
                }
                Intcode::Halt => {
                    break;
                }
            }
        }
        None
    }
}

// Get full list of permutations of given slice
fn get_perms(vals: &[i32]) -> Vec<Vec<i32>> {
    if vals.len() == 1 {
        return vec![vals.to_vec()];
    }

    let mut perms = vec![];

    for idx in 0..vals.len() {
        let mut to_ch = vals.to_vec();
        let own = to_ch.remove(idx);

        let children = get_perms(&to_ch);
        for mut c in children {
            c.push(own);
            perms.push(c);
        }
    }

    perms
}

// Part 1
fn run_amps(input: &[i32]) -> i32 {
    let mut signal = 0;
    for v in input {
        let mut amp = Amp::new();
        signal = amp.run(Some(*v), signal).unwrap();
    }
    signal
}

// Part 2
fn run_amps2(input: &[i32]) -> i32 {
    let mut amps = vec![Amp::new(); 5];
    let mut signal = 0;
    // Feed phases once and then return None
    let mut in_iter = input.iter().cloned();

    // Keep feeding back until completion
    // can't iter_mut().cycle() here :/
    for idx in (0..amps.len()).cycle() {
        if let Some(s) = amps[idx].run(in_iter.next(), signal) {
            signal = s;
        } else {
            break;
        }
    }

    signal
}

fn main() {
    let vals = [0, 1, 2, 3, 4];
    let perms = get_perms(&vals);

    let m = perms
        .iter()
        .map(|p| (p, run_amps(&p)))
        .max_by_key(|(_, r)| *r)
        .unwrap()
        .1;
    println!("Part 1: {}", m);

    let vals = [5, 6, 7, 8, 9];
    let perms = get_perms(&vals);
    let m = perms
        .iter()
        .map(|p| (p, run_amps2(&p)))
        .max_by_key(|(_, r)| *r)
        .unwrap()
        .1;
    println!("Part 2: {}", m);
}
