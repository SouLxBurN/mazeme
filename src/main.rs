use std::sync::mpsc;
use std::thread;

use console::Term;

/// Create program that renders a single character in the middle of the screen on launch.
/// Character is controllable with arrow inputs.
/// We'll go from there.
///
/// Create a fixed bounding box.
/// Center on the inital terminal size.
const SYMBOL: char = '●';
const ESC: &str = "\x1b";
const BOARD_SIZE: usize = 25;

const SHADE: char = '░';

enum Movement {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn main() {
    let mut board: [[char; BOARD_SIZE]; BOARD_SIZE] = [[' '; BOARD_SIZE]; BOARD_SIZE];
    let mut position: (usize, usize) = (BOARD_SIZE/2, BOARD_SIZE/2);
    board[position.0][position.1] = SYMBOL;

    let (tx, rx) = mpsc::channel();
    thread::spawn(|| render(rx));
    if let Err(e) = tx.send(board.clone()) {
        panic!("Could not send board state to render {e}");
    }

    let stdout = Term::buffered_stdout();

    loop {
        if let Ok(c) = stdout.read_char() {
            position = match c {
                'w' => move_position(&mut board, position, Movement::UP),
                'a' => move_position(&mut board, position, Movement::LEFT),
                's' => move_position(&mut board, position, Movement::DOWN),
                'd' => move_position(&mut board, position, Movement::RIGHT),
                _ => break,
            };
            if let Err(e) = tx.send(board.clone()) {
                panic!("Could not send board state to render {e}");
            }
        }
    }
}

/// move_position
fn move_position(board: &mut [[char; BOARD_SIZE]; BOARD_SIZE], (mut y_pos, mut x_pos): (usize, usize), action: Movement) -> (usize, usize) {
    board[y_pos][x_pos] = ' ';
    match action {
        Movement::UP => y_pos = (y_pos + board.len() - 1) % board.len(),
        Movement::DOWN => y_pos = (y_pos+1) % board.len(),
        Movement::LEFT => x_pos = (x_pos + board.len() - 1) % board.len(),
        Movement::RIGHT => x_pos = (x_pos+1) % board.len(),
    }
    board[y_pos][x_pos]= SYMBOL;
    (y_pos, x_pos)
}

/// render
fn render(rx: mpsc::Receiver<[[char; BOARD_SIZE]; BOARD_SIZE]>) {
    loop {
        if let Ok(board) = rx.recv() {
            println!("{ESC}[2J{ESC}[H");
            for _ in 0..(3*BOARD_SIZE+2) {
                print!("{SHADE}");
            }
            println!("{ESC}[E{ESC}[A");
            for row in board.iter() {
                print!("{SHADE}");
                for v in row.iter() {
                    print!(" {} ", v);
                }
                print!("{SHADE}");
                println!("{ESC}[E{ESC}[A");
            }
            for _ in 0..(3*BOARD_SIZE+2) {
                print!("{SHADE}");
            }
            println!("");
        }
    }
}
