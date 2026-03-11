use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let lines = buf.lines();
    let v: Vec<_> = lines.skip(1).collect();
    let pledges = [
        "Never gonna give you up",
        "Never gonna let you down",
        "Never gonna run around and desert you",
        "Never gonna make you cry",
        "Never gonna say goodbye",
        "Never gonna tell a lie and hurt you",
        "Never gonna stop",
    ];

    println!(
        "{}",
        if v.iter().any(|x| !pledges.contains(x)) {
            "Yes"
        } else {
            "No"
        }
    );
}
