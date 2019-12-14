use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Reaction {
    input: HashMap<String, usize>,
    output: (String, usize),
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

impl FromStr for Reaction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split("=>");
        let ing = iter.next().ok_or(ParseError::Empty)?;
        let ing: String = ing.replace(",", " ");
        let out = iter.next().ok_or(ParseError::Empty)?;

        let mut input = HashMap::new();
        let mut it = ing.split_whitespace();
        while let Some(s) = it.next() {
            let cnt = s.parse()?;
            let name = it.next().ok_or(ParseError::Empty)?;
            input.insert(name.into(), cnt);
        }

        let mut it = out.split_whitespace();
        let cnt = it.next().ok_or(ParseError::Empty)?.parse::<usize>()?;
        let name = it.next().ok_or(ParseError::Empty)?;

        Ok(Reaction {
            input,
            output: (name.into(), cnt),
        })
    }
}

// return ORE consumed
fn produce(
    name: &str,
    amount: usize,
    rules: &HashMap<String, Reaction>,
    resources: &mut HashMap<String, usize>,
) -> usize {
    let r = &rules[name];
    let mut ore_cnt = 0;

    let mut produced = 0;

    if let Some(&o) = r.input.get("ORE") {
        let times = (amount + r.output.1 - 1) / r.output.1;
        ore_cnt += o * times;
        produced += r.output.1 * times;
    } else {
        let times = (amount + r.output.1 - 1) / r.output.1;
        for (n, &c) in &r.input {
            let need = c * times;
            let have = *resources.get(n).unwrap_or(&0);
            if have < need {
                ore_cnt += produce(&n, need - have, rules, resources);
            }

            // consume the available resources once they're ready
            let r = resources.get_mut(n).unwrap();
            *r -= need;
        }
        produced += r.output.1 * times;
    }

    let r = resources.entry(name.into()).or_insert(0);
    *r += produced;
    ore_cnt
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    let rules: HashMap<String, Reaction> = instr
        .lines()
        .map(|l| {
            let r: Reaction = l.parse().unwrap();
            (r.output.0.clone(), r)
        })
        .collect();

    let mut resources = HashMap::new();
    let p1 = produce("FUEL", 1, &rules, &mut resources);
    println!("Part 1: {}", p1);

    let tgt: usize = 1_000_000_000_000;
    let mut begin = 0;
    let mut end = 2 *(tgt / p1);

    loop {
        let mid = (end + begin) / 2;
        let ore = produce("FUEL", mid, &rules, &mut HashMap::new());
        let ore_n = produce("FUEL", mid + 1, &rules, &mut HashMap::new());

        if ore < tgt && ore_n > tgt {
            println!("Part 2: {}", mid);
            break;
        } else if ore_n > tgt {
            end = mid - 1;
        } else if ore_n < tgt {
            begin = mid + 1;
        } else {
            panic!("NOPE");
        }
    }
}
