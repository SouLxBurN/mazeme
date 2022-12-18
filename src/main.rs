use console::Term;
use std::io::Read;
use std::sync::mpsc::{self, SendError};
use std::{thread, io};
use std::time::Duration;
mod game_state;
mod render;

use game_state::Movement::*;
use game_state::Clock::*;
use game_state::{GameState, GameStateHandler};
use render::start_render;

use self::game_state::StateEvent;

/// main function
fn main() {
    let board_size = determine_board_size();

    let (tx, rx) = mpsc::channel();
    let state = GameState::new(board_size);
    start_render(rx, state.board.len());
    let state_handler = GameStateHandler::new(state, tx);
    let move_channel = state_handler.get_sender();
    let time_channel = state_handler.get_sender();


    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(1000));
            if let Err(_) = time_channel.send(StateEvent::Clock(SUB(1))) {
                break;
            }
        }
    });

    let stdout = Term::buffered_stdout();
    loop {
        if let Ok(c) = stdout.read_char() {
            if let Err(_) = match c {
                'w' => move_channel.send(StateEvent::Movement(UP)),
                'a' => move_channel.send(StateEvent::Movement(LEFT)),
                's' => move_channel.send(StateEvent::Movement(DOWN)),
                'd' => move_channel.send(StateEvent::Movement(RIGHT)),
                _ => move_channel.send(StateEvent::NoOP),
            } {
                break;
            }
        }
    }

    // io::stdin().read(&mut [0u8]).unwrap();
}

fn determine_board_size() -> usize {
    let term_dims = term_size::dimensions();
    if let None = term_dims {
        println!("terminal size unknown");
        std::process::exit(1);
    }

    let (wd, ht) = term_dims.unwrap();

    if wd/3 < ht {
        return (wd/3)-2;
    }
    ht-2
}
