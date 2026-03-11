use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();

    let (n, m): (usize, usize) = {
        let mut first_line = lines.next().unwrap().split_whitespace();
        (
            first_line.next().unwrap().parse().unwrap(),
            first_line.next().unwrap().parse().unwrap(),
        )
    };
    let mut start_point = (0, 0);
    let map: Vec<Vec<State>> = lines
        .enumerate()
        .map(|(row_index, line)| {
            line.chars()
                .enumerate()
                .map(|(column_index, c)| match c {
                    'O' => State::O,
                    'X' => State::X,
                    'I' => {
                        start_point = (row_index, column_index);
                        State::I
                    }
                    'P' => State::P,
                    _ => panic!("Unknown char {}", c),
                })
                .collect()
        })
        .collect();
    let mut visited = vec![vec![false; m]; n];
    let mut stack = vec![start_point];
    let mut friends = 0;

    while let Some((row_index, column_index)) = stack.pop() {
        match map[row_index][column_index] {
            State::X => continue,
            State::P => friends += 1,
            _ => {}
        }

        let mut next_closer = |x: usize, y: usize| {
            if !visited[x][y] {
                visited[x][y] = true;
                stack.push((x, y));
            }
        };

        if row_index > 0 {
            next_closer(row_index - 1, column_index);
        }
        if row_index < n - 1 {
            next_closer(row_index + 1, column_index);
        }
        if column_index > 0 {
            next_closer(row_index, column_index - 1);
        }
        if column_index < m - 1 {
            next_closer(row_index, column_index + 1);
        }
    }

    if friends > 0 {
        println!("{}", friends);
    } else {
        println!("TT");
    }
}

enum State {
    O,
    X,
    I,
    P,
}
