use std::io::stdin;

pub fn solve() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let mut n: i32 = buf.trim().parse().unwrap();
    
    if n.count_ones() > 1 {
        let s = format!("{:b}", n);
        let (_, a) = s.trim_start_matches('0').split_at(1);
        n = i32::from_str_radix(a, 2).unwrap() * 2;
    }

    println!("{}", n);
}