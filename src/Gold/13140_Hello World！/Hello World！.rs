use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    let answer = check(n);
    if answer.is_ok() {
        let a = answer.unwrap();
        println!("{:7}\n+{:6}\n-------\n{:7}", a.0, a.1, n);
    } else {
        println!("No Answer");
    }
}

fn check(n: usize) -> Result<(usize, usize), String> {
    if n > 184010 {
        return Err("No Answer".to_string());
    }

    // h, w, e, o, l, r, d
    // h, w는 0이 될 수 없다.
    let mut stack = vec![1, 2, 0, 3, 4, 5, 6];
    let mut used: [bool; 10] = [
        true, true, true, true, true, true, true, false, false, false,
    ];
    'outer: while !stack.is_empty() {
        if stack.len() == 7 {
            let lhs = stack[0] * 10000 + stack[2] * 1000 + stack[4] * 110 + stack[3];
            let rhs =
                stack[1] * 10000 + stack[3] * 1000 + stack[5] * 100 + stack[4] * 10 + stack[6];
            if lhs + rhs == n {
                return Ok((lhs, rhs));
            }

            while let Some(last) = stack.pop() {
                used[last] = false;
                for (i, value) in used.iter_mut().enumerate().skip(last + 1) {
                    if !*value {
                        *value = true;
                        stack.push(i);
                        continue 'outer;
                    }
                }
            }
        } else {
            let range = if stack.len() < 2 { 1..10 } else { 0..10 };

            for i in range {
                if !used[i] {
                    used[i] = true;
                    stack.push(i);
                    continue 'outer;
                }
            }
        }
    }

    Err("No Answer".to_string())
}
