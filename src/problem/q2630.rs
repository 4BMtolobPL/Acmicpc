use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let n = lines.next().unwrap().parse().unwrap();

    let mut paper: Vec<Vec<u8>> = Vec::with_capacity(n);
    for line in lines {
        let row = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        paper.push(row)
    }

    let mut zero = 0;
    let mut one = 0;

    let mut stack = vec![((0, 0), n)];
    while !stack.is_empty() {
        let ((x, y), width) = stack.pop().unwrap();
        if is_paper(&paper, (x, y), width) {
            match paper[x][y] {
                0 => zero += 1,
                1 => one += 1,
                _ => {}
            }
        } else {
            let half = width / 2;
            stack.push(((x, y), half));
            stack.push(((x + half, y), half));
            stack.push(((x, y + half), half));
            stack.push(((x + half, y + half), half));
        }
    }

    println!("{}\n{}", zero, one);
}

fn is_paper(paper: &[Vec<u8>], left_top: (usize, usize), width: usize) -> bool {
    let (x, y) = left_top;
    let criterion = paper[x][y];
    for row in paper.iter().skip(x).take(width) {
        for value in row.iter().skip(y).take(width) {
            if *value != criterion {
                return false;
            }
        }
    }
    true
}
