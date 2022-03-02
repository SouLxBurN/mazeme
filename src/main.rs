use console::Term;
use std::sync::mpsc::{self, SendError};
use std::thread;
use std::time::Duration;
mod game_state;
mod render;

use game_state::Movement::*;
use game_state::Clock::*;
use game_state::{GameState, GameStateHandler};
use render::start_render;

use self::game_state::StateEvent;

const DEFAULT_BOARD_SIZE: usize = 20;

/// main function
fn main() {
    validate_terminal_size(DEFAULT_BOARD_SIZE);

    let (tx, rx) = mpsc::channel();
    let state = GameState::new(DEFAULT_BOARD_SIZE);
    start_render(rx, state.board.len());
    let state_handler = GameStateHandler::new(state, tx);
    let move_channel = state_handler.get_sender();

    let time_channel = state_handler.get_sender();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(1000));
            if let Err(e) = time_channel.send(StateEvent::Clock(SUB(1))) {
                panic!("Timer Update Failed! {e}");
            }
        }
    });

    let stdout = Term::buffered_stdout();
    loop {
        if let Ok(c) = stdout.read_char() {
            match c {
                'w' => panic_if_failed(move_channel.send(StateEvent::Movement(UP))),
                'a' => panic_if_failed(move_channel.send(StateEvent::Movement(LEFT))),
                's' => panic_if_failed(move_channel.send(StateEvent::Movement(DOWN))),
                'd' => panic_if_failed(move_channel.send(StateEvent::Movement(RIGHT))),
                _ => (),
            };
        }
    }
}

fn panic_if_failed(r: Result<(), SendError<StateEvent>>) {
    if let Err(err) = r {
        panic!("Move Command Failed! {err}");
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
