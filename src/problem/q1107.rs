use std::io::stdin;

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let target: usize = buf.trim().parse().unwrap();

    buf.clear();
    stdin().read_line(&mut buf).unwrap();
    let m: usize = buf.trim().parse().unwrap();

    if m == 0 {
        println!("{}", target.to_string().len().min(target.abs_diff(100)));
    } else if m == 10 {
        stdin().read_line(&mut buf).unwrap();
        println!("{}", target.abs_diff(100));
    } else {
        buf.clear();
        stdin().read_line(&mut buf).unwrap();

        if target == 100 {
            println!("0");
        } else {
            let mut buttons = [true; 10];
            for i in buf.split_whitespace().map(|x| x.parse::<usize>().unwrap()) {
                buttons[i] = false;
            }

            let mut min = target.abs_diff(100);
            for i in (0..target).rev() {
                if is_pressable(&buttons, i) {
                    min = min.min(i.abs_diff(target) + i.to_string().len());
                    break;
                }
            }
            for i in target..(target + target.abs_diff(100)) {
                if is_pressable(&buttons, i) {
                    min = min.min(i.abs_diff(target) + i.to_string().len());
                    break;
                }
            }

            println!("{}", min);
        }
    }
}

fn is_pressable(buttons: &[bool; 10], i: usize) -> bool {
    let mut a = i;
    while a > 9 {
        if !buttons[a % 10] {
            return false;
        }
        a /= 10;
    }
    buttons[a]
}
