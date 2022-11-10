use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    println!("{}", get_min(buf.trim()));
}

fn get_min(line: &str) -> i32 {
    if line.contains('-') {
        let mut iter = line.split('-');
        let lhs = get_min(iter.next().unwrap());
        let rhs: Vec<&str> = iter.collect();
        let mut sum = 0;
        for i in rhs {
            sum += get_max(i);
        }
        lhs - sum
    } else if line.contains('+') {
        let (lhs, rhs) = line.split_once('+').unwrap();
        get_min(lhs) + get_min(rhs)
    } else {
        line.trim().parse().unwrap()
    }
}

fn get_max(line: &str) -> i32 {
    if line.contains('+') {
        let iter = line.split('+');
        let mut sum = 0;
        for i in iter {
            sum += i.trim().parse::<i32>().unwrap();
        }
        sum
    } else {
        line.trim().parse().unwrap()
    }
}
