use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
    let n: i32 = iter.next().unwrap();
    let l = iter.next().unwrap();

    println!("{}", get_list(n, l));
}

fn get_list(n: i32, l: i32) -> String {
    let calc_1 = n * 2;
    for len in l..=100 {
        if calc_1 % len != 0 {
            continue;
        }

        let calc_2 = calc_1 / len - len + 1;
        if calc_2 < 0 {
            break;
        } else if calc_2 % 2 != 0 {
            continue;
        }

        let a = calc_2 / 2;

        return (a..(a + len))
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
    }

    "-1".to_string()
}
