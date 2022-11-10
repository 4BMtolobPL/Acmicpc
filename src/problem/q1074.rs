use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace().map(|x| x.parse().unwrap());
    let n: usize = iter.next().unwrap();
    let r = iter.next().unwrap();
    let c = iter.next().unwrap();

    let mut width = 1;
    for _ in 0..n {
        width *= 2;
    }

    println!("{}", get_z(c, r, width) - 1);
}

fn get_z(x: usize, y: usize, width: usize) -> usize {
    if width == 1 {
        return 1;
    }
    let half = width / 2;
    if x < half && y < half {
        get_z(x, y, half)
    } else if x >= half && y < half {
        half * half + get_z(x - half, y, half)
    } else if x < half && y >= half {
        width * width / 2 + get_z(x, y - half, half)
    } else {
        (width * width / 4 * 3) + get_z(x - half, y - half, half)
    }
}
