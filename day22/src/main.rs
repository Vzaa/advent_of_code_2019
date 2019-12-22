use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
enum Action {
    DealIntoStack,
    Cut(i32),
    DealWithInc(usize),
}

#[derive(Debug)]
enum ParseActionError {
    Int(ParseIntError),
    Empty,
}

impl From<ParseIntError> for ParseActionError {
    fn from(e: ParseIntError) -> Self {
        ParseActionError::Int(e)
    }
}

impl FromStr for Action {
    type Err = ParseActionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("deal into new stack") {
            Ok(Action::DealIntoStack)
        } else if s.contains("cut") {
            let n = s
                .split_whitespace()
                .nth(1)
                .ok_or(ParseActionError::Empty)?
                .parse()?;
            Ok(Action::Cut(n))
        } else if s.contains("deal with increment") {
            let n = s
                .split_whitespace()
                .nth(3)
                .ok_or(ParseActionError::Empty)?
                .parse()?;
            Ok(Action::DealWithInc(n))
        } else {
            Err(ParseActionError::Empty)
        }
    }
}

fn deal_into_stack(deck: &mut [i64]) {
    deck.reverse();
}

fn cut(deck: &mut Vec<i64>, n: i32) {
    if n > 0 {
        for _ in 0..n {
            let p = deck.pop().unwrap();
            deck.insert(0, p);
        }
    } else if n < 0 {
        deck.reverse();
        for _ in 0..n.abs() {
            let p = deck.pop().unwrap();
            deck.insert(0, p);
        }
        deck.reverse();
    }
}

fn deal_with_inc(deck: &mut Vec<i64>, n: usize) {
    let size = deck.len();
    let mut space = vec![None; size];

    for idx in (0..).map(|i| i * n) {
        if let Some(card) = deck.pop() {
            space[idx % size] = Some(card);
        } else {
            break;
        }
    }
    let space = space.iter().map(|s| s.unwrap()).collect::<Vec<i64>>();

    deck.extend_from_slice(&space);
    deck.reverse();
}

fn deal_into_stack_revi(deck_len: usize, i: usize) -> usize {
    deck_len - i - 1
}

fn cut_revi(deck_len: usize, n: i32, i: usize) -> usize {
    if n > 0 {
        let n = n as usize;
        (i + n) % deck_len
    } else if n < 0 {
        let n = n.abs() as usize;
        (i + deck_len - n) % deck_len
    } else {
        i
    }
}

fn deal_with_inc_revi(deck_len: usize, n: usize, i: usize) -> usize {
    let deck_len = deck_len as isize;
    let n = n as isize;
    let i = i as isize;

    let rem = deck_len % n;

    let remi = i % n;
    let divi = i / n;

    if remi == 0 {
        return (i / n) as usize;
    }

    for (e, v) in (1..n)
        .map(|i| i * rem)
        .map(|v| (n - v).rem_euclid(n))
        .enumerate()
    {
        if v == remi {
            let e = (e + 1) as isize;
            return (((e * deck_len) / n) + (divi + 1)) as usize;
        }
    }

    unreachable!();
}

fn deal_into_stack_i(deck_len: usize, i: usize) -> usize {
    deck_len - i - 1
}

fn cut_i(deck_len: usize, n: i32, i: usize) -> usize {
    if n > 0 {
        let n = n as usize;
        (i + (deck_len - n)) % deck_len
    } else if n < 0 {
        let n = n.abs() as usize;
        (i + n) % deck_len
    } else {
        i
    }
}

fn deal_with_inc_i(deck_len: usize, n: usize, i: usize) -> usize {
    (i * n) % deck_len
}

#[allow(dead_code)]
fn once(mut idx: usize, actions: &[Action]) -> usize {

    for a in actions.iter().rev() {
        idx = match a {
            Action::DealIntoStack => deal_into_stack_revi(SIZE, idx),
            Action::Cut(n) => cut_revi(SIZE, *n, idx),
            Action::DealWithInc(n) => deal_with_inc_revi(SIZE, *n, idx),
        };
    }

    idx
}

#[allow(dead_code)]
fn once_i(mut idx: usize, actions: &[Action], size: usize) -> usize {
    for a in actions.iter() {
        idx = match a {
            Action::DealIntoStack => deal_into_stack_i(size, idx),
            Action::Cut(n) => cut_i(size, *n, idx),
            Action::DealWithInc(n) => deal_with_inc_i(size, *n, idx),
        };
    }

    idx
}

const SIZE: usize = 119315717514047;

fn main() {
    let txt = std::fs::read_to_string("input").unwrap();
    let actions = txt
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<Action>>();
    if true {
        let mut deck = (0..10007).rev().collect::<Vec<_>>();

        for a in &actions {
            match a {
                Action::DealIntoStack => deal_into_stack(&mut deck),
                Action::Cut(n) => cut(&mut deck, *n),
                Action::DealWithInc(n) => deal_with_inc(&mut deck, *n),
            }
        }

        deck.reverse();
        let r = deck.iter().enumerate().find(|(_, &v)| v == 2019).unwrap();
        println!("Part 1: {}", r.0);
    }

    // Used this to find the constants for my input
    if false {
        let mut last = None;
        for s in 0..10 {
            let mut a = s;
            for _ in 0..1_000_000 {
                a = rev(a);
            }
            if let Some(l) = last {
                println!("{}->{}: {}", s - 1, s, a as isize - l as isize);
            } else {
                println!("{}", a);
            }
            last = Some(a);
        }
    }

    {
        let mut idx = 2020;
        let end = 101741582076661_usize;

        let mut rem = end;
        while rem > 0 {
            if rem > 1_000_000 {
                idx = rev_mil(idx);
                rem -= 1_000_000;
            } else {
                idx = rev(idx);
                rem -= 1;
            }
        }

        println!("Part 2: {}", idx);
    }
}

#[allow(dead_code)]
fn fwd(n: usize) -> usize {
    let n = n as u128;
    let og = 115619106397456_u128;
    let j = 36917093953130_u128;
    let s = SIZE as u128;

    ((og + n * j) % s) as usize
}

fn rev(n: usize) -> usize {
    let n = n as u128;
    let og: u128 = 85834995146770 as u128;
    let j: u128 = 80725416546647 as u128;
    let s = SIZE as u128;

    ((og + n * j) % s) as usize
}

fn rev_mil(n: usize) -> usize {
    let n = n as u128;
    let og: u128 = 115753313030480 as u128;
    let j: u128 = 63912695741104 as u128;
    let s = SIZE as u128;

    ((og + n * j) % s) as usize
}
