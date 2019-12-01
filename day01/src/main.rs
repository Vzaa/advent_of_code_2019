use std::fs::File;
use std::io::{BufRead, BufReader};

fn p1() {
    let rdr = BufReader::new(File::open("input").unwrap());
    let mass_iter = rdr.lines().map(|l| l.unwrap().parse::<i32>().unwrap());
    let fuel_sum: i32 = mass_iter.map(|m| m / 3 - 2).sum();

    println!("{:}", fuel_sum);
}

fn p2_fuel(m: i32) -> i32 {
    let f = m / 3 - 2;
    if f > 0 {
        f + p2_fuel(f)
    } else {
        0
    }
}

fn p2() {
    let rdr = BufReader::new(File::open("input").unwrap());
    let mass_iter = rdr.lines().map(|l| l.unwrap().parse::<i32>().unwrap());
    let fuel_sum: i32 = mass_iter.map(|m| p2_fuel(m)).sum();

    println!("{:}", fuel_sum);
}

fn main() {
    p1();
    p2();
}
