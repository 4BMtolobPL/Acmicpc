use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut lines = buf.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();

    let mut posts = vec![Post::new("".to_string())];
    for i in 0..n {
        let parent_index: usize = lines.next().unwrap().parse().unwrap();
        let content = lines.next().unwrap();

        let post = Post::new(content.to_string());
        posts.push(post);

        posts[parent_index].add_child(i + 1);
    }

    let mut out = String::new();
    dfs(1, 0, &posts, &mut out);

    print!("{out}")
}

fn dfs(index: usize, depth: usize, posts: &Vec<Post>, out: &mut String) {
    writeln!(out, "{}{}", ".".repeat(depth), posts[index].content).unwrap();

    for child in posts[index].children.iter() {
        dfs(*child, depth + 1, posts, out);
    }
}

struct Post {
    content: String,
    children: Vec<usize>,
}

impl Post {
    fn new(content: String) -> Post {
        Post {
            content,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: usize) {
        self.children.push(child);
    }
}
