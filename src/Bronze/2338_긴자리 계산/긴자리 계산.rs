use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let v: Vec<&str> = buf.split_whitespace().collect();
    println!("{}", add(v[0], v[1]));
    println!("{}", sub(v[0], v[1]));
    println!("{}", mul(v[0], v[1]));
}


// lhs + rhs
fn add(lhs: &str, rhs: &str) -> String {
    if lhs == "0" {
        return rhs.to_string()
    } else if rhs == "0" {
        return lhs.to_string()
    }

    if lhs.starts_with('-') && !rhs.starts_with('-') {
        return sub(rhs, lhs.trim_start_matches('-'))
    } else if !lhs.starts_with('-') && rhs.starts_with('-') {
        return sub(lhs, rhs.trim_start_matches('-'))
    } else if lhs.starts_with('-') && rhs.starts_with('-') {
        return format!("-{}", add(lhs.trim_start_matches('-'), rhs.trim_start_matches('-')))
    }

    let (long, short) = if lhs.len() >= rhs.len() { (lhs, rhs) } else { (rhs, lhs) };

    let mut result = String::new();

    let mut floor = 0;

    for (x, y) in short.chars().rev().zip(long.chars().rev()) {
        let t = x.to_digit(10).unwrap() + y.to_digit(10).unwrap() + floor;
        floor  = t / 10;
        result.push_str(&(t % 10).to_string());
    }
    for c in long.chars().rev().skip(short.len()) {
        let t = c.to_digit(10).unwrap() + floor;
        floor  = t / 10;
        result.push_str(&(t % 10).to_string());
    }
    if floor == 1 {
        result.push('1');
    }

    result.chars().rev().collect()
}

// lhs - rhs
fn sub(lhs: &str, rhs: &str) -> String {
    if lhs == "0" && rhs == "0" {
        return "0".to_string()
    } else if lhs == "0" {
        if rhs.starts_with('-') {
            return rhs.trim_start_matches('-').to_string();
        } else {
            return format!("-{}", rhs)
        }
    } else if rhs == "0" {
        return lhs.to_string()
    }

    if lhs.starts_with('-') && !rhs.starts_with('-') {
        return add(lhs, &("-".to_string() + rhs))
    } else if !lhs.starts_with('-') && rhs.starts_with('-') {
        return add(lhs, rhs.trim_start_matches('-'))
    } else if lhs.starts_with('-') && rhs.starts_with('-') {
        return sub(rhs.trim_start_matches('-'), lhs.trim_start_matches('-'))
    }

    if lhs.len() < rhs.len() {
        return "-".to_string() + &sub(rhs, lhs)
    } else if lhs.len() == rhs.len() {
        let mut left = lhs.chars();
        let mut right = rhs.chars();
        loop {
            let x = left.next();
            let y = right.next();
            if x == None {
                break;
            }
            if y == None {
                break;
            }
            let v = x.unwrap().to_digit(10).unwrap();
            let w = y.unwrap().to_digit(10).unwrap();
            if v > w {
                break;
            } else if v < w {
                return "-".to_string() + &sub(rhs, lhs)
            } 
        }
    }

    let mut result = String::new();

    let mut floor = 0;

    for (x, y) in lhs.chars().rev().zip(rhs.chars().rev()) {
        let mut t = x.to_digit(10).unwrap() as i32 - y.to_digit(10).unwrap() as i32 - floor;
        floor  = if t < 0 { t += 10; 1 } else { 0 };
        result.push_str(&t.to_string());
    }
    for c in lhs.chars().rev().skip(rhs.len()) {
        let mut t = c.to_digit(10).unwrap() as i32 - floor;
        floor = if t < 0 { t += 10; 1 } else { 0 };
        result.push_str(&t.to_string())
    }


    result = result.trim_end_matches("0").to_string();
    if result.is_empty() {
        result.push('0');
    }
    result.chars().rev().collect()
}

fn mul(lhs: &str, rhs: &str) -> String {
    if lhs == "0" || rhs == "0" {
        return "0".to_string()
    }

    if lhs.starts_with('-') && !rhs.starts_with('-') {
        return format!("-{}", mul(lhs.trim_start_matches('-'), rhs))
    } else if !lhs.starts_with('-') && rhs.starts_with('-') {
        return format!("-{}", mul(lhs, rhs.trim_start_matches('-')))
    } else if lhs.starts_with('-') && rhs.starts_with('-') {
        return mul(lhs.trim_start_matches('-'), rhs.trim_start_matches('-'))
    }

    let mut result = String::from("0");

    for (rhs_index, rhs_value) in rhs.char_indices().rev() {
        if rhs_value == '0' {
            continue;
        }
        let mut tmp = String::from("0");
        for _ in 0..rhs_value.to_digit(10).unwrap() {
            tmp = add(lhs, &tmp);
        }
        for _ in 0..(rhs.len() - rhs_index - 1) {
            tmp.push('0');
        }
        result = add(&result, &tmp);
    }

    return result;
}