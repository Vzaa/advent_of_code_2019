const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn main() {
    let dat = std::fs::read_to_string("input").unwrap();
    let dat = dat.trim();
    let dat: Vec<i32> = dat
        .chars()
        .map(|c| c.to_string().parse::<i32>().unwrap())
        .collect();

    let min_idx = dat
        .chunks(WIDTH * HEIGHT)
        .enumerate()
        .map(|(n, c)| (n, c.iter().filter(|&&p| p == 0).count()))
        .min_by_key(|(_, c)| *c)
        .unwrap();

    let layer = dat.chunks(WIDTH * HEIGHT).nth(min_idx.0).unwrap();
    let ones = layer.iter().filter(|&&v| v == 1).count();
    let twos = layer.iter().filter(|&&v| v == 2).count();
    println!("Part 1: {}", ones * twos);

    println!("Part 2:");
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let val = dat
                .chunks(WIDTH * HEIGHT) // Iterate over layers
                .map(|layer| layer[(y * WIDTH) + x]) // Iterate over (x, y) pixels of layers
                .find(|&p| p != 2) // Find first non-transparent
                .unwrap_or(2);
            // Other ways I did this:
            // Using fold, not short-circuting:
            // .fold(2, |acc, p| if acc == 2 { p } else { acc });
            // Using try_fold in an ugly way but it is short-circuting:
            // .try_fold(2, |acc, p| if acc == 2 { Ok(p) } else { Err(acc) });
            if val == 1 {
                print!("▓");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
