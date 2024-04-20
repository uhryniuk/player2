use rand::Rng;
use serde::{Deserialize, Serialize};

use super::router::{
    RouterInspector,
    Method::{GET, POST},
    get, post
};

use std::collections::HashMap;

use axum::{
    body::Body,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::{Path, Query},
    http::{Method, StatusCode},
    response::{Html, IntoResponse, Json},
    Router,
};

use serde_json::{self, json};


pub trait Minimax<T: Clone, S: Clone + Ord> {
    /// Compares which value in all children is the best, max is true, then we max the first layer
    fn get_move(&self, depth: i32, max: bool) -> T {
        let mut computed: Vec<(T, S)> = Vec::new();
        let possible = self.generate_children(self.value(), max);

        for child in possible {
            let mut local_max = !max.clone(); // TODO double check if we should naught it here?
            let mut result = child.clone();
            for _ in 0..(depth - 1) {
                // TODO fix the depth, think about what it should be.
                let local = self.generate_children(result.clone(), local_max);

                let mut local_optimal = result.clone();
                for child in local.iter() {
                    local_optimal = if local_max {
                        self.max(child.clone(), result.clone())
                    } else {
                        self.min(child.clone(), result.clone())
                    }
                }
                result = local_optimal;

                local_max = !local_max;
            }
            computed.push((child, self.evaluate(result)));
        }

        let mut optimal: T = self.value();
        let optimal_val = self.evaluate(optimal.clone());
        for (t, _) in computed.iter() {
            optimal = if max {
                self.max(t.clone(), optimal.clone())
            } else {
                self.min(t.clone(), optimal.clone())
            }
        }
        optimal = if optimal_val == self.evaluate(optimal.clone()) {
            let mut rng = rand::thread_rng();
            let (t, s) = computed.iter().nth(rng.gen_range(0..7)).unwrap().clone();
            t
        } else {
            optimal.clone()
        };

        optimal
    }

    fn value(&self) -> T;
    fn max(&self, x: T, y: T) -> T {
        if self.evaluate(x.clone()) > self.evaluate(y.clone()) {
            x.clone()
        } else {
            y.clone()
        }
    }
    fn min(&self, x: T, y: T) -> T {
        if self.evaluate(x.clone()) < self.evaluate(y.clone()) {
            x.clone()
        } else {
            y.clone()
        }
    }
    fn generate_children(&self, value: T, max: bool) -> Vec<T>;
    fn evaluate(&self, value: T) -> S;
}

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
        // Consider enhancing to add perference for depth.
        // Add 'decay' (a negative factor) for each subsequent move.

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


pub async fn raw(
    Path((one, two)): Path<(String, String)>,
    queries: Query<HashMap<String, String>>,
) -> (StatusCode, Json<serde_json::Value>) {
    println!("q params {:?}", queries);
    (
        StatusCode::OK,
        Json(serde_json::json!({"first": one, "second": two})),
    )
}

async fn get_move(payload: Json<GameState>) -> (StatusCode, Json<GameState>) {
    let return_move = payload.get_move(31, true);
    (
        StatusCode::OK,
        Json(GameState {
            board: return_move,
            value: None,
        }),
    )
}

async fn handler(queries: Query<HashMap<String, String>>, ws: WebSocketUpgrade) -> impl IntoResponse {
    // NOTE use queries to do the auto matching games
    // NOTE use path params to specify connecting via username.
    println!("{:?}", queries);
    tracing::info!("User connected");
    //ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            // Do something...
            tracing::info!("Some message from user");
 
        } else {
            // client disconnected
            tracing::info!("Disconnect");
            return;
        };

        //if socket.send(msg).await.is_err() {
        //    // client disconnected on error.
        //    tracing::info!("Error Disconnect");
        //    return;
        //}
    }
}

pub fn inspector() -> RouterInspector {
    let router_inspector = RouterInspector::default()
        .add("/make-move", POST, post(get_move)) //.add("/ws", GET, get(handler))
        .add("/raw/:burger/:bother", GET, get(raw));
    
    router_inspector
}
