use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug)]
struct Edge {
    u: String,
    v: String,
}

#[derive(Debug)]
enum ParseEdgeError {
    Empty,
}

type OrbitMap = HashMap<String, Vec<Edge>>;
type Reachables = HashMap<String, HashSet<String>>;
type Dists = HashMap<String, usize>;

impl FromStr for Edge {
    type Err = ParseEdgeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(')');
        let u = it.next().ok_or(ParseEdgeError::Empty)?.to_owned();
        let v = it.next().ok_or(ParseEdgeError::Empty)?.to_owned();
        Ok(Edge { u, v })
    }
}

fn calc_dists_from(pos: &str, orbit_map: &OrbitMap, dists: &mut Dists, dist: usize) {
    dists.insert(pos.into(), dist);
    for e in &orbit_map[pos] {
        calc_dists_from(&e.v, orbit_map, dists, dist + 1);
    }
}

fn calc_reachables_from(pos: &str, orbit_map: &OrbitMap, reachables: &mut Reachables) {
    reachables.insert(pos.to_string(), HashSet::new());

    for e in &orbit_map[pos] {
        reachables.get_mut(pos).unwrap().insert(e.v.clone());
        calc_reachables_from(&e.v, orbit_map, reachables);
        let child: Vec<_> = reachables[&e.v].iter().cloned().collect();
        for c in child {
            reachables.get_mut(pos).unwrap().insert(c.clone());
        }
    }
}

fn main() {
    let rdr = BufReader::new(File::open("input").unwrap());
    let edges: Vec<Edge> = rdr.lines().map(|l| l.unwrap().parse().unwrap()).collect();

    let mut orbit_map = HashMap::new();
    for e in edges {
        let key = &e.u;
        orbit_map.entry(e.v.clone()).or_insert_with(Vec::new);
        let es = orbit_map.entry(key.clone()).or_insert_with(Vec::new);
        es.push(e);
    }

    let mut dists: HashMap<String, usize> = HashMap::new();
    calc_dists_from("COM", &orbit_map, &mut dists, 0);

    let sum: usize = dists.values().sum();
    println!("Part 1: {}", sum);

    let mut reachables: HashMap<String, HashSet<String>> = HashMap::new();
    calc_reachables_from("COM", &orbit_map, &mut reachables);

    // Get furthest from "COM" that has both SAN and YOU reachable
    let sanyou = reachables
        .iter()
        .filter(|(_, v)| v.contains("SAN") && v.contains("YOU"))
        .max_by_key(|(k, _)| dists[*k])
        .unwrap().0;

    let dist_sanyou = dists[sanyou];
    let dist_san = dists["SAN"];
    let dist_you = dists["YOU"];
    let hops = (dist_san - dist_sanyou) + (dist_you - dist_sanyou) - 2;
    println!("Part 2: {}", hops);
}
