// Had this fn lying somewhere, it's in reverse order from what we need here though
fn digits(n: i32) -> impl Iterator<Item = i32> {
    let mut state = n;

    std::iter::from_fn(move || {
        if state == 0 {
            None
        } else {
            let digit = state % 10;
            state /= 10;
            Some(digit)
        }
    })
}

// Do the check in reverse here
fn inc_check(n: i32) -> bool {
    let mut d_iter = digits(n).peekable();

    while let Some(d) = d_iter.next() {
        match (d, d_iter.peek()) {
            (d, Some(&d_next)) if d < d_next => return false,
            _ => (),
        }
    }
    true
}

fn adj_check(n: i32) -> bool {
    let mut d_iter = digits(n).peekable();

    while let Some(d) = d_iter.next() {
        match (d, d_iter.peek()) {
            (d, Some(&d_next)) if d == d_next => return true,
            _ => (),
        }
    }
    false
}

fn adj_check_2(n: i32) -> bool {
    let mut d_iter = digits(n).peekable();
    let mut streak = 0;

    while let Some(d) = d_iter.next() {
        match (d, d_iter.peek()) {
            (d, Some(&d_next)) if d == d_next => streak += 1,
            _ => {
                if streak == 1 {
                    return true;
                }
                streak = 0;
            }
        }
    }

    false
}

fn main() {
    let begin = 271973;
    let end = 785961;

    let cnt = (begin..end)
        .filter(|&n| inc_check(n) && adj_check(n))
        .count();

    println!("Part 1: {}", cnt);

    let cnt = (begin..end)
        .filter(|&n| inc_check(n) && adj_check_2(n))
        .count();

    println!("Part 2: {}", cnt);
}
