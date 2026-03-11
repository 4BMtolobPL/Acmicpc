use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let store: Vec<char> = buf.trim().chars().collect();

    let mut r = 0;
    let mut b = 0;
    let mut g = 0;

    for i in 0..store.len() {
        match store.get(i).unwrap() {
            '@' => r = i,
            '#' => b = i,
            '!' => g = i,
            _ => continue,
        }
    }

    if r < b && b < g {
        println!("{}", g - r - 1);
    } else if g < b && b < r {
        println!("{}", r - g - 1);
    } else {
        println!("-1");
    }
}
