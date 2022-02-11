use std::io::{self, Write};
use std::sync::mpsc;
use std::thread;

use console::Term;
use figlet_rs::FIGfont;

/// Create program that renders a single character in the middle of the screen on launch.
/// Character is controllable with arrow inputs.
/// We'll go from there.
///
/// Create a fixed bounding box.
/// Center on the inital terminal size.
const SYMBOL: char = '●';
const ESC: &str = "\x1b";
const BOARD_SIZE: usize = 25;

const BORDER: char = '░';
const WALL: char = '█';

enum Movement {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct GameState {
    board: [[char; BOARD_SIZE]; BOARD_SIZE],
    position: Position,
    win_position: Position,
    victory: bool,
}

impl GameState {
    /// move_position
    pub fn move_position(&mut self, action: Movement) {
        let mut new_pos = Position {
            y: self.position.y,
            x: self.position.x,
        };

        match action {
            Movement::UP => new_pos.y = (self.position.y + self.board.len() - 1) % self.board.len(),
            Movement::DOWN => new_pos.y = (self.position.y + 1) % self.board.len(),
            Movement::LEFT => {
                new_pos.x = (self.position.x + self.board.len() - 1) % self.board.len()
            }
            Movement::RIGHT => new_pos.x = (self.position.x + 1) % self.board.len(),
        }

        if self.is_valid_move(&new_pos) {
            self.board[self.position.y][self.position.x] = ' ';
            self.board[new_pos.y][new_pos.x] = SYMBOL;
            self.position = new_pos;

            if self.is_win_position() {
                self.victory = true;
            }
        }
    }

    /// is_valid_move
    /// Accepts a board reference and the destination position.
    /// Returns true if move is valid, otherwise false.
    fn is_valid_move(&self, new_position: &Position) -> bool {
        !self.victory && self.board[new_position.y][new_position.x] == ' '
    }

    fn is_win_position(&self) -> bool {
        self.position == self.win_position
    }
}

fn main() {
    let mut state = GameState {
        board: [[' '; BOARD_SIZE]; BOARD_SIZE],
        position: Position {
            x: BOARD_SIZE / 2,
            y: BOARD_SIZE / 2,
        },
        win_position: Position { x: 0, y: 0 },
        victory: false,
    };

    state.board[state.position.y][state.position.x] = SYMBOL;
    state.board[state.position.y + 2][state.position.x + 2] = WALL;
    state.board[state.position.y + 2][state.position.x] = WALL;
    state.board[state.position.y + 2][state.position.x + 1] = WALL;
    state.board[state.position.y + 1][state.position.x + 2] = WALL;

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || render(rx));
    if let Err(e) = tx.send(state.clone()) {
        panic!("Could not send board state to render {e}");
    }

    let stdout = Term::buffered_stdout();

    loop {
        if let Ok(c) = stdout.read_char() {
            match c {
                'w' => state.move_position(Movement::UP),
                'a' => state.move_position(Movement::LEFT),
                's' => state.move_position(Movement::DOWN),
                'd' => state.move_position(Movement::RIGHT),
                _ => (),
            };

            // Render
            if let Err(e) = tx.send(state.clone()) {
                panic!("Could not send board state to render {e}");
            }
        }
    }
}

/// render
fn render(rx: mpsc::Receiver<GameState>) {
    let (wd, ht) = term_size::dimensions().unwrap_or((BOARD_SIZE + 1, BOARD_SIZE + 1));
    let (draw_x, draw_y) = ((wd - (3 * BOARD_SIZE + 1)) / 2, (ht - BOARD_SIZE + 1) / 2);

    loop {
        if let Ok(state) = rx.recv() {
            print!("{ESC}[2J{ESC}[{draw_y};{draw_x}H");

            // Draw Top Border
            for _ in 0..(3 * BOARD_SIZE + 2) {
                print!("{BORDER}");
            }

            print!("{ESC}[E{ESC}[{draw_x}G");
            // Draw each row
            for row in state.board.iter() {
                // Left Border
                print!("{BORDER}");
                for v in row.iter() {
                    match *v {
                        SYMBOL => print!("◀◆▶"),
                        _ => print!("{v}{v}{v}"),
                    }
                }
                // Right Border
                print!("{BORDER}\n");
                print!("{ESC}[{draw_x}G");
            }

            // Draw Bottom Border
            for _ in 0..(3 * BOARD_SIZE + 2) {
                print!("{BORDER}");
            }

            if state.victory {
                let ffont = FIGfont::standand().unwrap();
                if let Some(msg) = ffont.convert("You Win!") {
                    let mut m_w = msg.to_string().lines().map(|s| s.len()).max().unwrap_or(1);
                    let mut m_h = msg.height as usize;
                    if m_w > wd || m_h > ht {
                        m_w = 0;
                        m_h = 0;
                    }
                    let midpoint = ((wd - m_w) / 2, (ht - m_h) / 2);
                    for (i, l) in msg.to_string().lines().enumerate() {
                        print!("{ESC}[{ht};{w}H", w = midpoint.0, ht = midpoint.1 + i);
                        print!("{}", l);
                    }
                }
            }

            io::stdout().flush().unwrap();
        }
    }
}
