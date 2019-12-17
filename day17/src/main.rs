use std::collections::{HashMap, HashSet};
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
    Open,
    Scaffold,
    Robot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    N,
    S,
    W,
    E,
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
enum Turn {
    L,
    R,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        let out = match c {
            '#' => Tile::Scaffold,
            '.' => Tile::Open,
            '^' => Tile::Robot,
            _ => return Err(()),
        };

        Ok(out)
    }
}

impl Dir {
    fn mv(self, p: Pos) -> Pos {
        match self {
            Dir::N => (p.0, p.1 - 1),
            Dir::S => (p.0, p.1 + 1),
            Dir::W => (p.0 - 1, p.1),
            Dir::E => (p.0 + 1, p.1),
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

    fn get_i64(&mut self) -> Option<i64> {
        self.cpu.run(&mut None)
    }

    fn get_char(&mut self) -> Option<char> {
        self.cpu.run(&mut None).map(|v| (v as u8).into())
    }

    fn start(&mut self) {
        self.cpu.mem[0] = 2;
    }

    fn write_fn(&mut self, fntext: &str) {
        for c in fntext.chars() {
            let asc = c as u8;
            let mut feed = Some(asc as i64);

            while feed.is_some() {
                if let Some(o) = self.cpu.run(&mut feed) {
                    print!("{}", o as u8 as char);
                }
            }
        }

        let mut feed = Some('\n' as i64);
        while feed.is_some() {
            if let Some(o) = self.cpu.run(&mut feed) {
                print!("{}", o as u8 as char);
            }
        }
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
            let t = tilemap.get(&(x, y)).unwrap_or(&Tile::Open);

            let c = match t {
                Tile::Open => '.',
                Tile::Scaffold => '#',
                Tile::Robot => '^',
            };

            print!("{}", c);
        }
        println!();
    }
}

fn get4(p: Pos) -> [Pos; 4] {
    [Dir::N.mv(p), Dir::E.mv(p), Dir::S.mv(p), Dir::W.mv(p)]
}

fn get4w(p: Pos) -> [(Dir, Pos); 4] {
    [
        (Dir::N, Dir::N.mv(p)),
        (Dir::E, Dir::E.mv(p)),
        (Dir::S, Dir::S.mv(p)),
        (Dir::W, Dir::W.mv(p)),
    ]
}

fn turns_to_cmd(turns: &[(Turn, i32)]) -> String {
    let tx: Vec<String> = turns
        .iter()
        .map(|(t, v)| format!("{},{}", t.to_c(), v))
        .collect();
    tx.join(",")
}

impl Turn {
    fn from_dirs(a: Dir, b: Dir) -> Turn {
        match (a, b) {
            (Dir::N, Dir::E) => Turn::R,
            (Dir::N, Dir::W) => Turn::L,
            (Dir::S, Dir::E) => Turn::L,
            (Dir::S, Dir::W) => Turn::R,
            (Dir::W, Dir::N) => Turn::R,
            (Dir::W, Dir::S) => Turn::L,
            (Dir::E, Dir::N) => Turn::L,
            (Dir::E, Dir::S) => Turn::R,
            _ => panic!("Invalid"),
        }
    }

    fn to_c(self) -> char {
        match self {
            Turn::L => 'L',
            Turn::R => 'R',
        }
    }
}

fn moves_to_turns(moves: &[(Dir, i32)]) -> Vec<(Turn, i32)> {
    let mut dir = Dir::N;
    let mut turns = vec![];

    for &(ndir, val) in moves {
        let turn = Turn::from_dirs(dir, ndir);
        turns.push((turn, val));
        dir = ndir;
    }

    turns
}

fn main() {
    // Part 1
    let mut tilemap: HashMap<Pos, Tile> = HashMap::new();
    let mut droid = Droid::new();
    {
        let mut p = (0, 0);
        while let Some(v) = droid.get_char() {
            match v {
                '\n' => p = (0, p.1 + 1),
                c => {
                    tilemap.insert(p, c.try_into().unwrap());
                    p = (p.0 + 1, p.1);
                }
            }
        }

        let v: i64 = tilemap
            .iter()
            .filter(|(_, &v)| v == Tile::Scaffold || v == Tile::Robot)
            .filter(|(&k, _)| {
                get4(k).iter().all(|k| {
                    tilemap.get(k) == Some(&Tile::Robot) || tilemap.get(k) == Some(&Tile::Scaffold)
                })
            })
            .map(|(&k, _)| k.0 * k.1)
            .sum();
        println!("Part 1: {}", v);
    }

    // Part 2
    {
        let mut rpos: Pos = tilemap
            .iter()
            .find(|(_, &v)| v == Tile::Robot)
            .map(|(k, _)| *k)
            .unwrap();

        let mut moves = vec![];
        let mut dir = Dir::E;
        let mut steps = 0;
        loop {
            let next = dir.mv(rpos);
            if tilemap.get(&next) == Some(&Tile::Scaffold) {
                rpos = next;
                steps += 1;
            } else {
                moves.push((dir, steps));
                steps = 0;

                // Ugly:
                if let Some(d) = get4w(rpos)
                    .iter()
                    .filter(|(d, _)| *d != dir.opposite())
                    .find(|(_, p)| tilemap.get(p) == Some(&Tile::Scaffold))
                {
                    dir = d.0;
                } else {
                    break;
                }
            }
        }

        let turns = moves_to_turns(&moves);
        println!("{}", turns_to_cmd(&turns));

        let mut repeats = HashSet::new();

        {
            let rip = turns.clone();
            for s in (3..7).rev() {
                for b in 0..(rip.len() - s) {
                    let sl = &rip[b..b + s];

                    let cnt: usize = rip.windows(s).filter(|w| *w == sl).count();
                    if cnt > 1 {
                        let txt = turns_to_cmd(sl);
                        if txt.len() <= 20 {
                            repeats.insert((sl.to_owned(), cnt * txt.len()));
                        }
                    }
                }
            }
        }

        let mut repeats: Vec<_> = repeats.iter().collect();
        repeats.sort_by_key(|a| a.1);

        for r in &repeats {
            println!("{}", turns_to_cmd(&r.0));
        }

        // Solved by hand with the output printed from above
        let mut tt = turns_to_cmd(&turns);
        let a = "R,8,L,12,R,8";
        let b = "L,12,L,12,L,10,R,10";
        let c = "L,10,L,10,R,8";
        tt = tt.replace(&a, "A");
        tt = tt.replace(&b, "B");
        tt = tt.replace(&c, "C");

        let mut droid = Droid::new();
        droid.start();
        droid.write_fn(&tt);
        droid.write_fn(a);
        droid.write_fn(b);
        droid.write_fn(c);
        droid.write_fn("n");

        while let Some(v) = droid.get_i64() {
            let c: Result<u8, _> = v.try_into();
            if let Ok(c) = c {
                print!("{}", c as char);
            } else {
                print!("Part 2: {}", v);
            }
        }
        println!();
    }
}
