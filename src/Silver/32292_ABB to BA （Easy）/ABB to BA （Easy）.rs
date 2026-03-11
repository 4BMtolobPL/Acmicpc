use std::collections::VecDeque;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut lines = buf.lines();

    let test_cases: usize = lines.next().unwrap().parse().unwrap();

    let mut out = String::new();
    for _ in 0..test_cases {
        let _n = lines.next().unwrap();
        let s = lines.next().unwrap();

        let mut stack = Vec::new();
        let mut queue = VecDeque::from_iter(s.chars());
        while let Some(ch) = queue.pop_front() {
            if ch == 'B' && stack.len() >= 2 {
                if stack[stack.len() - 2] == 'A' && stack[stack.len() - 1] == 'B' {
                    stack.pop();
                    stack.pop();
                    queue.push_front('A');
                    queue.push_front('B');
                } else {
                    stack.push(ch);
                }
            } else {
                stack.push(ch)
            }
        }

        writeln!(&mut out, "{}", stack.iter().collect::<String>()).unwrap();
    }

    print!("{out}");
}
