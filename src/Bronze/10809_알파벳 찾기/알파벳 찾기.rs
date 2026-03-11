use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let s = buf.trim();

    let mut map = HashMap::new();
    for (index, c) in s.char_indices() {
        map.entry(c).or_insert(index);
    }

    let mut result = String::new();

    for i in 97..123 {
        match map.get(&char::from(i as u8)) {
            Some(x) => result.push_str(&x.to_string()),
            None => {
                result.push_str("-1");
        },
        }
        result.push(' ');
    }
    
    println!("{}", result.trim());
}