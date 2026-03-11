use std::cmp::PartialEq;
use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut lines = buf.lines().map(|line| line.chars().collect());

    let b1 = Bitmask::from(&(lines.by_ref().take(4).collect::<Vec<Vec<char>>>()));
    let b2 = Bitmask::from(&(lines.collect::<Vec<Vec<char>>>()));

    let mut queue = VecDeque::new();
    if b1 != b2 {
        queue.push_back(b1);
    }

    let mut visited = vec![false; 0xFFFF];
    visited[b1.bits as usize] = true;

    let mut count = 0;
    'outer: while !queue.is_empty() {
        count += 1;
        for _ in 0..queue.len() {
            let b = queue.pop_front().unwrap();

            let next_f = |from: (usize, usize),
                          to: (usize, usize),
                          visited: &mut Vec<bool>,
                          queue: &mut VecDeque<Bitmask>| {
                if let Some(next) = b.next(from, to) {
                    if next == b2 {
                        return true;
                    }
                    if !visited[next.bits as usize] {
                        visited[next.bits as usize] = true;
                        queue.push_back(next);
                    }
                }

                false
            };

            for (x, y) in b.get_knights() {
                if x > 1 {
                    if y > 0 && next_f((x, y), (x - 2, y - 1), &mut visited, &mut queue) {
                        break 'outer;
                    }
                    if y < Bitmask::LENGTH - 1
                        && next_f((x, y), (x - 2, y + 1), &mut visited, &mut queue)
                    {
                        break 'outer;
                    }
                }
                if y > 1 {
                    if x > 0 && next_f((x, y), (x - 1, y - 2), &mut visited, &mut queue) {
                        break 'outer;
                    }
                    if x < Bitmask::LENGTH - 1
                        && next_f((x, y), (x + 1, y - 2), &mut visited, &mut queue)
                    {
                        break 'outer;
                    }
                }
                if x < Bitmask::LENGTH - 2 {
                    if y > 0 && next_f((x, y), (x + 2, y - 1), &mut visited, &mut queue) {
                        break 'outer;
                    }
                    if y < Bitmask::LENGTH - 1
                        && next_f((x, y), (x + 2, y + 1), &mut visited, &mut queue)
                    {
                        break 'outer;
                    }
                }
                if y < Bitmask::LENGTH - 2 {
                    if x > 0 && next_f((x, y), (x - 1, y + 2), &mut visited, &mut queue) {
                        break 'outer;
                    }
                    if x < Bitmask::LENGTH - 1
                        && next_f((x, y), (x + 1, y + 2), &mut visited, &mut queue)
                    {
                        break 'outer;
                    }
                }
            }
        }
    }

    println!("{count}");
}

#[derive(Copy, Clone)]
struct Bitmask {
    bits: u16,
}

impl Bitmask {
    const LENGTH: usize = 4;

    fn from(v: &[Vec<char>]) -> Self {
        let mut bits: u16 = 0;

        for row in v.iter().rev() {
            for c in row.iter().rev() {
                bits <<= 1;
                if *c == '1' {
                    bits |= 1;
                }
            }
        }

        Self { bits }
    }

    fn get(&self, row: usize, col: usize) -> bool {
        self.bits >> (row * Self::LENGTH + col) & 1 == 1
    }

    fn get_knights(&self) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        for i in 0..Self::LENGTH {
            for j in 0..Self::LENGTH {
                if self.get(i, j) {
                    v.push((i, j));
                }
            }
        }

        v
    }

    fn next(&self, from: (usize, usize), to: (usize, usize)) -> Option<Self> {
        if self.get(to.0, to.1) {
            return None;
        }

        let mut clone = *self;
        clone.off(from.0, from.1);
        clone.on(to.0, to.1);
        Some(clone)
    }

    fn on(&mut self, row: usize, col: usize) {
        self.bits |= 1 << (row * Self::LENGTH + col);
    }

    fn off(&mut self, row: usize, col: usize) {
        self.bits &= !(1 << (row * Self::LENGTH + col));
    }
}

impl PartialEq for Bitmask {
    fn eq(&self, other: &Self) -> bool {
        self.bits == other.bits
    }
}
