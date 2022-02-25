use console::Term;
use std::sync::mpsc;

mod game_state;
mod render;

use game_state::{GameState, Movement};
use render::start_render;

const DEFAULT_BOARD_SIZE: usize = 20;

/// main function
fn main() {
    validate_terminal_size(DEFAULT_BOARD_SIZE);

    let mut state = GameState::new(DEFAULT_BOARD_SIZE);
    let (tx, rx) = mpsc::channel();
    start_render(rx, state.board.len());

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

/// Based on terminal dimensions, determine if game board will render correctly.
fn validate_terminal_size(board_size: usize) {
    let dims = term_size::dimensions();
    if dims == None {
        println!("terminal size unknown");
        std::process::exit(1);
    }

    let (wd, ht) = dims.unwrap();
    if wd/3 < board_size {
            println!("Terminal width too low for size of maze {DEFAULT_BOARD_SIZE}");
            std::process::exit(1);
    }
    if ht < board_size {
            println!("Terminal height is too small for the size of the maze {DEFAULT_BOARD_SIZE}");
            std::process::exit(1);
    }
}
