use std::iter::repeat;

fn main() {
    let base = [0, 1, 0, -1];
    let txt = std::fs::read_to_string("input").unwrap();
    let txt = txt.trim();
    let v: Vec<_> = txt
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i16)
        .collect();

    let mut cur = v.clone();
    for _ in 0..100 {
        let mut ph = vec![];
        for p in 1..=cur.len() {
            let iter = base.iter().flat_map(|e| repeat(e).take(p)).cycle().skip(1);
            let x = cur.iter().zip(iter).map(|(a, b)| (a * b)).sum::<i16>() % 10;
            ph.push(x.abs());
        }
        cur = ph;
    }

    print!("Part 1: ");
    for c in &cur[0..8] {
        print!("{}", c);
    }
    println!();

    let v: Vec<_> = txt
        .chars()
        .cycle()
        .take(txt.len() * 10000)
        .map(|c| c.to_digit(10).unwrap() as i16)
        .collect();

    let offset: usize = txt[0..7].parse().unwrap();
    let mut lut = vec![vec![-1; v.len()]; 101];

    // Prefill the LUT so we don't blow the stack
    for x in (offset..v.len()).rev() {
        for y in 0..=100 {
            get_at((x, y), &v, &mut lut);
        }
    }

    print!("Part 2: ");
    for off in offset..offset + 8 {
        let v = get_at((off, 100), &v, &mut lut);
        print!("{}", v);
    }
    println!();
}

fn get_at(p: (usize, usize), v: &[i16], lut: &mut [Vec<i16>]) -> i16 {
    if lut[p.1][p.0] != -1 {
        return lut[p.1][p.0];
    }

    let val = if p.1 == 0 || p.0 == v.len() - 1 {
        v[p.0]
    } else {
        let above = (p.0, p.1 - 1);
        let right = (p.0 + 1, p.1);
        (get_at(above, v, lut) + get_at(right, v, lut)) % 10
    };

    lut[p.1][p.0] = val;
    val
}
