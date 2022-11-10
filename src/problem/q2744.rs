use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    println!(
        "{}",
        buf.trim()
            .chars()
            .map(|c| if c.is_uppercase() {
                c.to_lowercase().to_string()
            } else {
                c.to_uppercase().to_string()
            })
            .collect::<String>()
    );
}
