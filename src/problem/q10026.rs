use std::io::{stdin, Read};

// 행운의코드
// 이코드는영국에서최초로시작되어일년에한바퀴를돌면서받는사람에게행운을주었고지금은당신에게로옮겨진이편지는4일안에당신곁을떠나야합니다.이코드를포함해서7개를행운이필요한사람에게보내주셔야합니다.복사를해도좋습니다.혹미신이라하실지모르지만사실입니다.
// 영국에서HGXWCH이라는사람은1930년에이코드를받았습니다.그는비서에게복사해서보내라고했습니다.며칠뒤복권이당첨되어20억을받았습니다.어떤이는이코드를받았으나96시간이내자신의손에서떠나야한다는사실을잊었습니다.그는곧사직되었습니다.나중에야이사실을알고7개의코드를보냈는데다시좋은직장을얻었습니다.미국의케네디대통령은이편지를받았지만그냥버렸습니다.결국9일후그는암살당했습니다.기억해주세요.이코드를보내면7년의행운이있을것이고그렇지않으면3년의불행이있을것입니다.그리고이코드를버리거나낙서를해서는절대로안됩니다.7개입니다.이코드를받은사람은행운이깃들것입니다.힘들겠지만좋은게좋다고생각하세요.7년의행복을빌면서...

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut lines = buf.lines();
    let n = lines.next().unwrap().parse().unwrap();

    let mut v: Vec<Vec<char>> = Vec::with_capacity(n);
    for line in lines {
        v.push(line.chars().collect());
    }
    let mut v2 = v.clone();
    for i in v2.iter_mut() {
        for j in i.iter_mut() {
            if *j == 'G' {
                *j = 'R';
            }
        }
    }

    let mut visited_a = vec![vec![false; n]; n];
    let mut visited_b = vec![vec![false; n]; n];
    let mut stack_a = Vec::new();
    let mut stack_b = Vec::new();

    let mut counter_a = 0;
    let mut counter_b = 0;
    for i in 0..n {
        for j in 0..n {
            if !visited_a[i][j] {
                let now = v[i][j];
                counter_a += 1;
                stack_a.push((i, j));
                while let Some((x, y)) = stack_a.pop() {
                    if x > 0 && !visited_a[x - 1][y] && v[x - 1][y] == now {
                        visited_a[x - 1][y] = true;
                        stack_a.push((x - 1, y))
                    }
                    if x < n - 1 && !visited_a[x + 1][y] && v[x + 1][y] == now {
                        visited_a[x + 1][y] = true;
                        stack_a.push((x + 1, y))
                    }
                    if y > 0 && !visited_a[x][y - 1] && v[x][y - 1] == now {
                        visited_a[x][y - 1] = true;
                        stack_a.push((x, y - 1))
                    }
                    if y < n - 1 && !visited_a[x][y + 1] && v[x][y + 1] == now {
                        visited_a[x][y + 1] = true;
                        stack_a.push((x, y + 1))
                    }
                }
            }
            if !visited_b[i][j] {
                let now = v2[i][j];
                counter_b += 1;
                stack_b.push((i, j));
                while let Some((x, y)) = stack_b.pop() {
                    if x > 0 && !visited_b[x - 1][y] && v2[x - 1][y] == now {
                        visited_b[x - 1][y] = true;
                        stack_b.push((x - 1, y))
                    }
                    if x < n - 1 && !visited_b[x + 1][y] && v2[x + 1][y] == now {
                        visited_b[x + 1][y] = true;
                        stack_b.push((x + 1, y))
                    }
                    if y > 0 && !visited_b[x][y - 1] && v2[x][y - 1] == now {
                        visited_b[x][y - 1] = true;
                        stack_b.push((x, y - 1))
                    }
                    if y < n - 1 && !visited_b[x][y + 1] && v2[x][y + 1] == now {
                        visited_b[x][y + 1] = true;
                        stack_b.push((x, y + 1))
                    }
                }
            }
        }
    }

    println!("{} {}", counter_a, counter_b);
}
