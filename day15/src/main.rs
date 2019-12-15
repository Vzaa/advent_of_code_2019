use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

type Pos = (i64, i64);
type TileMap = HashMap<Pos, Tile>;

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

    fn run(&mut self, mut input: Option<i64>) -> Option<i64> {
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
                    mem_set(
                        &mut self.mem,
                        self.rel,
                        self.pc + 1,
                        m1,
                        input.take().unwrap(),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Oxygen,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    N,
    S,
    W,
    E,
}

#[derive(Debug)]
enum Resp {
    Hit,
    Mv,
    MvO,
}

impl TryFrom<i64> for Resp {
    type Error = ();

    fn try_from(v: i64) -> Result<Self, Self::Error> {
        let out = match v {
            0 => Resp::Hit,
            1 => Resp::Mv,
            2 => Resp::MvO,
            _ => return Err(()),
        };

        Ok(out)
    }
}

impl From<Dir> for i64 {
    fn from(d: Dir) -> i64 {
        match d {
            Dir::N => 1,
            Dir::S => 2,
            Dir::W => 3,
            Dir::E => 4,
        }
    }
}

impl Dir {
    fn mv(self, p: Pos) -> Pos {
        match self {
            Dir::N => (p.0, p.1 + 1),
            Dir::S => (p.0, p.1 - 1),
            Dir::W => (p.0 + 1, p.1),
            Dir::E => (p.0 - 1, p.1),
        }
    }

    fn opposite(self) -> Dir {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::W => Dir::E,
            Dir::E => Dir::W,
        }
    }
}

#[derive(Debug)]
struct Droid {
    pos: Pos,
    cpu: Cpu,
}

impl Droid {
    fn new() -> Droid {
        Droid {
            pos: (0, 0),
            cpu: Cpu::new(),
        }
    }

    fn mv(&mut self, d: Dir) -> (Pos, Tile) {
        let r: Resp = self
            .cpu
            .run(Some(d.into()))
            .expect("CPU Halted Unexpectedly")
            .try_into()
            .expect("Invalid Output");

        let (p, t) = match r {
            Resp::Hit => (d.mv(self.pos), Tile::Wall),
            Resp::Mv => (d.mv(self.pos), Tile::Empty),
            Resp::MvO => (d.mv(self.pos), Tile::Oxygen),
        };

        self.pos = match r {
            Resp::Hit => self.pos,
            Resp::Mv => d.mv(self.pos),
            Resp::MvO => d.mv(self.pos),
        };

        (p, t)
    }
}

#[allow(dead_code)]
fn draw_map(tilemap: &TileMap) {
    let (max_x, max_y) = (
        tilemap.keys().map(|p| p.0).max().unwrap(),
        tilemap.keys().map(|p| p.1).max().unwrap(),
    );

    let (min_x, min_y) = (
        tilemap.keys().map(|p| p.0).min().unwrap(),
        tilemap.keys().map(|p| p.1).min().unwrap(),
    );

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let t = tilemap.get(&(x, y)).unwrap_or(&Tile::Unknown);

            let c = match t {
                Tile::Empty => '.',
                Tile::Wall => '#',
                Tile::Oxygen => 'O',
                Tile::Unknown => ' ',
            };

            print!("{}", c);
        }
        println!();
    }
}

fn get4(p: Pos) -> [(Dir, Pos); 4] {
    [
        (Dir::N, Dir::N.mv(p)),
        (Dir::E, Dir::E.mv(p)),
        (Dir::S, Dir::S.mv(p)),
        (Dir::W, Dir::W.mv(p)),
    ]
}

fn find_oxygen(droid: &mut Droid, tilemap: &mut TileMap, depth: i32) -> bool {
    let dpos = droid.pos;
    let env = get4(dpos);

    for (d, p) in env.iter() {
        let t = *tilemap.get(p).unwrap_or(&Tile::Unknown);
        if t == Tile::Unknown {
            let (np, nt) = droid.mv(*d);
            tilemap.insert(np, nt);

            if nt == Tile::Oxygen {
                println!("Part 1: {}", depth);
                return true;
            } else if nt == Tile::Empty {
                if find_oxygen(droid, tilemap, depth + 1) {
                    return true;
                }
                droid.mv(d.opposite());
            }
        }
    }
    false
}

fn explore_full(droid: &mut Droid, tilemap: &mut TileMap) {
    let dpos = droid.pos;
    let env = get4(dpos);

    for (d, p) in env.iter() {
        let t = *tilemap.get(p).unwrap_or(&Tile::Unknown);

        if t == Tile::Unknown {
            let (np, nt) = droid.mv(*d);
            tilemap.insert(np, nt);

            if nt == Tile::Oxygen || nt == Tile::Empty {
                explore_full(droid, tilemap);
                droid.mv(d.opposite());
            }
        }
    }
}

fn spread_oxygen(tilemap: &mut TileMap) {
    let oxygens: Vec<_> = tilemap
        .iter()
        .filter(|(_, &v)| v == Tile::Oxygen)
        .map(|(k, _)| *k)
        .collect();
    for opos in oxygens {
        for (_, p) in get4(opos).iter() {
            let t = tilemap.get_mut(p).unwrap();
            if *t == Tile::Empty {
                *t = Tile::Oxygen;
            }
        }
    }
}

fn main() {
    // Part 1
    {
        let mut tilemap: HashMap<Pos, Tile> = HashMap::new();
        let mut droid = Droid::new();

        let t = find_oxygen(&mut droid, &mut tilemap, 1);
        assert!(t);
    }

    // Part 2
    {
        let mut tilemap: HashMap<Pos, Tile> = HashMap::new();
        let mut droid = Droid::new();
        explore_full(&mut droid, &mut tilemap);

        for c in 1.. {
            spread_oxygen(&mut tilemap);
            let e_cnt: usize = tilemap.values().filter(|&&v| v == Tile::Empty).count();
            if e_cnt == 0 {
                println!("Part 2: {}", c);
                break;
            }
        }
    }
}
