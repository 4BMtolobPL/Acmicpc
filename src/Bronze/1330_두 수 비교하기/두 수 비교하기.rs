use std::io;

fn main() {
    let mut buf  = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let v: Vec<f64> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();

    if v[0] > v[1] { println!(">"); }
    else if v[0] < v[1] { println!("<"); }
    else { println!("=="); }
}