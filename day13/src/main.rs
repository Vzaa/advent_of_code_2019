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
    Block,
    HPaddle,
    Ball,
}

#[derive(Debug)]
enum ToS { // Tile or Score
    PosTile(Pos, Tile),
    Score(i64),
}

impl TryFrom<i64> for Tile {
    type Error = ();

    fn try_from(v: i64) -> Result<Self, Self::Error> {
        let out = match v {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HPaddle,
            4 => Tile::Ball,
            _ => return Err(()),
        };

        Ok(out)
    }
}

#[derive(Debug)]
struct Arcade {
    pos: Pos,
    cpu: Cpu,
}

impl Arcade {
    fn new() -> Arcade {
        Arcade {
            pos: (0, 0),
            cpu: Cpu::new(),
        }
    }

    fn set_free_play(&mut self) {
        self.cpu.mem[0] = 2;
    }

    fn get_tile(&mut self) -> Option<(Pos, Tile)> {
        let x = self.cpu.run(None)?;
        let y = self.cpu.run(None)?;
        let t = self.cpu.run(None)?.try_into().unwrap();

        Some(((x, y), t))
    }

    fn get_tos(&mut self, input: i64) -> Option<ToS> {
        let x = self.cpu.run(Some(input))?;
        let y = self.cpu.run(None)?;
        let t = self.cpu.run(None)?;

        if x == -1 && y == 0 {
            Some(ToS::Score(t))
        } else {
            Some(ToS::PosTile((x, y), t.try_into().unwrap()))
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
            let t = tilemap.get(&(x, y)).unwrap_or(&Tile::Empty);

            let c = match t {
                Tile::Empty => ' ',
                Tile::Wall => '#',
                Tile::Block => '=',
                Tile::HPaddle => '_',
                Tile::Ball => 'o',
            };

            print!("{}", c);
        }
        println!();
    }
}

fn main() {
    // Part 1
    {
        let mut tilemap: HashMap<Pos, Tile> = HashMap::new();
        let mut arcade = Arcade::new();

        while let Some((p, t)) = arcade.get_tile() {
            tilemap.insert(p, t);
        }
        let c: usize = tilemap.values().filter(|&&t| t == Tile::Block).count();
        println!("Part 1: {}", c);
    }

    // Part 2
    {
        let mut tilemap: HashMap<Pos, Tile> = HashMap::new();
        let mut arcade = Arcade::new();

        arcade.set_free_play();

        let mut score = 0;
        let mut input = 0;
        while let Some(tos) = arcade.get_tos(input) {
            match tos {
                ToS::Score(s) => score = s,
                ToS::PosTile(p, t) => {
                    tilemap.insert(p, t);
                }
            }

            let paddle_x = tilemap
                .iter()
                .find(|(_, &t)| t == Tile::HPaddle)
                .map(|(p, _)| p)
                .unwrap_or(&(0, 0))
                .0;

            let ball_x = tilemap
                .iter()
                .find(|(_, &t)| t == Tile::Ball)
                .map(|(p, _)| p)
                .unwrap_or(&(0, 0))
                .0;

            if ball_x > paddle_x {
                input = 1;
            } else if ball_x < paddle_x {
                input = -1;
            } else {
                input = 0;
            }
        }
        println!("Part 2: {}", score);
    }
}
