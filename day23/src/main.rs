use std::collections::{HashMap, VecDeque};
use std::convert::{TryFrom, TryInto};

#[derive(Debug)]
enum Mode {
    Pos,
    Im,
    Rel,
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
    Adj(Mode),
    Halt,
}

#[derive(Debug)]
enum ErrorIntcode {
    InvalidOpcode,
    InvalidMode,
}

impl TryFrom<i64> for Mode {
    type Error = ErrorIntcode;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Mode::Pos),
            1 => Ok(Mode::Im),
            2 => Ok(Mode::Rel),
            _ => Err(ErrorIntcode::InvalidMode),
        }
    }
}

impl TryFrom<i64> for Intcode {
    type Error = ErrorIntcode;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
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
            9 => Ok(Intcode::Adj(m1)),
            99 => Ok(Intcode::Halt),
            _ => Err(ErrorIntcode::InvalidOpcode),
        }
    }
}

fn read_mem() -> Vec<i64> {
    let text = std::fs::read_to_string("input").unwrap();
    let text = text.trim();
    text.split(',').map(|s| s.parse().unwrap()).collect()
}

fn mem_get(mem: &[i64], rel: usize, addr: usize, m: Mode) -> i64 {
    match m {
        Mode::Pos => mem[mem[addr] as usize],
        Mode::Im => mem[addr],
        Mode::Rel => mem[(mem[addr] + rel as i64) as usize],
    }
}

fn mem_set(mem: &mut [i64], rel: usize, addr: usize, m: Mode, v: i64) {
    match m {
        Mode::Pos => mem[mem[addr] as usize] = v,
        Mode::Im => mem[addr] = v,
        Mode::Rel => mem[(mem[addr] + rel as i64) as usize] = v,
    }
}

#[derive(Debug, Clone)]
struct Cpu {
    mem: Vec<i64>,
    pc: usize,
    rel: usize,
}

impl Cpu {
    fn new() -> Cpu {
        let mut mem = read_mem();
        // Ugly way to add more memory but whatevz
        mem.extend(std::iter::repeat(0).take(10000));
        Cpu { mem, pc: 0, rel: 0 }
    }

