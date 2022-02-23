///
/// GameState
///
mod gen;

use self::gen::generate_maze;

const WALL: char = '░';
pub const SYMBOL: char = '●';
pub const GOAL: char = '▓';

pub enum Movement {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub board: Vec<Vec<char>>,
    pub position: Position,
    pub win_position: Position,
    pub victory: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct BoardCell {
    wall_right: bool,
    wall_bottom: bool,
}

impl GameState {
    /// new
    pub fn new(board_size: usize) -> GameState {
        let board = convert_generated_maze(generate_maze(board_size));
        let board_size = board.len();
        let mut state = GameState {
            board,
            position: Position {
                x: 0,
                y: 0,
            },
            win_position: Position { x: board_size-1, y: board_size-1 },
            victory: false,

        };
        state.board[state.position.y][state.position.x] = SYMBOL;
        state.board[state.win_position.y][state.win_position.x] = GOAL;

        return state;
    }

    /// move_position
    pub fn move_position(&mut self, action: Movement) {
        let mut new_pos = Position {
            y: self.position.y,
            x: self.position.x,
        };

        match action {
            Movement::UP => if let Some(y) = self.position.y.checked_sub(1) {
                new_pos.y = y;
            }
            Movement::DOWN => if self.position.y < self.board.len()-1 {
                new_pos.y = self.position.y+1;
            }
            Movement::LEFT => if let Some(x) = self.position.x.checked_sub(1) {
                new_pos.x = x;
            }
            Movement::RIGHT => if self.position.x < self.board.len()-1 {
                new_pos.x = self.position.x+1;
            }
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
        !self.victory && self.board[new_position.y][new_position.x] != WALL
    }

    fn is_win_position(&self) -> bool {
        self.position == self.win_position
    }

    // pub fn start_game(&self) {

    // }

}

/// convert_generated_maze
pub fn convert_generated_maze(generated: Vec<Vec<BoardCell>>) -> Vec<Vec<char>> {
    // Convert Board into render board.
    let render_size = (2 * generated.len()) - 1;
    let mut render_board = vec![vec![' '; render_size]; render_size];

    for y in 0..generated.len() {
        for x in 0..generated.len() {
            let c = generated[y][x];

            let ry = 2*y;
            let rx = 2*x;

            if y < generated.len()-1 {
                if x < generated.len()-1 {
                    if c.wall_bottom {
                        render_board[ry+1][rx] = WALL;
                    }
                    if c.wall_right {
                        render_board[ry][rx+1] = WALL;
                    }
                    render_board[ry+1][rx+1] = WALL;
                } else {
                    if c.wall_bottom {
                        render_board[ry+1][rx] = WALL;
                    }
                }

            } else {
                if c.wall_right {
                    render_board[ry][rx+1] = WALL;
                }
            }
        }
    }
    render_board
}
