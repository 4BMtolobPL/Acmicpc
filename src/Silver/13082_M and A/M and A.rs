use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut iter = buf.split_ascii_whitespace();

    let first_company: Vec<char> = iter.next().unwrap().chars().collect();
    let second_company: Vec<char> = iter.next().unwrap().chars().collect();

    if check_new_name(&first_company, &second_company, &first_company)
        || check_new_name(&second_company, &first_company, &first_company)
    {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn check_new_name(lhs: &Vec<char>, rhs: &Vec<char>, target: &[char]) -> bool {
    let mut l_index = 0;
    let mut r_index = 0;
    let mut is_lhs_turn = true;

    let check = |pointer: usize, target: &Vec<char>, check: char| -> Option<usize> {
        let mut index = pointer;
        if index >= target.len() {
            return None;
        }
        while index < target.len() {
            if target[index] == check {
                index += 1;
                return Some(index);
            } else {
                index += 1;
            }
        }
        None
    };
    'outer: for c in target.iter() {
        if is_lhs_turn {
            if let Some(b) = check(l_index, lhs, *c) {
                l_index = b;
                is_lhs_turn = !is_lhs_turn;
                continue 'outer;
            } else {
                return false;
            }
        } else if let Some(b) = check(r_index, rhs, *c) {
            r_index = b;
            is_lhs_turn = !is_lhs_turn;
            continue 'outer;
        } else {
            return false;
        }
    }

    true
}
