use console::Term;
use std::sync::mpsc;

mod game_state;
mod render;

use game_state::{GameState, Movement};
use render::start_render;

const BOARD_SIZE: usize = 20;

/// main function
fn main() {
    let mut state = GameState::new(BOARD_SIZE);
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
