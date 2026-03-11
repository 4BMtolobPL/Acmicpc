use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let n: i32 = buf.trim().parse().unwrap();

    for _ in 0..n {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        let s: Vec<&str> = buf.trim().split('X').collect();

        let mut score = 0;
        for i in s {
            score += (1..=(i.len() as i32)).sum::<i32>();
        }
    
        println!("{}", score);
    }
}