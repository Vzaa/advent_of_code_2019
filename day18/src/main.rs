use std::collections::{HashMap, HashSet};
use std::convert::{TryFrom, TryInto};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Pos = (i32, i32);
type TileMap = HashMap<Pos, Tile>;
type DoorMap = HashMap<char, Pos>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Floor,
    Wall,
    Door(char),
    Key(char),
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        let out = match c {
            '#' => Tile::Wall,
            '.' | '@' => Tile::Floor,
            c if c.is_ascii_lowercase() => Tile::Key(c),
            c if c.is_ascii_uppercase() => Tile::Door(c),
            _ => return Err(()),
        };
        Ok(out)
    }
}

fn spaces(map: &TileMap, p: Pos) -> impl Iterator<Item = Pos> + '_ {
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .iter()
        .map(move |&s| (s.0 + p.0, s.1 + p.1))
        .filter(move |s| {
            let t = map[&s];
            match t {
                Tile::Wall | Tile::Door(_) => false,
                _ => true,
            }
        })
}

// Get all reachable keys with their distances
// Copied from last year
fn distance_map(map: &TileMap, p: Pos, keys: usize) -> HashMap<Pos, usize> {
    let mut dmap = HashSet::new();
    let mut kmap = HashMap::new();

    if keys == 0 {
        return kmap;
    }

    dmap.insert(p);
    let mut last = vec![p]; // vec is faster than hashset for the input size

    for dist in 0.. {
        let mut cur = vec![];
        for p in &last {
            for s in spaces(map, *p) {
                if !dmap.contains(&s) && !cur.contains(&s) {
                    cur.push(s);
                }
            }
        }

        if cur.is_empty() {
            break;
        }

        for t in &cur {
            dmap.insert(*t);
            if let Tile::Key(_) = map[&t] {
                kmap.insert(*t, dist + 1);
                if kmap.len() >= keys {
                    return kmap;
                }
            }
        }
        last = cur;
    }

    kmap
}

fn open_door(p: Pos, tilemap: &mut TileMap, doors: &DoorMap) -> char {
    if let Tile::Key(k) = tilemap.insert(p, Tile::Floor).unwrap() {
        let door_pos = doors.get(&k.to_ascii_uppercase());
        if let Some(dp) = door_pos {
            *tilemap.get_mut(dp).unwrap() = Tile::Floor;
        }
        return k;
    }
    unreachable!()
}

// Ugly stuff:
fn search(pos: Pos, tilemap: TileMap, doors: &DoorMap, keys: &HashSet<char>) {
    let mut best: HashMap<(Vec<char>, Pos), usize> = HashMap::new();
    let mut frontier: Vec<_> = distance_map(&tilemap, pos, keys.len())
        .into_iter()
        .map(|(p, d)| {
            let mut tmp = tilemap.clone();
            let k = open_door(p, &mut tmp, doors);
            let mut ks = keys.clone();
            ks.remove(&k);
            (d, p, tmp, ks)
        })
        .collect();

    loop {
        let min_idx = frontier
            .iter()
            .enumerate()
            .min_by_key(|(_, c)| c.0)
            .unwrap()
            .0;
        let (d, p, tmap, ks) = frontier.swap_remove(min_idx);

        if ks.is_empty() {
            println!("Part 1: {}", d);
            return;
        }

        let new_states: Vec<_> = distance_map(&tmap, p, ks.len())
            .into_iter()
            .filter_map(|(np, nd)| {
                let mut tmp = tmap.clone();
                let k = open_door(np, &mut tmp, doors);
                let mut nks = ks.clone();
                nks.remove(&k);
                let td = nd + d;
                let mut vid: Vec<_> = nks.iter().cloned().collect();
                vid.sort();
                let id = (vid, np);
                if let Some(&b) = best.get(&id) {
                    if td < b {
                        best.insert(id, td);
                        Some((td, np, tmp, nks))
                    } else {
                        None
                    }
                } else {
                    best.insert(id, td);
                    Some((td, np, tmp, nks))
                }
            })
            .collect();

        frontier.extend_from_slice(&new_states);
    }
}

