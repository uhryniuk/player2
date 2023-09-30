use crate::models::minimax::Minimax;
use serde::{Serialize, Deserialize};

const ROWS: i32 = 6;
const COLS: i32 = 7;
pub const AGENT: i32 = 2;
pub const PLAYER: i32 = 1;
pub type Board = [[i32; ROWS as usize]; COLS as usize];
pub type Move = i32;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub board: Board,  // 6 columns and 7 rows...
    pub value: Option<i32>, // The evaluated huristic value.
}

impl GameState {
    pub fn new(board: Board) -> GameState {
        GameState { board, value: None }
    }

    pub fn from(board: Board, value: Option<i32>) -> GameState {
        GameState { board, value }
    }

    pub fn dumbie_state() -> GameState {
        GameState { board: GameState::blank_board(), value: None }
    }

    pub fn blank_board() -> Board {
        [
            [0,0,0,0,0,0], // 1
            [0,0,0,0,0,0], // 2
            [0,0,0,0,0,0], // 3
            [0,0,0,0,0,0], // 4
            [0,0,0,0,0,0], // 5
            [0,0,0,0,0,0], // 6
            [0,0,0,0,0,0], // 7
        ]
    }

    pub fn generate_child(value: Board, index: i32, max_first: bool) -> Board {
        let mut board = value.clone();
        let mut column = board[index as usize];
        let child_value = if max_first { AGENT.clone() } else { PLAYER.clone() };
        
        for i in 0..(ROWS as i8){
            // Hacky way to count down.
            let idx = (i - ROWS as i8).abs() - 1;
            if column[idx as usize] == 0 {
                column[idx as usize] = child_value;
                break;
            }
        }
        board[index as usize] = column;
        
        return board
    }
}


impl Minimax<Board, i32> for GameState {
    fn value(&self) -> Board {
        self.board
    }

    fn generate_children(&self, value: Board, max: bool) -> Vec<Board> {
        (0..COLS).into_iter().map(|i| GameState::generate_child(value, i, max)).collect()
    }

    fn evaluate(&self, value: Board) -> i32 {
       1 
    }
}

