use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut lines = buf.lines();

    let mut out: Vec<String> = Vec::new();
    loop {
        let m = lines.next().unwrap().parse().unwrap();

        if m == 0 {
            break;
        }

        let mut v: Vec<Vec<Option<Merge>>> = vec![vec![None; 9]; m];

        for i in 0..m {
            let mut line = lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|x| x.parse().unwrap());

            let n = line.next().unwrap();
            let mut index = 0;
            for _ in 0..n {
                while v[i][index].is_some() {
                    index += 1;
                }

                let x = line.next().unwrap();
                let y = line.next().unwrap();

                for k in 0..x {
                    for l in 0..y {
                        if k == 0 && l == 0 {
                            v[i + k][index + l] = Some(Merge::None);
                        } else if k == 0 {
                            v[i][index + l] = Some(Merge::Left);
                        } else if l == 0 {
                            v[i + k][index] = Some(Merge::Up);
                        } else {
                            v[i + k][index + l] = Some(Merge::UpLeft);
                        }
                    }
                }
            }
        }

        for (i, row) in v.iter().enumerate() {
            let mut s_1 = String::new();
            for col in row.iter() {
                if let Some(x) = col {
                    match x {
                        Merge::None | Merge::Left => write!(s_1, " --").unwrap(),
                        Merge::Up | Merge::UpLeft => write!(s_1, "   ").unwrap(),
                    }
                } else {
                    break;
                }
            }
            out.push(s_1.trim_end().to_string());

            let mut s_2 = String::new();
            for (j, col) in row.iter().enumerate() {
                if let Some(x) = col {
                    match x {
                        Merge::None => write!(s_2, "|{}{}", i + 1, j + 1).unwrap(),
                        Merge::Up => write!(s_2, "|  ").unwrap(),
                        Merge::Left | Merge::UpLeft => write!(s_2, "   ").unwrap(),
                    }
                } else {
                    break;
                }
            }
            write!(s_2, "|").unwrap();
            out.push(s_2);
        }

        let mut s_last = String::new();
        for i in v.last().unwrap() {
            if i.is_none() {
                break;
            }

            write!(s_last, " --").unwrap();
        }
        writeln!(s_last).unwrap();

        out.push(s_last.clone());
    }

    print!("{}", out.join("\n"));
}

#[derive(Clone, Debug)]
enum Merge {
    None,
    Up,
    Left,
    UpLeft,
}