    fn run(&mut self, input: &mut Option<i64>) -> Option<i64> {
        loop {
            let op: Intcode = self.mem[self.pc].try_into().unwrap();
            match op {
                Intcode::Add(m1, m2, m3) => {
                    let (p_a, p_b) = (
                        mem_get(&self.mem, self.rel, self.pc + 1, m1),
                        mem_get(&self.mem, self.rel, self.pc + 2, m2),
                    );
                    mem_set(&mut self.mem, self.rel, self.pc + 3, m3, p_a + p_b);
                    self.pc += 4;
                }
                Intcode::Mult(m1, m2, m3) => {
                    let (p_a, p_b) = (
                        mem_get(&self.mem, self.rel, self.pc + 1, m1),
                        mem_get(&self.mem, self.rel, self.pc + 2, m2),
                    );
                    mem_set(&mut self.mem, self.rel, self.pc + 3, m3, p_a * p_b);
                    self.pc += 4;
                }
                Intcode::In(m1) => {
                    if input.is_none() {
                        return None;
                    }
                    mem_set(
                        &mut self.mem,
                        self.rel,
                        self.pc + 1,
                        m1,
                        input.take().expect("No input"),
                    );
                    self.pc += 2;
                }
                Intcode::Out(m1) => {
                    let p_a = mem_get(&self.mem, self.rel, self.pc + 1, m1);
                    self.pc += 2;
                    return Some(p_a);
                }
                Intcode::Jit(m1, m2) => {
                    let (p_a, p_b) = (
                        mem_get(&self.mem, self.rel, self.pc + 1, m1),
                        mem_get(&self.mem, self.rel, self.pc + 2, m2),
                    );
                    if p_a != 0 {
                        self.pc = p_b as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                Intcode::Jif(m1, m2) => {
                    let (p_a, p_b) = (
                        mem_get(&self.mem, self.rel, self.pc + 1, m1),
                        mem_get(&self.mem, self.rel, self.pc + 2, m2),
                    );
                    if p_a == 0 {
                        self.pc = p_b as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                Intcode::Lt(m1, m2, m3) => {
                    let (p_a, p_b) = (
                        mem_get(&self.mem, self.rel, self.pc + 1, m1),
                        mem_get(&self.mem, self.rel, self.pc + 2, m2),
                    );
                    mem_set(&mut self.mem, self.rel, self.pc + 3, m3, (p_a < p_b) as i64);
                    self.pc += 4;
                }
                Intcode::Equ(m1, m2, m3) => {
                    let (p_a, p_b) = (
                        mem_get(&self.mem, self.rel, self.pc + 1, m1),
                        mem_get(&self.mem, self.rel, self.pc + 2, m2),
                    );
                    mem_set(
                        &mut self.mem,
                        self.rel,
                        self.pc + 3,
                        m3,
                        (p_a == p_b) as i64,
                    );
                    self.pc += 4;
                }
                Intcode::Adj(m1) => {
                    let p_a = mem_get(&self.mem, self.rel, self.pc + 1, m1);
                    self.rel = (self.rel as i64 + p_a) as usize;
                    self.pc += 2;
                }
                Intcode::Halt => {
                    break;
                }
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
struct Nic {
    cpu: Cpu,
    in_q: VecDeque<i64>,
    out_q: VecDeque<i64>,
}

impl Nic {
    fn new() -> Nic {
        Nic {
            cpu: Cpu::new(),
            out_q: VecDeque::new(),
            in_q: VecDeque::new(),
        }
    }

    fn tick(&mut self) {
        if self.in_q.is_empty() {
            let mut feed = Some(-1);
            while let Some(output) = self.cpu.run(&mut feed) {
                self.out_q.push_back(output);
            }
        } else {
            while let Some(input) = self.in_q.pop_front() {
                let mut feed = Some(input);
                while feed.is_some() {
                    if let Some(output) = self.cpu.run(&mut feed) {
                        self.out_q.push_back(output);
                    }
                }
            }
            while let Some(output) = self.cpu.run(&mut None) {
                self.out_q.push_back(output);
            }
        }
    }

    fn set_id(&mut self, id: i64) {
        let mut val = Some(id);
        self.cpu.run(&mut val);
        assert!(val.is_none());
    }
}

fn main() {
    let mut nics = vec![Nic::new(); 50];

    let mut queues: HashMap<i64, VecDeque<i64>> = HashMap::new();
    for (id, nic) in nics.iter_mut().enumerate() {
        nic.set_id(id as i64);
        queues.insert(id as i64, VecDeque::new());
    }

    let mut p1 = false;
    let mut nat = (None, None);
    let mut last_nat_y = None;

    loop {
        for (id, nic) in nics.iter_mut().enumerate() {
            let id = id as i64;

            let q = queues.get_mut(&id).unwrap();
            while let Some(p) = q.pop_front() {
                nic.in_q.push_back(p);
            }

            nic.tick();

            assert_eq!(nic.out_q.len() % 3, 0);
            while let (Some(dst), Some(x), Some(y)) = (
                nic.out_q.pop_front(),
                nic.out_q.pop_front(),
                nic.out_q.pop_front(),
            ) {
                if dst == 255 {
                    if !p1 {
                        println!("Part 1: {}", y);
                        p1 = true;
                    }
                    nat = (Some(x), Some(y));
                } else {
                    let q = queues.get_mut(&dst).unwrap();
                    q.push_back(x);
                    q.push_back(y);
                }
            }
        }

        let idle = queues.values().all(|q| q.is_empty());

        if idle {
            let nic0 = queues.get_mut(&0).unwrap();
            if let (Some(x), Some(y)) = nat {
                nic0.push_back(x);
                nic0.push_back(y);

                if let Some(y_last) = last_nat_y {
                    if y == y_last {
                        println!("Part 2: {}", y);
                        return;
                    }
                }
                last_nat_y = Some(y);
            } else {
                panic!("NAT empty in idle");
            }
        }
    }
}
