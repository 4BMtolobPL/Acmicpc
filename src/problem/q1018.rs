#[allow(dead_code)]
pub fn solve() {
    let v: Vec<i32> = get_line()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut board: Vec<String> = Vec::new();
    for _ in 0..v[0] {
        board.push(get_line());
    }

    board_check(board);
}

fn get_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn board_check(lines: Vec<String>) {
    let mut min = i32::MAX;
    for row in 0..(lines.len() - 7) {
        for char_index in 0..(lines[row].len() - 7) {
            let mut v: Vec<String> = Vec::new();
            for item in lines.iter().skip(row).take(8) {
                v.push(item.chars().skip(char_index).take(8).collect());
            }
            min = min.min(lines_check(v));
        }
    }
    println!("{}", min);
}

fn lines_check(lines: Vec<String>) -> i32 {
    let w = "WBWBWBWB";
    let b = "BWBWBWBW";
    let start_w = format!("{}{}{}{}{}{}{}{}", w, b, w, b, w, b, w, b);
    let start_b = format!("{}{}{}{}{}{}{}{}", b, w, b, w, b, w, b, w);

    let line = lines.join("");

    let mut counter_w = 0;
    for (c, r) in line.chars().zip(start_w.chars()) {
        if c != r {
            counter_w += 1;
        }
    }
    let mut counter_b = 0;
    for (c, r) in line.chars().zip(start_b.chars()) {
        if c != r {
            counter_b += 1;
        }
    }

    counter_w.min(counter_b)
}
