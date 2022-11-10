use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let n: usize = buf.lines().next().unwrap().parse().unwrap();

    let mut d: Vec<Vec<char>> = Vec::with_capacity(n);
    for line in buf.lines().skip(1) {
        d.push(line.chars().collect());
    }

    println!("{}", quad_tree(&d, (0, 0), n));
}

fn quad_tree(map: &[Vec<char>], left_top: (usize, usize), len: usize) -> String {
    let (x, y) = left_top;
    if is_flat(map, left_top, len) {
        map[x][y].to_string()
    } else {
        let half = len / 2;
        format!(
            "({}{}{}{})",
            quad_tree(map, (x, y), half),
            quad_tree(map, (x, y + half), half),
            quad_tree(map, (x + half, y), half),
            quad_tree(map, (x + half, y + half), half)
        )
    }
}

fn is_flat(map: &[Vec<char>], left_top: (usize, usize), len: usize) -> bool {
    let (x, y) = left_top;
    let standard = map[x][y];
    for i in map.iter().skip(x).take(len) {
        for j in i.iter().skip(y).take(len) {
            if standard != *j {
                return false;
            }
        }
    }
    true
}
