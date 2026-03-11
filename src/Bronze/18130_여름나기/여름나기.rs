use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut lines = buf
        .lines()
        .map(|line| line.split_ascii_whitespace().map(|x| x.parse().unwrap()));

    let (_n, q): (usize, usize) = {
        let mut iter = lines.next().unwrap();
        (iter.next().unwrap(), iter.next().unwrap())
    };

    let v: Vec<(usize, _, _)> = lines
        .map(|mut line| {
            (
                line.next().unwrap(),
                line.next().unwrap(),
                line.next().unwrap(),
            )
        })
        .collect();

    let mut index = 0;
    let mut min = usize::MAX;

    for (i, val) in v.iter().enumerate() {
        let new = get_price(q, *val);
        if new < min {
            index = i;
            min = new;
        }
    }

    println!("{} {}", index + 1, min);
}

fn get_price(time: usize, fan: (usize, usize, usize)) -> usize {
    let mut sum = fan.0;

    let mut x = time / fan.1;
    if time % fan.1 == 0 {
        x -= 1;
    }

    sum += (fan.2 * (x + 1)) * x / 2;

    sum
}
