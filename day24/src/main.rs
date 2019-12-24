use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Pos = (i32, i32);
type Pos3 = (i32, i32, i32);

#[derive(Hash, Copy, Clone, Debug, PartialEq, Eq)]
enum C {
    Empty,
    Bug,
}

impl TryFrom<char> for C {
    type Error = ();
    fn try_from(c: char) -> Result<C, ()> {
        let out = match c {
            '.' => C::Empty,
            '#' => C::Bug,
            _ => return Err(()),
        };

        Ok(out)
    }
}

fn neighbors(area: &HashMap<Pos, C>, p: Pos) -> Vec<C> {
    let nlist = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    nlist
        .iter()
        .map(|&s| (s.0 + p.0, s.1 + p.1))
        .map(|n| area.get(&n).copied().unwrap_or(C::Empty))
        .collect()
}

fn l_edge(d: i32) -> Vec<Pos3> {
    vec![(0, 0, d), (0, 1, d), (0, 2, d), (0, 3, d), (0, 4, d)]
}

fn r_edge(d: i32) -> Vec<Pos3> {
    vec![(4, 0, d), (4, 1, d), (4, 2, d), (4, 3, d), (4, 4, d)]
}

fn u_edge(d: i32) -> Vec<Pos3> {
    vec![(0, 0, d), (1, 0, d), (2, 0, d), (3, 0, d), (4, 0, d)]
}

fn d_edge(d: i32) -> Vec<Pos3> {
    vec![(0, 4, d), (1, 4, d), (2, 4, d), (3, 4, d), (4, 4, d)]
}

fn wrap(p: i32) -> i32 {
    if p == -1 {
        1
    } else if p == 5 {
        3
    } else {
        unreachable!()
    }
}

fn is_wrap(p: i32) -> bool {
    p == -1 || p == 5
}

fn neighbors3(area: &HashMap<Pos3, C>, p: Pos3) -> Vec<C> {
    #[derive(Copy, Clone)]
    enum D {
        L,
        R,
        U,
        D,
    };

    // 2d vector + direction to know where to enter inner level from
    let nlist = [(0, 1, D::D), (0, -1, D::U), (1, 0, D::R), (-1, 0, D::L)];

    let l: Vec<_> = nlist
        .iter()
        .map(|&s| (s.0 + p.0, s.1 + p.1, p.2, s.2))
        .flat_map(|(x, y, d, dir)| {
            let p = (x, y);
            match p {
                (2, 2) => match dir {
                    D::L => r_edge(d + 1),
                    D::R => l_edge(d + 1),
                    D::U => d_edge(d + 1),
                    D::D => u_edge(d + 1),
                },
                (x, y) => {
                    let val = match (is_wrap(x), is_wrap(y)) {
                        (false, false) => (x, y, d),
                        (true, false) => (wrap(x), 2, d - 1),
                        (false, true) => (2, wrap(y), d - 1),
                        _ => unreachable!(),
                    };
                    vec![val]
                }
            }
        })
        .map(|p| {
            assert!(p.0 >= 0);
            assert!(p.1 >= 0);
            assert!(p.0 < 5);
            assert!(p.1 < 5);
            area.get(&p).copied().unwrap_or(C::Empty)
        })
        .collect();

    l
}

#[allow(dead_code)]
fn draw_map(area: &HashMap<Pos, C>) {
    'outer: for y in 0.. {
        for x in 0.. {
            if let Some(c) = area.get(&(x, y)) {
                let x = match c {
                    C::Empty => '.',
                    C::Bug => '#',
                };
                print!("{}", x);
            } else if x == 0 {
                break 'outer;
            } else {
                break;
            };
        }
        println!();
    }
}

fn calc_biod(area: &HashMap<Pos, C>) -> i32 {
    let mut sum = 0;

    'outer: for y in 0.. {
        for x in 0.. {
            if let Some(c) = area.get(&(x, y)) {
                if *c == C::Bug {
                    sum += 2_i32.pow((y * 5 + x) as u32);
                }
            } else if x == 0 {
                break 'outer;
            } else {
                break;
            };
        }
    }
    sum
}

fn p1() {
    let rdr = BufReader::new(File::open("input").unwrap());
    let mut area = HashMap::new();

    for (y, line) in rdr.lines().enumerate() {
        let line = line.unwrap();
        for (x, c) in line.chars().enumerate() {
            area.insert((x as i32, y as i32), c.try_into().unwrap());
        }
    }

    let mut past = HashMap::new();

    for t in 1.. {
        let mut area2 = HashMap::new();

        for (k, v) in area.iter() {
            let n = neighbors(&area, *k);
            let n = match v {
                C::Empty => {
                    let c = n.iter().filter(|&&a| a == C::Bug).count();
                    if c == 1 || c == 2 {
                        C::Bug
                    } else {
                        *v
                    }
                }
                C::Bug => {
                    let c = n.iter().filter(|&&a| a == C::Bug).count();
                    if c != 1 {
                        C::Empty
                    } else {
                        *v
                    }
                }
            };

            area2.insert(*k, n);
        }

        area = area2;

        let mut keys: Vec<_> = area.keys().collect();
        keys.sort();
        let sorted: Vec<_> = keys.iter().map(|k| area[&k]).collect();

        if past.contains_key(&sorted) {
            let p1 = calc_biod(&area);
            println!("Part 1: {}", p1);
            return;
        }

        past.insert(sorted, t);
    }
}

fn p2() {
    let rdr = BufReader::new(File::open("input").unwrap());
    let mut area = HashMap::new();

    for (y, line) in rdr.lines().enumerate() {
        let line = line.unwrap();
        for (x, c) in line.chars().enumerate() {
            if x == 2 && y == 2 {
                continue;
            }

            // Ugly but whatever
            for d in -250..=250 {
                area.insert((x as i32, y as i32, d), C::Empty);
            }
            area.insert((x as i32, y as i32, 0), c.try_into().unwrap());
        }
    }

    for _ in 1..=200 {
        let mut area2 = HashMap::new();

        for (k, v) in area.iter() {
            let n = neighbors3(&area, *k);
            let n = match v {
                C::Empty => {
                    let c = n.iter().filter(|&&a| a == C::Bug).count();
                    if c == 1 || c == 2 {
                        C::Bug
                    } else {
                        *v
                    }
                }
                C::Bug => {
                    let c = n.iter().filter(|&&a| a == C::Bug).count();
                    if c != 1 {
                        C::Empty
                    } else {
                        *v
                    }
                }
            };

            area2.insert(*k, n);
        }
        area = area2;
    }
    let bugs = area.values().filter(|&&v| v == C::Bug).count();
    println!("Part 2: {}", bugs);
}

fn main() {
    p1();
    p2();
}
