// Had this fn lying from excerism, it's in reverse order from what we need here though
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
    let mut d_iter = digits(n);
    let mut d = d_iter.next().unwrap();

    for d_next in d_iter {
        if d < d_next {
            return false;
        }
        d = d_next;
    }
    true
}

fn adj_check(n: i32) -> bool {
    let mut d_iter = digits(n);
    let mut d = d_iter.next().unwrap();

    for d_next in d_iter {
        if d == d_next {
            return true;
        }
        d = d_next;
    }
    false
}

fn adj_check_2(n: i32) -> bool {
    let mut d_iter = digits(n);
    let mut d = d_iter.next().unwrap();
    let mut streak = 0;

    for d_next in d_iter {
        if d == d_next {
            streak += 1;
        } else {
            if streak == 1 {
                return true;
            }
            streak = 0;
        }
        d = d_next;
    }

    streak == 1
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
