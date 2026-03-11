use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let lines = buf.lines();
    let names: Vec<&str> = lines.skip(1).collect();

    let mut sum = 0;

    for i in 0..names.len() {
        'o: for j in i + 1..names.len() {
            for l in 1..=names[i].len().min(names[j].len()) {
                if names[j].ends_with(names[i].chars().take(l).collect::<String>().as_str())
                    || names[i]
                        .ends_with(names[j].chars().take(l).collect::<String>().as_str())
                {
                    sum += 1;
                    continue 'o;
                }
            }
        }
    }

    println!("{}", sum);
}