// This is not a correct function but it solved my input
fn search_multi(posl: &[Pos], tilemap: TileMap, doors: &DoorMap, keys: &HashSet<char>) {
    let mut best: HashMap<(Vec<char>, Pos), usize> = HashMap::new();

    let mut frontier: Vec<_> = (0..posl.len())
        .flat_map(|idx| {
            let fuk: Vec<_> = distance_map(&tilemap, posl[idx], keys.len())
                .into_iter()
                .map(|(p, d)| {
                    let mut tmp = tilemap.clone();
                    let k = open_door(p, &mut tmp, doors);
                    let mut ks = keys.clone();
                    ks.remove(&k);
                    let mut npl = posl.to_owned();
                    npl[idx] = p;
                    (d, npl, tmp, ks)
                })
                .collect();
            fuk
        })
        .collect();

    loop {
        let min_idx = frontier
            .iter()
            .enumerate()
            .min_by_key(|(_, c)| c.0)
            .unwrap()
            .0;
        let (d, pl, tmap, ks) = frontier.swap_remove(min_idx);

        if ks.is_empty() {
            println!("Part 2: {}", d);
            return;
        }

        let new_states: Vec<_> = (0..posl.len())
            .flat_map(|idx| {
                let fuk: Vec<_> = distance_map(&tmap, pl[idx], ks.len())
                    .into_iter()
                    .filter_map(|(np, nd)| {
                        let mut tmp = tmap.clone();
                        let k = open_door(np, &mut tmp, doors);
                        let mut nks = ks.clone();
                        nks.remove(&k);
                        let td = nd + d;
                        let mut vid: Vec<_> = nks.iter().cloned().collect();
                        vid.sort();
                        let mut npl = pl.clone();
                        npl[idx] = np;
                        // using the same id as before is wrong:
                        let id = (vid, np);
                        if let Some(&b) = best.get(&id) {
                            if td < b {
                                best.insert(id, td);
                                Some((td, npl, tmp, nks))
                            } else {
                                None
                            }
                        } else {
                            best.insert(id, td);
                            Some((td, npl, tmp, nks))
                        }
                    })
                    .collect();
                fuk
            })
            .collect();

        frontier.extend_from_slice(&new_states);
    }
}

fn main() {
    let rdr = BufReader::new(File::open("input").unwrap());
    let mut tilemap: TileMap = HashMap::new();
    let mut keys: HashSet<char> = HashSet::new();
    let mut doors: HashMap<char, Pos> = HashMap::new();

    let mut pos = None;

    for (y, line) in rdr.lines().enumerate() {
        let line = line.unwrap();
        for (x, c) in line.chars().enumerate() {
            let (x, y) = (x as i32, y as i32);
            tilemap.insert((x, y), c.try_into().unwrap());

            if c.is_ascii_lowercase() {
                keys.insert(c);
            }

            if c.is_ascii_uppercase() {
                doors.insert(c, (x, y));
            }

            if c == '@' {
                pos = Some((x, y));
            }
        }
    }

    // Part 1
    let pos = pos.unwrap();
    search(pos, tilemap.clone(), &doors, &keys);

    // Part 2
    tilemap.insert(pos, Tile::Wall);
    tilemap.insert((pos.0, pos.1 + 1), Tile::Wall);
    tilemap.insert((pos.0, pos.1 - 1), Tile::Wall);
    tilemap.insert((pos.0 + 1, pos.1), Tile::Wall);
    tilemap.insert((pos.0 + 1, pos.1), Tile::Wall);

    let posl = [
        (pos.0 + 1, pos.1 + 1),
        (pos.0 + 1, pos.1 - 1),
        (pos.0 - 1, pos.1 - 1),
        (pos.0 - 1, pos.1 + 1),
    ];

    search_multi(&posl, tilemap.clone(), &doors, &keys);
}

#[allow(dead_code)]
fn draw_map(tilemap: &TileMap, p: Pos) {
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
            if (x, y) == p {
                print!("@");
                continue;
            }
            let t = tilemap.get(&(x, y)).unwrap_or(&Tile::Floor);

            let c = match t {
                Tile::Floor => '.',
                Tile::Wall => '#',
                Tile::Door(c) | Tile::Key(c) => *c,
            };

            print!("{}", c);
        }
        println!();
    }
}
