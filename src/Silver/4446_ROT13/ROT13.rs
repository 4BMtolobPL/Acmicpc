use std::collections::HashMap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    let vowel = ['a', 'i', 'y', 'e', 'o', 'u'];
    let consonant = [
        'b', 'k', 'x', 'z', 'n', 'h', 'd', 'c', 'w', 'g', 'p', 'v', 'j', 'q', 't', 's', 'r', 'l',
        'm', 'f',
    ];
    let mut vowel_map = HashMap::new();
    let mut consonant_map = HashMap::new();

    for (i, c) in vowel.iter().enumerate() {
        vowel_map.insert(c, i);
    }

    for (i, c) in consonant.iter().enumerate() {
        consonant_map.insert(c, i);
    }

    let mut out = String::new();
    for c in buf.trim_end().chars() {
        let is_uppercase = c.is_ascii_uppercase();

        let ch = if is_uppercase {
            c.to_ascii_lowercase()
        } else {
            c
        };
        
        if vowel_map.contains_key(&ch) {
            let index: usize = (*vowel_map.get(&ch).unwrap() + vowel.len() - 3) % vowel.len();
            out.push(if is_uppercase {
                vowel[index].to_ascii_uppercase()
            } else {
                vowel[index]
            });
        } else if consonant_map.contains_key(&ch) {
            let index: usize =
                (*consonant_map.get(&ch).unwrap() + consonant.len() - 10) % consonant.len();
            out.push(if is_uppercase {
                consonant[index].to_ascii_uppercase()
            } else {
                consonant[index]
            });
        } else {
            out.push(c);
        }
    }

    println!("{out}");
}
