use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
    let a: i32 = iter.next().unwrap();
    let b = iter.next().unwrap();
    let c = iter.next().unwrap();

    if let Some(ans) = get_root(a, b, c) {
        if ans.0 == ans.1 {
            println!("둘다틀렸근");
        } else if is_pow_of_2(ans.0) && is_pow_of_2(ans.1) {
            println!("이수근");
        } else {
            println!("정수근");
        }
    } else {
        println!("둘다틀렸근");
    }
}

fn get_root(a: i32, b: i32, c: i32) -> Option<(i32, i32)> {
    let r = b * b - 4 * a * c;
    let root = (r as f64).sqrt();

    if root.is_nan() {
        return None;
    }
    if root.fract() != 0.0 {
        return None;
    }

    let mut ans = (-b + root.trunc() as i32, -b - root.trunc() as i32);
    if ans.0 % (a * 2) != 0 || ans.1 % (a * 2) != 0 {
        return None;
    }
    ans.0 /= a * 2;
    ans.1 /= a * 2;

    Some(ans)
}

fn is_pow_of_2(n: i32) -> bool {
    n > 1 && (n & (n - 1) == 0)
}
