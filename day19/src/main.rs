use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

type Pos = (i64, i64);
type TileMap = HashMap<Pos, i64>;

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
        mem.extend(std::iter::repeat(0).take(1024));
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
                    //println!("{}", p_a);
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

#[derive(Debug)]
struct Drone {
    cpu: Cpu,
}

impl Drone {
    fn new() -> Drone {
        Drone { cpu: Cpu::new() }
    }

    fn feed_pos(&mut self, p: Pos) -> Option<i64> {
        let mut tmp = Some(p.0);
        self.cpu.run(&mut tmp);

        tmp = Some(p.1);
        self.cpu.run(&mut tmp)
    }
}

#[allow(dead_code)]
fn draw_map(tilemap: &HashMap<Pos, i64>) {
    let (max_x, max_y) = (
        tilemap.keys().map(|p| p.0).max().unwrap(),
        tilemap.keys().map(|p| p.1).max().unwrap(),
    );

    let (min_x, min_y) = (
        tilemap.keys().map(|p| p.0).min().unwrap(),
        tilemap.keys().map(|p| p.1).min().unwrap(),
    );

    for y in min_y..=max_y {
        print!("{:2}: ", y);
        for x in min_x..=max_x {
            let t = tilemap.get(&(x, y)).unwrap_or(&0);

            let c = match t {
                0 => '.',
                1 => '#',
                _ => panic!("rip"),
            };

            print!("{}", c);
        }
        println!();
    }
}

fn check_sq(p: Pos, tilemap: &TileMap) -> bool {
    for y in p.1..(p.1 + 100) {
        for x in p.0..(p.0 + 100) {
            if *tilemap.get(&(x, y)).unwrap_or(&0) != 1 {
                return false;
            }
        }
    }
    true
}

fn check_from_y(tilemap: &TileMap, y_start: i64) -> Option<i64> {
    let max_x = tilemap.keys().map(|p| p.0).max().unwrap();

    for x in 0..=max_x {
        if check_sq((x, y_start), &tilemap) {
            return Some(x * 10000 + y_start);
        }
    }
    None
}

fn main() {
    let mut tilemap: HashMap<Pos, i64> = HashMap::new();
    let mut drone = Drone::new();
    // need to reset cpu for some reason
    let back = drone.cpu.clone();

    let mut sum = 0;
    for y in 0..=49 {
        for x in 0..=49 {
            drone.cpu = back.clone();
            let p = (x, y);
            let r = drone.feed_pos(p).expect("no resp");
            tilemap.insert(p, r);
            sum += r;
        }
    }
    println!("Part 1: {}", sum);

    for y in 0.. {
        let mut detected = false;
        for x in 0..y {
            drone.cpu = back.clone();
            let p = (x, y);
            let r = drone.feed_pos(p).expect("no resp");
            tilemap.insert(p, r);
            if r == 1 {
                detected = true;
            }
            if r == 0 && detected {
                break;
            }
        }
        if let Some(p2) = check_from_y(&tilemap, y - 100) {
            println!("Part 2: {}", p2);
            break;
        }
    }
}
