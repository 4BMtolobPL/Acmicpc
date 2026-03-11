use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    let mut out = String::new();
    for (case_index, line) in buf.lines().enumerate().skip(1) {
        let mut stack = Vec::new();
        let mut score = 0;
        for c in line.chars() {
            if let Some(last) = stack.last() {
                if c == *last {
                    stack.pop();
                    score += 10;
                } else {
                    stack.push(c);
                }
            } else {
                stack.push(c);
            }
        }

        score += stack.len() / 2 * 5;

        writeln!(&mut out, "Case #{case_index}: {score}").unwrap();
    }

    print!("{out}");
}
