use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let n = lines.next().unwrap().parse::<usize>().unwrap();

    let paper: Vec<Vec<i32>> = lines
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();

    let (white, black) = get_sum_color(&paper, n, 0, 0);
    println!("{}\n{}", white, black);
}

fn get_sum_color(
    paper: &Vec<Vec<i32>>,
    size: usize,
    row_index: usize,
    column_index: usize,
) -> (i32, i32) {
    if check_same_color(paper, size, row_index, column_index) {
        if paper[row_index][column_index] == 0 {
            (1, 0)
        } else {
            (0, 1)
        }
    } else {
        let half_size = size / 2;

        let (white1, black1) = get_sum_color(paper, half_size, row_index, column_index);
        let (white2, black2) = get_sum_color(paper, half_size, row_index + half_size, column_index);
        let (white3, black3) = get_sum_color(paper, half_size, row_index, column_index + half_size);
        let (white4, black4) = get_sum_color(
            paper,
            half_size,
            row_index + half_size,
            column_index + half_size,
        );

        (
            white1 + white2 + white3 + white4,
            black1 + black2 + black3 + black4,
        )
    }
}

fn check_same_color(
    paper: &[Vec<i32>],
    size: usize,
    row_index: usize,
    column_index: usize,
) -> bool {
    let color = paper[row_index][column_index];
    for row in paper.iter().skip(row_index).take(size) {
        for value in row.iter().skip(column_index).take(size) {
            if *value != color {
                return false;
            }
        }
    }

    true
}
