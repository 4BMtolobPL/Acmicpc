use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
    let r: usize = iter.next().unwrap();
    let c = iter.next().unwrap();

    println!("{}", roll_dice(r, c));
}

fn roll_dice(r: usize, c: usize) -> usize {
    let dice = vec![
        Dice::new(vec![2, 4, 5, 3], 1),
        Dice::new(vec![6, 4, 1, 3], 2),
        Dice::new(vec![1, 5, 6, 2], 3),
        Dice::new(vec![1, 2, 6, 5], 4),
        Dice::new(vec![1, 4, 6, 3], 5),
        Dice::new(vec![5, 4, 2, 3], 6),
    ];

    let m = c % 4;
    let d = c / 4;

    let mut direction = true;
    let mut state = (4, 0);
    let mut sum = 0;

    for _ in 0..r {
        // 한줄 더하기
        sum += dice[state.0].sum * d;
        for j in 0..m {
            if direction {
                sum += dice[state.0].rolls[(state.1 + j) % 4];
            } else {
                sum += dice[state.0].rolls[(state.1 + 4 - j) % 4];
            }
        }

        // 다음 줄 세팅
        let now_face = if direction {
            dice[state.0].rolls[(state.1 + m + 3) % 4]
        } else {
            dice[state.0].rolls[(state.1 + 4 - m + 1) % 4]
        };
        let next_face = dice[state.0].up;
        state.0 = 6 - now_face;
        state.1 = dice[state.0]
            .rolls
            .iter()
            .position(|x| *x == next_face)
            .unwrap();
        direction = !direction;
    }

    sum
}

struct Dice {
    rolls: Vec<usize>,
    sum: usize,
    up: usize,
}

impl Dice {
    fn new(rolls: Vec<usize>, up: usize) -> Dice {
        let sum = rolls.iter().sum();

        Dice { rolls, sum, up }
    }
}
