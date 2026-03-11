use std::io;

fn main() {
    let mut buf  = String::new();
    let mut v: Vec<i32> = Vec::new();
    for _ in 0..3 {
        io::stdin().read_line(&mut buf).unwrap();
        v.push(buf.trim().parse().unwrap());
        buf.clear();
    }
    let product = v[0] * v[1] * v[2];

    let mut arr: [i32; 10] = [0; 10];
    
    for i in product.to_string().chars() {
        arr[i.to_digit(10).unwrap() as usize] += 1;
    }

    for i in arr {
        println!("{}", i);
    }
}