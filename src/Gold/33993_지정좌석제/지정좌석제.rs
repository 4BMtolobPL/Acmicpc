use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut lines = buf.lines();

    let (_n, r, c, w): (usize, _, _, _) = {
        let mut iter = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap());
        (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        )
    };
    let w_half = w / 2;

    let friends: Vec<(usize, usize)> = lines
        .map(|line| {
            let mut iter = line.split_ascii_whitespace().map(|x| x.parse().unwrap());

            let x: usize = iter.next().unwrap();
            let y: usize = iter.next().unwrap();

            (x, y)
        })
        .collect();

    let mut v: Vec<Vec<(i32, bool)>> = vec![vec![(0, false); c + 1]; r + 1];
    for (x, y) in friends.iter() {
        v[*x][*y].1 = true;

        let top_left = (
            x.saturating_sub(w_half).max(1),
            y.saturating_sub(w_half).max(1),
        );
        let bottom_right = (x + w_half + 1, y + w_half + 1);

        v[top_left.0][top_left.1].0 += 1;

        if bottom_right.1 <= c {
            v[top_left.0][bottom_right.1].0 -= 1;
        }
        if bottom_right.0 <= r {
            v[bottom_right.0][top_left.1].0 -= 1;
        }
        if bottom_right.0 <= r && bottom_right.1 <= c {
            v[bottom_right.0][bottom_right.1].0 += 1;
        }
    }

    let mut max = 0;
    let mut max_point = (0, 0);
    for i in 1..=r {
        let mut sum = 0;
        for j in 1..=c {
            sum += v[i][j].0;
            v[i][j].0 = sum + v[i - 1][j].0;

            if !v[i][j].1 && max < v[i][j].0 {
                max = v[i][j].0;
                max_point = (i, j);
            }
        }
    }

    println!("{max}\n{} {}", max_point.0, max_point.1);
}
