import utils from './utils'
import checkWinner from './check-winner';

const URLs = {
  MINIMAX_ENDPOINT : 'http://127.0.0.1:8000/api/minimax/connect4/move'
}

function flattenBoard(board) {
  const newBoard = []
  board.forEach(row => row.forEach(e => newBoard.push(Number(e))));
  return newBoard;
}

function foldBoard(board) {
  const newBoard = []
  let temp = [];
  for(let i = 0; i < 7 * 6; i++) {
    temp.push(board[i]);
    if (temp.length == 7) {
      newBoard.push(temp);
      temp = [];
    }
  }
  return newBoard;
}

function testBoardAdapters(board) {  // Test for the flatten and fold identity.
  const newBoard = foldBoard(flattenBoard(board));
  for(let i = 0; i < board.length; i++) {
    for(let j = 0; j < board[i].length; j++) {
      if (board[i][j] != newBoard[i][j]) return false;
    }
  }
  return true;
}

// TODO consider writing this, a bit dirty.
async function getAgentMove(boardState, winnerState){
  const [board, setBoard]   = boardState;
  const [winner, setWinner] = winnerState;
  const transposedBoard = utils.transposeBoard(board)

  let newBoard = flattenBoard(transposedBoard);
  let win = checkWinner(transposedBoard)

  if (!win){
    try {
      const newBoardResponse = await fetch(URLs.MINIMAX_ENDPOINT, {
        method : "POST",
        mode: 'cors',
        headers: new Headers({'content-type': 'application/json'}),
        body : JSON.stringify({slots: newBoard}),
      })
      
      // Board is rendered in transposed format.
      newBoard = await newBoardResponse.json();
    
      console.log(newBoard);
      newBoard = foldBoard(newBoard.slots)
      console.log(newBoard);

      win = checkWinner(foldBoard(newBoard)) // Check before Transposing again.
    } catch (e) {
      console.log("Error occured when calling the board: ",e)
    }
    
  }

  // Render engine needs matrix transposed form of board.
  const newTransposedBoard = utils.transposeBoard(newBoard)
  setBoard(newTransposedBoard)
  setWinner(win)

  return null
}

const api = {
  getAgentMove : getAgentMove,
  URLs : URLs
}

export default api
