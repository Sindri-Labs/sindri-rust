use std::io::{self, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor};

pub const PUZZLE: [u8; 81] = [
    0, 0, 2, 0, 0, 0, 9, 7, 0, 8, 0, 0, 0, 0, 5, 0, 0, 2, 0, 0, 0, 4, 0, 0, 0, 8, 0, 2, 3, 0, 0, 0,
    0, 0, 5, 0, 0, 1, 0, 0, 0, 0, 7, 0, 0, 9, 0, 4, 0, 0, 0, 0, 0, 0, 0, 7, 8, 0, 0, 3, 0, 0, 1, 0,
    0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 9, 5, 2, 0, 0, 0,
];
struct SudokuBoard {
    initial: Vec<u8>,
    current: Vec<u8>,
    cursor_pos: (usize, usize), // (row, col)
}

impl SudokuBoard {
    fn new(initial: Vec<u8>) -> Self {
        SudokuBoard {
            initial: initial.clone(),
            current: initial,
            cursor_pos: (0, 0),
        }
    }

    fn is_initial_number(&self, row: usize, col: usize) -> bool {
        self.initial[row * 9 + col] != 0
    }

    fn set_number(&mut self, num: u8) {
        let idx = self.cursor_pos.0 * 9 + self.cursor_pos.1;
        if !self.is_initial_number(self.cursor_pos.0, self.cursor_pos.1) {
            self.current[idx] = num;
        }
    }

    fn move_cursor(&mut self, direction: &str) {
        match direction {
            "up" if self.cursor_pos.0 > 0 => self.cursor_pos.0 -= 1,
            "down" if self.cursor_pos.0 < 8 => self.cursor_pos.0 += 1,
            "left" if self.cursor_pos.1 > 0 => self.cursor_pos.1 -= 1,
            "right" if self.cursor_pos.1 < 8 => self.cursor_pos.1 += 1,
            _ => {}
        }
    }

    fn print(&self) {
        print!("{}", clear::All);
        print!("{}", cursor::Goto(1, 1));

        print!("{}", cursor::Goto(1, 1));
        println!("╔═══════╦═══════╦═══════╗");
        for (i, row) in self.current.chunks(9).enumerate() {
            let row_pos = (i as u16) + 2 + ((i as u16) / 3); // Convert all to u16 first
            print!("{}", cursor::Goto(1, row_pos));
            print!("║ ");
            for (j, &num) in row.iter().enumerate() {
                if (i, j) == self.cursor_pos {
                    if num == 0 {
                        print!("\x1b[47m\x1b[30m· \x1b[0m");
                    } else {
                        print!("\x1b[47m\x1b[30m{} \x1b[0m", num);
                    }
                } else {
                    if num == 0 {
                        print!("· ");
                    } else if self.is_initial_number(i, j) {
                        print!("\x1b[1m{} \x1b[0m", num);
                    } else {
                        print!("{} ", num);
                    }
                }
                if (j + 1) % 3 == 0 && j < 8 {
                    print!("║ ");
                }
            }
            println!("║");

            if (i + 1) % 3 == 0 && i < 8 {
                let separator_pos = row_pos + 1;
                print!("{}", cursor::Goto(1, separator_pos));
                print!("╠═══════╬═══════╬═══════╣\n");
            }
        }
        print!("{}", cursor::Goto(1, 13));
        print!("╚═══════╩═══════╩═══════╝\n");
        print!("{}", cursor::Goto(1, 14));
        print!("Use arrow keys to move, numbers 0-9 to fill\n");
        print!("{}", cursor::Goto(1, 15));
        print!("'q' to quit, 's' to submit, 'c' to show solution\n");
        io::stdout().flush().unwrap();
        println!("\n");
    }

    fn is_complete(&self) -> bool {
        !self.current.contains(&0)
    }

    fn get_solution() -> Vec<u8> {
        vec![
            4, 5, 2, 3, 1, 8, 9, 7, 6, 8, 6, 3, 7, 9, 5, 4, 1, 2, 7, 9, 1, 4, 2, 6, 3, 8, 5, 2, 3,
            7, 1, 8, 4, 6, 5, 9, 6, 1, 5, 2, 3, 9, 7, 4, 8, 9, 8, 4, 5, 6, 7, 1, 2, 3, 5, 7, 8, 6,
            4, 3, 2, 9, 1, 3, 2, 9, 8, 7, 1, 5, 6, 4, 1, 4, 6, 9, 5, 2, 8, 3, 7,
        ]
    }
}

pub fn get_sudoku_solution() -> Option<Vec<u8>> {
    let initial_grid = PUZZLE;
    let mut board = SudokuBoard::new(initial_grid.to_vec());
    let stdin = io::stdin();
    let _raw = io::stdout().into_raw_mode().unwrap();

    board.print();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => {
                print!("{}{}", clear::All, cursor::Goto(1, 1));
                println!("Quitting...");
                return None;
            }
            Key::Char('s') => {
                if board.is_complete() {
                    print!("{}{}", clear::All, cursor::Goto(1, 1));
                    println!("Submitted!");
                    return Some(board.current);
                } else {
                    print!("{}{}", clear::All, cursor::Goto(1, 1));
                    println!("Error: Puzzle is not complete! All cells must be filled.");
                    return None;
                }
            }
            Key::Char('c') => {
                let solution = SudokuBoard::get_solution();
                print!("{}{}", clear::All, cursor::Goto(1, 1));
                println!("Cheated!");

                return Some(solution);
            }
            Key::Char(n) if n.is_digit(10) => {
                board.set_number(n.to_digit(10).unwrap() as u8);
            }
            Key::Up => board.move_cursor("up"),
            Key::Down => board.move_cursor("down"),
            Key::Left => board.move_cursor("left"),
            Key::Right => board.move_cursor("right"),
            _ => continue,
        }
        board.print();
    }

    print!("{}", clear::All);
    print!("{}", cursor::Goto(1, 1));
    None
}
