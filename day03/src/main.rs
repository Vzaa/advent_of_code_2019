use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

type Pos = (i32, i32);

#[derive(Debug)]
enum Dir {
    L(i32),
    R(i32),
    U(i32),
    D(i32),
}

impl Dir {
    fn len(&self) -> i32 {
        match self {
            Dir::L(x) => *x,
            Dir::R(x) => *x,
            Dir::U(x) => *x,
            Dir::D(x) => *x,
        }
    }

    // Get pos + one unit in this direction
    fn mv(&self, p: Pos) -> Pos {
        match self {
            Dir::L(_) => (p.0 - 1, p.1),
            Dir::R(_) => (p.0 + 1, p.1),
            Dir::U(_) => (p.0, p.1 + 1),
            Dir::D(_) => (p.0, p.1 - 1),
        }
    }
}

#[derive(Debug)]
enum ParseDirError {
    Int(ParseIntError),
    InvalidDir,
    Empty,
}

impl From<ParseIntError> for ParseDirError {
    fn from(e: ParseIntError) -> Self {
        ParseDirError::Int(e)
    }
}

impl FromStr for Dir {
    type Err = ParseDirError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir = s.chars().nth(0).ok_or(ParseDirError::Empty)?;
        let mag: i32 = s[1..].parse()?;

        match dir {
            'L' => Ok(Dir::L(mag)),
            'R' => Ok(Dir::R(mag)),
            'U' => Ok(Dir::U(mag)),
            'D' => Ok(Dir::D(mag)),
            _ => Err(ParseDirError::InvalidDir),
        }
    }
}

fn m_dist_o(a: Pos) -> i32 {
    a.0.abs() + a.1.abs()
}

fn main() {
    let mut line_rdr = BufReader::new(File::open("input").unwrap()).lines();
    let mut wires = Vec::new();
    while let Some(Ok(line)) = line_rdr.next() {
        let wire: Vec<Dir> = line.split(',').map(|s| s.parse().unwrap()).collect();
        wires.push(wire);
    }

    let mut wire_maps = Vec::new();
    for wire in &wires {
        let mut wire_map = HashMap::new();
        let mut pos = (0, 0);
        let mut dist = 0;
        for dir in wire {
            for _ in 0..dir.len() {
                dist += 1;
                pos = dir.mv(pos);
                wire_map.entry(pos).or_insert(dist);
            }
        }
        wire_maps.push(wire_map);
    }

    let isect: Vec<_> = wire_maps[0]
        .keys()
        .filter(|p| wire_maps[1].contains_key(p))
        .map(|p| *p)
        .collect();

    let minp = *isect.iter().min_by_key(|&&p| m_dist_o(p)).unwrap();
    println!("Part 1: {:?}", m_dist_o(minp));

    let minp = isect
        .iter()
        .min_by_key(|&p| wire_maps[0][p] + wire_maps[1][p])
        .unwrap();

    println!("Part 2: {:?}", wire_maps[0][minp] + wire_maps[1][minp]);
}
