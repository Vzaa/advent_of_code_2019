use std::collections::{HashMap, HashSet};
use std::convert::{TryFrom, TryInto};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Pos = (i32, i32);
type TileMap = HashMap<Pos, Tile>;
type PortalMap = HashMap<(char, char), Vec<Pos>>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Passage,
    Wall,
    Empty,
    P(char),
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        let out = match c {
            '#' => Tile::Wall,
            '.' => Tile::Passage,
            ' ' => Tile::Empty,
            c if c.is_ascii_uppercase() => Tile::P(c),
            _ => return Err(()),
        };
        Ok(out)
    }
}

fn movable(map: &TileMap, p: Pos) -> impl Iterator<Item = Pos> + '_ {
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .iter()
        .map(move |&s| (s.0 + p.0, s.1 + p.1))
        .filter(move |s| {
            let t = map[&s];
            match t {
                Tile::Passage => true,
                _ => false,
            }
        })
}

fn find(map: &TileMap, pmap: &PortalMap, start: Pos, finish: Pos) -> Option<i32> {
    let mut dmap = HashSet::new();

    dmap.insert(start);
    let mut last = vec![start]; // vec is faster than hashset for the input size

    for dist in 0.. {
        let mut cur = vec![];
        for p in &last {
            if *p == finish {
                return Some(dist);
            }

            for s in movable(map, *p) {
                if !dmap.contains(&s) && !cur.contains(&s) {
                    cur.push(s);
                }
            }

            let portal = pmap.values().find(|v| v.contains(&p));
            if let Some(por) = portal {
                let exit = por.iter().find(|e| **e != *p);
                if let Some(e) = exit {
                    if !dmap.contains(&e) && !cur.contains(&e) {
                        cur.push(*e);
                    }
                }
            }
        }

        if cur.is_empty() {
            break;
        }

        for t in &cur {
            dmap.insert(*t);
        }
        last = cur;
    }
    None
}

fn is_outer(p: Pos, c: Pos) -> bool {
    p.0 <= 2 || p.1 <= 2 || p.0 >= c.0 - 3 || p.1 >= c.1 - 3
}

fn find_depth(map: &TileMap, pmap: &PortalMap, start: Pos, finish: Pos) -> Option<i32> {
    let mut dmap = HashSet::new();

    let (max_x, max_y) = (
        map.keys().map(|p| p.0).max().unwrap(),
        map.keys().map(|p| p.1).max().unwrap(),
    );

    let corner = (max_x, max_y);

    dmap.insert((start, 0));
    let mut last = vec![(start, 0)];

    for dist in 0.. {
        let mut cur = vec![];
        for &(pnt, depth) in &last {
            if (pnt, depth) == (finish, 0) {
                return Some(dist);
            }

            for s in movable(map, pnt) {
                if !dmap.contains(&(s, depth)) && !cur.contains(&(s, depth)) {
                    cur.push((s, depth));
                }
            }

            let portal = pmap.values().find(|v| v.contains(&pnt));
            if let Some(por) = portal {
                let exit = por.iter().find(|e| **e != pnt);
                if let Some(e) = exit {
                    let new_depth = if is_outer(pnt, corner) {
                        depth - 1
                    } else {
                        depth + 1
                    };

                    if new_depth >= 0
                        && !dmap.contains(&(*e, new_depth))
                        && !cur.contains(&(*e, new_depth))
                    {
                        cur.push((*e, new_depth));
                    }
                }
            }
        }

        if cur.is_empty() {
            break;
        }

        for t in &cur {
            dmap.insert(*t);
        }
        last = cur;
    }
    None
}

#[derive(Debug, Clone, Copy)]
struct Portal {
    a: Pos,
    b: Pos,
    name: (char, char),
}

fn add(a: Pos, b: Pos) -> Pos {
    (a.0 + b.0, a.1 + b.1)
}

fn find_portals(tilemap: &TileMap) -> PortalMap {
    let passages = tilemap
        .iter()
        .filter(|(_, &v)| v == Tile::Passage)
        .map(|p| p.0);

    let pairs = [
        ((0, 1), (0, 2)),
        ((0, -2), (0, -1)),
        ((1, 0), (2, 0)),
        ((-2, 0), (-1, 0)),
    ];

    let mut pmap: PortalMap = HashMap::new();

    let portals = passages.flat_map(|&p| {
        pairs.iter().cloned().filter_map(move |(a, b)| {
            let a = add(p, a);
            let b = add(p, b);
            let a = *tilemap.get(&a).unwrap_or(&Tile::Empty);
            let b = *tilemap.get(&b).unwrap_or(&Tile::Empty);
            match (a, b) {
                (Tile::P(an), Tile::P(bn)) => Some((p, an, bn)),
                _ => None,
            }
        })
    });

    for (p, a, b) in portals {
        let id = (a, b);
        let v = pmap.entry(id).or_insert_with(Vec::new);
        v.push(p);
    }

    pmap
}

fn main() {
    let rdr = BufReader::new(File::open("input").unwrap());
    let mut tilemap: TileMap = HashMap::new();

    for (y, line) in rdr.lines().enumerate() {
        let line = line.unwrap();
        for (x, c) in line.chars().enumerate() {
            let (x, y) = (x as i32, y as i32);
            tilemap.insert((x, y), c.try_into().unwrap());
        }
    }

    let pmap = find_portals(&tilemap);
    let begin = pmap[&('A', 'A')][0];
    let end = pmap[&('Z', 'Z')][0];

    let p1 = find(&tilemap, &pmap, begin, end).expect("P2 no path found");
    println!("Part 1: {}", p1);
    let p2 = find_depth(&tilemap, &pmap, begin, end).expect("P2 no path found");
    println!("Part 2: {}", p2);
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
            let t = tilemap.get(&(x, y)).unwrap_or(&Tile::Passage);

            let c = match t {
                Tile::Passage => '.',
                Tile::Wall => '#',
                Tile::Empty => ' ',
                Tile::P(p) => *p,
            };

            print!("{}", c);
        }
        println!();
    }
}
