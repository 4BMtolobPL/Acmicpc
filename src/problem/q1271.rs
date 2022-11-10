use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace().map(|x| x.to_string());
    let mut n = iter.next().unwrap();
    let m = iter.next().unwrap();

    let mut out = String::new();
    for i in (0..(n.len() + 1 - m.len())).rev() {
        let mut x = m.to_string();
        for _ in 0..i {
            x.push('0');
        }

        let mut count = 0;
        while cmp(&n, &x) != std::cmp::Ordering::Less {
            count += 1;
            n = sub(&n, &x);
        }
        out.push(char::from_digit(count, 10).unwrap());
    }

    println!("{}\n{}", out.trim_start_matches('0'), n);
}

fn cmp(lhs: &str, rhs: &str) -> std::cmp::Ordering {
    if lhs.len() != rhs.len() {
        lhs.len().cmp(&rhs.len())
    } else {
        for (a, b) in lhs.as_bytes().iter().zip(rhs.as_bytes()) {
            match a.cmp(b) {
                std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                std::cmp::Ordering::Equal => continue,
                std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
            }
        }
        std::cmp::Ordering::Equal
    }
}

fn sub(lhs: &str, rhs: &str) -> String {
    let mut out = String::new();
    let mut carry = 0;
    for (a, b) in lhs.chars().rev().zip(rhs.chars().rev()) {
        let a = a.to_digit(10).unwrap();
        let b = b.to_digit(10).unwrap() + carry;
        let x = if a >= b {
            carry = 0;
            a - b
        } else {
            carry = 1;
            a + 10 - b
        };
        out.push(char::from_digit(x as u32, 10).unwrap());
    }

    for x in lhs.chars().rev().skip(rhs.len()) {
        let x = x.to_digit(10).unwrap();
        if x >= carry {
            out.push(char::from_digit(x - carry, 10).unwrap());
            carry = 0;
        } else {
            out.push(char::from_digit(x + 10 - carry, 10).unwrap());
            carry = 1;
        }
    }

    out = out.chars().rev().collect();
    out = out.trim_start_matches('0').to_string();
    if out.is_empty() {
        out.push('0');
    }
    out
}
