use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut lines = buf.lines();
    let n = lines.next().unwrap().parse().unwrap();

    let mut v: Vec<Vec<char>> = Vec::with_capacity(n);
    for line in lines {
        v.push(line.chars().collect());
    }
    let mut v2 = v.clone();
    for i in v2.iter_mut() {
        for j in i.iter_mut() {
            if *j == 'G' {
                *j = 'R';
            }
        }
    }

    let mut visited_a = vec![vec![false; n]; n];
    let mut visited_b = vec![vec![false; n]; n];
    let mut stack_a = Vec::new();
    let mut stack_b = Vec::new();

    let mut counter_a = 0;
    let mut counter_b = 0;
    for i in 0..n {
        for j in 0..n {
            if !visited_a[i][j] {
                let now = v[i][j];
                counter_a += 1;
                stack_a.push((i, j));
                while let Some((x, y)) = stack_a.pop() {
                    if x > 0 && !visited_a[x - 1][y] && v[x - 1][y] == now {
                        visited_a[x - 1][y] = true;
                        stack_a.push((x - 1, y))
                    }
                    if x < n - 1 && !visited_a[x + 1][y] && v[x + 1][y] == now {
                        visited_a[x + 1][y] = true;
                        stack_a.push((x + 1, y))
                    }
                    if y > 0 && !visited_a[x][y - 1] && v[x][y - 1] == now {
                        visited_a[x][y - 1] = true;
                        stack_a.push((x, y - 1))
                    }
                    if y < n - 1 && !visited_a[x][y + 1] && v[x][y + 1] == now {
                        visited_a[x][y + 1] = true;
                        stack_a.push((x, y + 1))
                    }
                }
            }
            if !visited_b[i][j] {
                let now = v2[i][j];
                counter_b += 1;
                stack_b.push((i, j));
                while let Some((x, y)) = stack_b.pop() {
                    if x > 0 && !visited_b[x - 1][y] && v2[x - 1][y] == now {
                        visited_b[x - 1][y] = true;
                        stack_b.push((x - 1, y))
                    }
                    if x < n - 1 && !visited_b[x + 1][y] && v2[x + 1][y] == now {
                        visited_b[x + 1][y] = true;
                        stack_b.push((x + 1, y))
                    }
                    if y > 0 && !visited_b[x][y - 1] && v2[x][y - 1] == now {
                        visited_b[x][y - 1] = true;
                        stack_b.push((x, y - 1))
                    }
                    if y < n - 1 && !visited_b[x][y + 1] && v2[x][y + 1] == now {
                        visited_b[x][y + 1] = true;
                        stack_b.push((x, y + 1))
                    }
                }
            }
        }
    }

    println!("{} {}", counter_a, counter_b);
}
