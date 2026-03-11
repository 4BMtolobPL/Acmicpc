use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());

    let s: i32 = iter.next().unwrap();
    let bread_a = Bread::new(iter.next().unwrap(), iter.next().unwrap());
    let bread_b = Bread::new(iter.next().unwrap(), iter.next().unwrap());

    let (better_bread, worse_bread) =
        if bread_a.profit * bread_b.cost > bread_b.profit * bread_a.cost {
            (bread_a, bread_b)
        } else {
            (bread_b, bread_a)
        };

    println!("{}", get_max(s, &better_bread, &worse_bread));
}

fn get_max(s: i32, better_bread: &Bread, worse_bread: &Bread) -> i32 {
    let mut max = 0;
    for amount_expensive in 0..=(s / worse_bread.cost).min(better_bread.cost) {
        let total_profit = get_total_profit(s, worse_bread, better_bread, amount_expensive);
        max = max.max(total_profit);
    }
    max
}

fn get_total_profit(s: i32, bread_a: &Bread, bread_b: &Bread, amount_a: i32) -> i32 {
    let left = s - bread_a.cost * amount_a;
    let amount_b = left / bread_b.cost;

    amount_a * bread_a.profit + amount_b * bread_b.profit
}

struct Bread {
    cost: i32,
    profit: i32,
}

impl Bread {
    fn new(cost: i32, profit: i32) -> Bread {
        Bread { cost, profit }
    }
}
