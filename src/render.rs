use std::io::{self, Write};
use std::sync::mpsc;
use std::thread;
use figlet_rs::FIGfont;

use crate::game_state::{SYMBOL, GOAL, GameState};

const ESC: &str = "\x1b";
const BORDER: char = '░';

pub fn start_render(rx: mpsc::Receiver<GameState>, board_size: usize) {
    let (wd, ht) = term_size::dimensions().unwrap_or((board_size+ 1, board_size + 1));
    let (draw_x, draw_y) = ((wd - (3 * board_size + 1)) / 2, (ht - board_size + 1) / 2);

    thread::spawn(move || {
        loop {
            if let Ok(state) = rx.recv() {
                let mut frame = format!("{ESC}[2J{ESC}[{y};{x}H", y=draw_y, x=draw_x);

                // Draw Top Border
                for i in 0..(3 * board_size + 2) {
                    if i == (3*board_size+2)/2 - 1 {
                        frame.push_str(format!("{ESC}[4m{ESC}[;35m{:03}{ESC}[0m{ESC}[24m", state.time_remaining).as_str());
                    } else if i == (3*board_size+2)/2 || i == (3*board_size+2)/2 + 1 {
                        // Do Nothing
                    } else {
                        frame.push_str(format!("{BORDER}").as_str());
                    }
                }

                frame.push_str(format!("{ESC}[E{ESC}[{x}G", x=draw_x).as_str());
                // Draw each row
                for row in state.board.iter() {
                    // Left Border
                    frame.push_str(format!("{BORDER}").as_str());
                    for v in row.iter() {
                        match *v {
                            SYMBOL => frame.push_str(format!("◀◆▶").as_str()),
                            GOAL => frame.push_str(format!("{ESC}[35m{v}{v}{v}{ESC}[0m").as_str()),
                            _ => frame.push_str(format!("{v}{v}{v}").as_str()),
                        }
                    }
                    // Right Border
                    frame.push_str(format!("{BORDER}\n").as_str());
                    frame.push_str(format!("{ESC}[{x}G", x=draw_x).as_str());
                }

                // Draw Bottom Border
                for _ in 0..(3 * board_size + 2) {
                    frame.push_str(format!("{BORDER}").as_str());
                }

                if state.victory || state.failure{
                    let ffont = FIGfont::standand().unwrap();
                    let mut message = "You Did It!";
                    if state.failure {
                        message = " You Lose!";
                    }
                    if let Some(msg) = ffont.convert(message) {
                        let mut m_w = msg.to_string().lines().map(|s| s.len()).max().unwrap_or(1);
                        let mut m_h = msg.height as usize;
                        if m_w > wd || m_h > ht {
                            m_w = 0;
                            m_h = 0;
                        }
                        let midpoint = ((wd - m_w) / 2, (ht - m_h) / 2);
                        for (i, l) in msg.to_string().lines().enumerate() {
                            frame.push_str(format!("{ESC}[{ht};{w}H", w = midpoint.0, ht = midpoint.1 + i).as_str());
                            frame.push_str(format!("{}", l).as_str());
                        }
                    }
                }
                print!("{frame}");
                io::stdout().flush().unwrap();
            }
        }
    });
}

