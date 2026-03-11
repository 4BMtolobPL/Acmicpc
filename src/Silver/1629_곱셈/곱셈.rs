use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    let mut iter = buf.split_whitespace().map(|x| x.parse().unwrap());
    let (a, b, c): (i64, _, _) = (
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    );

    let mut x = b;
    let mut count = 0;
    while x.count_ones() > 0 {
        x >>= 1;
        count += 1;
    }

    let mut v = vec![a % c];
    for _ in 0..count {
        let x = *v.last().unwrap();
        v.push(((x % c) * (x % c)) % c);
    }

    let mut x = b;
    let mut sum = 1;
    for value in v {
        if x % 2 == 1 {
            sum = ((sum % c) * (value % c)) % c;
        }

        x >>= 1;
    }

    println!("{}", sum);
}