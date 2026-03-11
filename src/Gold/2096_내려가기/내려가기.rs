use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let lines = buf.lines();

    let mut prev: Option<Node> = None;
    for line in lines.skip(1) {
        let mut iter = line.split_ascii_whitespace();
        let (l, m, r): (i32, i32, i32) = (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        );

        if let Some(node) = prev {
            let l_max = node.l_max + l;
            let m_max = node.l_max.max(node.r_max) + m;
            let r_max = node.r_max + r;

            let l_min = node.l_min + l;
            let m_min = node.l_min.min(node.r_min) + m;
            let r_min = node.r_min + r;

            let new_node = Node {
                l_max: l_max.max(m_max),
                l_min: l_min.min(m_min),
                r_max: r_max.max(m_max),
                r_min: r_min.min(m_min),
            };

            prev = Some(new_node);
        } else {
            prev = Some(Node {
                l_max: l.max(m),
                l_min: l.min(m),
                r_max: m.max(r),
                r_min: m.min(r),
            });
        }
    }

    let last = prev.unwrap();
    println!(
        "{} {}",
        last.l_max.max(last.r_max),
        last.l_min.min(last.r_min)
    );
}

struct Node {
    l_max: i32,
    l_min: i32,
    r_max: i32,
    r_min: i32,
}
