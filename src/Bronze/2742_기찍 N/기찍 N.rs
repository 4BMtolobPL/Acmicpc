use std::io;

fn main() {
    let mut buf  = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let num: usize = buf.trim().parse().unwrap();
    let mut output = String::new();
    for i in (1..=num).rev() {
        output.push_str(&i.to_string());
        output.push('\n');
    }
    println!("{}", output);
}