
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
