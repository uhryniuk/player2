use crate::models::minimax::Minimax;
use serde::{Deserialize, Serialize};

const ROWS: i32 = 6;
const COLS: i32 = 7;
pub const AGENT: i32 = 2;
pub const PLAYER: i32 = 1;
pub type Board = [[i32; COLS as usize]; ROWS as usize];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub board: Board,       // 6 columns and 7 rows...
    pub value: Option<i32>, // The evaluated huristic value.
}

impl GameState {

    pub fn blank_board() -> Board {
        [
            [0, 0, 0, 0, 0, 0, 0], // 1
            [0, 0, 0, 0, 0, 0, 0], // 2
            [0, 0, 0, 0, 0, 0, 0], // 3
            [0, 0, 0, 0, 0, 0, 0], // 4
            [0, 0, 0, 0, 0, 0, 0], // 5
            [0, 0, 0, 0, 0, 0, 0], // 6
        ]
    }

    pub fn generate_child(value: Board, col_idx: i32, max_first: bool) -> Board {
        let mut board = value.clone();

        let child_value = if max_first {
            AGENT.clone()
        } else {
            PLAYER.clone()
        };

        for i in (0..ROWS).rev() {
            if board[i as usize][col_idx as usize] == 0 {
                board[i as usize][col_idx as usize] = child_value;
                break;
            }
        }

        return board;
    }

    fn check_diagonal(&self, board: &Board) -> i32 {
        let mut score = 0;

        // positive sweep
        for i in (3..(ROWS) as usize).rev() {
            for j in 0..(COLS - 3) as usize {
                let mut counts: [i32; 3] = [0, 0, 0]; // zero, player, agent
                let scan = [
                    board[i][j],
                    board[i - 1][j + 1],
                    board[i - 2][j + 3],
                    board[i - 3][j + 3],
                ];
                for elem in scan {
                    counts[elem as usize] += 1;
                }
                score += self.calculate_huer(counts);
            }
        }

        // negative sweep
        for i in (0..(ROWS - 3) as usize).rev() {
            for j in 0..(COLS - 3) as usize {
                let mut counts: [i32; 3] = [0, 0, 0]; // zero, player, agent
                let scan = [
                    board[i][j],
                    board[i + 1][j + 1],
                    board[i + 2][j + 2],
                    board[i + 3][j + 3],
                ];
                for elem in scan {
                    counts[elem as usize] += 1;
                }
                score += self.calculate_huer(counts);
            }
        }

        score
    }

    fn check_vertical(&self, board: &Board) -> i32 {
        let mut score = 0;
        for i in (0..(ROWS - 3) as usize).rev() {
            for j in 0..COLS as usize {
                let mut counts: [i32; 3] = [0, 0, 0]; // zero, player, agent
                let scan = [
                    board[i][j],
                    board[i + 1][j],
                    board[i + 2][j],
                    board[i + 3][j],
                ];
                for elem in scan {
                    counts[elem as usize] += 1;
                }
                score += self.calculate_huer(counts);
            }
        }

        score
    }

    fn check_horizontal(&self, board: &Board) -> i32 {
        let mut score = 0;
        for i in (0..ROWS as usize).rev() {
            for j in 0..(COLS - 3) as usize {
                let mut counts: [i32; 3] = [0, 0, 0]; // zero, player, agent
                let scan = [
                    board[i][j],
                    board[i][j + 1],
                    board[i][j + 2],
                    board[i][j + 3],
                ];
                for elem in scan {
                    counts[elem as usize] += 1;
                }
                score += self.calculate_huer(counts);
            }
        }

        score
    }

    fn calculate_huer(&self, results: [i32; 3]) -> i32 {
        let mut score = 0;
        let (zero, player, agent) = (results[0], results[1], results[2]);
        if agent == 4 {
            score += 500001;
        }
        // preference to go for winning move vs. block
        else if agent == 3 && zero == 1 {
            score += 5000;
        } else if agent == 2 && zero == 2 {
            score += 500;
        } else if player == 2 && zero == 2 {
            score -= 501;
        }
        // preference to block
        else if player == 3 && zero == 1 {
            score -= 5001;
        }
        // preference to block
        else if player == 4 {
            score -= 500000;
        }
        score
    }
}

impl Minimax<Board, i32> for GameState {
    fn value(&self) -> Board {
        self.board
    }

    fn generate_children(&self, value: Board, max: bool) -> Vec<Board> {
        (0..ROWS)
            .into_iter()
            .map(|i| GameState::generate_child(value, i, max))
            .collect()
    }

    fn evaluate(&self, value: Board) -> i32 {
        self.check_horizontal(&value) + self.check_vertical(&value) + self.check_diagonal(&value)
    }
}
