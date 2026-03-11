use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    let mut iter = buf.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let n = iter.next().unwrap();
    let k = iter.next().unwrap();

    let mut max = 0;

    for i in 1..=k {
        let new_num = (n * i)
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
        max = max.max(new_num);
    }

    println!("{}", max);
}
