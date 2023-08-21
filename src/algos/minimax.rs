/*
 *  Params: depth: int, turn: &str
 *      Recurse rotating each turn in the zero sum game.
 *          Stop once the depth is hit, or cannot compute any more potentialities.
 *      Alternating between each layer, compute the max score for the current respective "turn"
 *          If player 1 just made the move, compute player2's optimal move on the next layer.
 *          Repeat this alternation for all depths.
 *
 *      Note:
 *          Consider making this algorithm take n players/turns
 *          Then it can be used on single player games where u just want to compute future
 *          outcomes.
 *              Example being, best move to make in testris.
 *
 *  Possible Minimax details:
 *      depth: int,
 *      turn_count,
 *
 *  
 *  Possible Eval functions:
 *      compare
 *      evaluate
 *      rotate # rotating the player/turn
 *
 */

pub trait Eval {
    fn compare(&self) -> bool {
        true
    }

    fn evaluate(&self) -> i32 {
        1
    }
}

#[derive(Debug)]
pub struct Minimax {
    depth: i32,     // Recurse Depth
    branch_count: i32,  // Number of branches to spawn (7 in C4 as example)
    player_starts: bool,
}

impl Minimax {
    pub fn new() -> Minimax {
        Minimax { depth: 3, branch_count: 7, player_starts: true }
    }

    pub fn from(depth: i32, branch_count: i32, player_starts: bool) -> Minimax {
        Minimax { depth, branch_count, player_starts }
    }

    pub fn recurse(&self) {
        println!("Down we go!");
    }
}

impl Eval for Minimax {
    fn compare(&self) -> bool {
        println!("This is minimax comparing!");
        true 
    }
}
