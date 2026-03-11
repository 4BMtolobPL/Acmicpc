use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut lines = buf.lines();
    let n = lines.next().unwrap().parse().unwrap();

    let mut minesweeper = vec![vec![Minesweeper::new(); n]; n];

    let mut is_game_end = false;
    let add_mine = |x: usize, y: usize, v: &mut Vec<Vec<Minesweeper>>| {
        if let Some(row) = v.get_mut(x) {
            if let Some(mine) = row.get_mut(y) {
                mine.adjacent_mines += 1;
            }
        }
    };
    for (row_index, mine_row) in lines.by_ref().take(n).enumerate() {
        for (col_index, c) in mine_row.chars().enumerate() {
            if c == '*' {
                minesweeper[row_index][col_index].is_mine = true;
                if row_index > 0 {
                    add_mine(row_index - 1, col_index, &mut minesweeper);
                    if col_index > 0 {
                        add_mine(row_index - 1, col_index - 1, &mut minesweeper);
                    }
                    if col_index + 1 < n {
                        add_mine(row_index - 1, col_index + 1, &mut minesweeper);
                    }
                }
                if row_index + 1 < n {
                    add_mine(row_index + 1, col_index, &mut minesweeper);
                    if col_index > 0 {
                        add_mine(row_index + 1, col_index - 1, &mut minesweeper);
                    }
                    if col_index + 1 < n {
                        add_mine(row_index + 1, col_index + 1, &mut minesweeper);
                    }
                }
                if col_index > 0 {
                    add_mine(row_index, col_index - 1, &mut minesweeper);
                }
                if col_index + 1 < n {
                    add_mine(row_index, col_index + 1, &mut minesweeper);
                }
            }
        }
    }

    for (row_index, row) in lines.enumerate() {
        for (col_index, c) in row.chars().enumerate() {
            if c == 'x' {
                minesweeper[row_index][col_index].is_open = true;

                if minesweeper[row_index][col_index].is_mine {
                    is_game_end = true;
                }
            }
        }
    }

    let mut out = String::new();
    for mine_row in minesweeper {
        let mut row = String::new();
        for mine in mine_row {
            if mine.is_mine && is_game_end {
                write!(row, "*").unwrap();
            } else if mine.is_open {
                write!(row, "{}", mine.adjacent_mines).unwrap();
            } else {
                write!(row, ".").unwrap();
            }
        }

        writeln!(out, "{row}").unwrap();
    }

    print!("{out}");
}

#[derive(Clone)]
struct Minesweeper {
    is_open: bool,
    is_mine: bool,
    adjacent_mines: i32,
}

impl Minesweeper {
    fn new() -> Minesweeper {
        Minesweeper {
            is_open: false,
            is_mine: false,
            adjacent_mines: 0,
        }
    }
}
