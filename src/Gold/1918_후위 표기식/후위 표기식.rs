use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let mut stack: Vec<String> = Vec::new();

    for c in buf.trim_end().chars() {
        if c.is_alphabetic() {
            if let Some(last) = stack.pop() {
                if last == "*" || last == "/" {
                    let left = stack.pop().unwrap();
                    stack.push(format!("{}{}{}", left, c, last));
                } else {
                    stack.push(last);
                    stack.push(c.to_string());
                }
            } else {
                stack.push(c.to_string())
            }
        } else {
            match c {
                ')' => {
                    let mut out = String::new();

                    while let Some(right) = stack.pop() {
                        let operator = stack.pop().unwrap();

                        if operator == "(" {
                            out = format!("{}{}", right, out);
                            break;
                        } else {
                            out = format!("{}{}{}", right, operator, out);
                        }
                    }

                    if let Some(last) = stack.pop() {
                        if last == "*" || last == "/" {
                            let left = stack.pop().unwrap();
                            out = format!("{}{}{}", left, out, last);
                        } else {
                            stack.push(last);
                        }
                    }

                    stack.push(out)
                }
                _ => stack.push(c.to_string()),
            }
        }
    }

    let mut out = String::new();
    while let Some(right) = stack.pop() {
        if let Some(operator) = stack.pop() {
            out = format!("{}{}{}", right, operator, out);
        } else {
            out = format!("{}{}", right, out);
        }
    }

    println!("{}", out);
}
