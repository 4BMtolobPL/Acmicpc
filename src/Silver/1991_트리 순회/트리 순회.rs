use std::collections::HashMap;
use std::io::stdin;

struct Node {
    left: Option<String>,
    right: Option<String>,
}

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    let lines: i32 = buf.trim().parse().unwrap();

    let mut node_map: HashMap<String, Node> = HashMap::new();

    for _ in 0..lines {
        buf.clear();
        stdin().read_line(&mut buf).unwrap();
        let input: Vec<String> = buf.split_whitespace().map(|x| x.to_string()).collect();

        if !node_map.contains_key(&input[0]) {
            node_map.insert(input[0].to_string(), Node { left: None, right: None});
        }

        if !input[1].eq(".") {
            node_map.insert(input[1].to_string(), Node { left: None, right: None });

            let parent_node = node_map.get_mut(&input[0]).unwrap();
            parent_node.left = Some(input[1].to_string());
        }
        if !input[2].eq(".") {
            node_map.insert(input[2].to_string(), Node { left: None, right: None });

            let parent_node = node_map.get_mut(&input[0]).unwrap();
            parent_node.right = Some(input[2].to_string());
        }
    }

    preorder(&node_map, "A");
    println!();
    inorder(&node_map, "A");
    println!();
    postorder(&node_map, "A");
}

fn preorder(x1: &HashMap<String, Node>, x10: &str) {
    let node = x1.get(x10).unwrap();

    print!("{}", x10);
    if node.left.is_some() {
        preorder(x1, &node.left.clone().unwrap());
    }
    if node.right.is_some() {
        preorder(x1, &node.right.clone().unwrap());
    }
}

fn inorder(x1: &HashMap<String, Node>, x10: &str) {
    let node = x1.get(x10).unwrap();

    if node.left.is_some() {
        inorder(x1, &node.left.clone().unwrap());
    }
    print!("{}", x10);
    if node.right.is_some() {
        inorder(x1, &node.right.clone().unwrap());
    }
}

fn postorder(x1: &HashMap<String, Node>, x10: &str) {
    let node = x1.get(x10).unwrap();

    if node.left.is_some() {
        postorder(x1, &node.left.clone().unwrap());
    }
    if node.right.is_some() {
        postorder(x1, &node.right.clone().unwrap());
    }
    print!("{}", x10);
}