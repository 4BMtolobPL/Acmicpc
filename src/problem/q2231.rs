use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let cipher = buf.trim().len();
    let n: usize = buf.trim().parse().unwrap();
    let lhs = if n < 19 { 1 } else { n - (cipher * 9) };
    let mut answer = 0;
    for i in lhs..n {
        if n == i + i
            .to_string()
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .sum::<u32>() as usize
        {
            answer = i;
            break;
        }
    }
    println!("{}", answer);
}
