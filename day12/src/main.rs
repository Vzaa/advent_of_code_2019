use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

type Pos = (i32, i32, i32);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Moon {
    p: Pos,
    v: Pos,
}

#[derive(Debug)]
enum ParseError {
    Int(ParseIntError),
    Empty,
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError::Int(e)
    }
}

// From last year
impl FromStr for Moon {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let clean: String = s
            .matches(|c: char| c.is_numeric() || c == ' ' || c == '-' || c == ',')
            .collect::<String>()
            .replace(",", " ");

        let mut it = clean.split_whitespace();
        let x: i32 = it.next().ok_or(ParseError::Empty)?.parse()?;
        let y: i32 = it.next().ok_or(ParseError::Empty)?.parse()?;
        let z: i32 = it.next().ok_or(ParseError::Empty)?.parse()?;

        Ok(Moon {
            p: (x, y, z),
            v: (0, 0, 0),
        })
    }
}

fn add(a: Pos, b: Pos) -> Pos {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn abs_mag(a: Pos) -> i32 {
    a.0.abs() + a.1.abs() + a.2.abs()
}

impl Moon {
    fn apply_g(&mut self, o: Pos) {
        if self.p.0 < o.0 {
            self.v.0 += 1;
        } else if self.p.0 > o.0 {
            self.v.0 -= 1;
        }

        if self.p.1 < o.1 {
            self.v.1 += 1;
        } else if self.p.1 > o.1 {
            self.v.1 -= 1;
        }

        if self.p.2 < o.2 {
            self.v.2 += 1;
        } else if self.p.2 > o.2 {
            self.v.2 -= 1;
        }
    }

    fn apply_v(&mut self) {
        self.p = add(self.p, self.v);
    }

    fn kin(&self) -> i32 {
        abs_mag(self.p) * abs_mag(self.v)
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

fn main() {
    // Part 1
    {
        let rdr = BufReader::new(File::open("input").unwrap());
        let mut moons: Vec<Moon> = rdr.lines().map(|l| l.unwrap().parse().unwrap()).collect();

        for _ in 0..1000 {
            for i in 0..moons.len() {
                for j in 0..moons.len() {
                    if i == j {
                        continue;
                    }
                    let p = moons[j].p;
                    moons[i].apply_g(p);
                }
            }

            for m in &mut moons {
                m.apply_v();
            }
        }

        let s = moons.iter().map(Moon::kin).sum::<i32>();
        println!("Part 1: {}", s);
    }

    // Part 2
    {
        let rdr = BufReader::new(File::open("input").unwrap());
        let mut moons: Vec<Moon> = rdr.lines().map(|l| l.unwrap().parse().unwrap()).collect();
        let (mut past_x, mut past_y, mut past_z) = (HashSet::new(), HashSet::new(), HashSet::new());
        let (mut turn_x, mut turn_y, mut turn_z) = (None, None, None);

        for turn in 0_usize.. {
            let state_x: Vec<_> = moons.iter().map(|m| (m.p.0, m.v.0)).collect();
            let state_y: Vec<_> = moons.iter().map(|m| (m.p.1, m.v.1)).collect();
            let state_z: Vec<_> = moons.iter().map(|m| (m.p.2, m.v.2)).collect();

            if !past_x.insert(state_x) && turn_x.is_none() {
                turn_x = Some(turn);
            }

            if !past_y.insert(state_y) && turn_y.is_none() {
                turn_y = Some(turn);
            }

            if !past_z.insert(state_z) && turn_z.is_none() {
                turn_z = Some(turn);
            }

            if turn_x.is_some() && turn_y.is_some() && turn_z.is_some() {
                break;
            }

            for i in 0..moons.len() {
                for j in 0..moons.len() {
                    if i == j {
                        continue;
                    }
                    let p = moons[j].p;
                    moons[i].apply_g(p);
                }
            }

            for m in &mut moons {
                m.apply_v();
            }
        }

        let (turn_x, turn_y, turn_z) = (turn_x.unwrap(), turn_y.unwrap(), turn_z.unwrap());
        let common = lcm(lcm(turn_x, turn_y), turn_z);
        println!("Part 2: {}", common);
    }
}
