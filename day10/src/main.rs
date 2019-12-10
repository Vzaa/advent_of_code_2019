use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Pos = (i32, i32);
type StarMap = HashMap<Pos, Spot>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Spot {
    Open,
    Asteroid,
}

impl TryFrom<char> for Spot {
    type Error = ();

    fn try_from(v: char) -> Result<Self, Self::Error> {
        let out = match v {
            '.' => Spot::Open,
            '#' => Spot::Asteroid,
            _ => return Err(()),
        };

        Ok(out)
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

fn scale(a: Pos, s: i32) -> Pos {
    (a.0 * s, a.1 * s)
}

fn div(a: Pos, d: i32) -> Pos {
    (a.0 / d, a.1 / d)
}

fn diff(a: Pos, b: Pos) -> Pos {
    (a.0 - b.0, a.1 - b.1)
}

fn add(a: Pos, b: Pos) -> Pos {
    (a.0 + b.0, a.1 + b.1)
}

fn is_blocked(p: Pos, o: Pos, starmap: &StarMap) -> bool {
    let v = diff(o, p);
    let denom = gcd(v.0.abs(), v.1.abs());
    let step = div(v, denom);
    (1..denom) // all possible jumps
        .map(|n| add(p, scale(step, n))) // get nth jump
        .any(|v| starmap.get(&v) == Some(&Spot::Asteroid))
}

fn get_los(p: Pos, starmap: &StarMap) -> usize {
    starmap
        .iter()
        .filter(|(&k, &v)| k != p && v == Spot::Asteroid && !is_blocked(p, k, &starmap))
        .count()
}

fn calc_slope(p: Pos) -> f32 {
    (p.0 as f32).atan2(p.1 as f32)
}

fn ordered_hitlist(p: Pos, starmap: &StarMap) -> Vec<Pos> {
    let mut all: Vec<_> = starmap
        .iter()
        .filter(|(&k, &v)| k != p && v == Spot::Asteroid && !is_blocked(p, k, &starmap))
        .map(|a| *a.0)
        .collect();

    all.sort_by(|a, b| {
        let v_a = diff(*a, p);
        let v_b = diff(*b, p);
        calc_slope(v_b).partial_cmp(&calc_slope(v_a)).unwrap()
    });

    all
}

fn main() {
    let rdr = BufReader::new(File::open("input").unwrap());
    let mut starmap: HashMap<Pos, Spot> = HashMap::new();

    for (y, line) in rdr.lines().enumerate() {
        let line = line.unwrap();
        for (x, c) in line.chars().enumerate() {
            let (x, y) = (x as i32, y as i32);
            starmap.insert((x, y), c.try_into().unwrap());
        }
    }
    let max_los = *starmap
        .iter()
        .filter(|(_, &v)| v == Spot::Asteroid)
        .max_by_key(|(&p, _)| get_los(p, &starmap))
        .unwrap()
        .0;

    println!("Part 1: {:?}", get_los(max_los, &starmap));

    let mut cnt = 0;
    loop {
        let hitlist = ordered_hitlist(max_los, &starmap);
        for hit in hitlist {
            starmap.insert(hit, Spot::Open);
            cnt += 1;
            if cnt == 200 {
                println!("Part 2: {}", hit.0 * 100 + hit.1);
                return;
            }
        }
    }
}
