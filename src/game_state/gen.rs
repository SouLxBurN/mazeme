///
/// Generate Maze Module
///
use rand::Rng;
use std::collections::HashSet;

use super::{BoardCell, Position};

/// generate_maze
pub fn generate_maze(board_size: usize) -> Vec<Vec<BoardCell>> {
    let gen_size = (board_size / 2) + 1;
    let mut board = vec![vec![BoardCell {
        wall_right: false,
        wall_bottom: false,
    }; gen_size]; gen_size];
    let mut pos = Position {
        y: gen_size / 2,
        x: gen_size / 2,
    };
    let mut visited = HashSet::new();
    let mut stack = vec![pos];

    let mut popped = false;
    while stack.len() > 0 && visited.len() < board_size.pow(2) {
        visited.insert(pos);

        let mut moves = vec![];
        // Move Up
        if pos.y > 0 {
            let mv = Position {
                y: pos.y - 1,
                x: pos.x,
            };
            if !visited.contains(&mv) {
                moves.push(mv);
            } else if !popped && stack[stack.len() - 1] != mv {
                board[mv.y][mv.x].wall_bottom = true;
            }
        }
        // Move Left
        if pos.x > 0 {
            let mv = Position {
                y: pos.y,
                x: pos.x - 1,
            };
            if !visited.contains(&mv) {
                moves.push(mv);
            } else if !popped && stack[stack.len() - 1] != mv {
                board[mv.y][mv.x].wall_right = true;
            }
        }
        // Move Down
        if pos.y < gen_size - 1 {
            let mv = Position {
                y: pos.y + 1,
                x: pos.x,
            };
            if !visited.contains(&mv) {
                moves.push(mv);
            } else if !popped && stack[stack.len() - 1] != mv {
                board[pos.y][pos.x].wall_bottom = true;
            }
        }
        // Move Right
        if pos.x < gen_size - 1 {
            let mv = Position {
                y: pos.y,
                x: pos.x + 1,
            };
            if !visited.contains(&mv) {
                moves.push(mv);
            } else if !popped && stack[stack.len() - 1] != mv {
                board[pos.y][pos.x].wall_right = true;
            }
        }

        if moves.len() > 0 {
            stack.push(pos);
            popped = false;
            // Choose randomly where to move.
            let mut rng = rand::thread_rng();
            let move_idx = rng.gen_range(0..moves.len());
            pos = moves[move_idx];
        } else {
            pos = stack.pop().unwrap();
            popped = true;
        }
    }
    board
}
