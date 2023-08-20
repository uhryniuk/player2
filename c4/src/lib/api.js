import utils from './utils'
import checkWinner from './check-winner';

const URLs = {
  MINIMAX_ENDPOINT : 'http://127.0.0.1:8000/api/minimax/connect4/move'
}

function flattenBoard(board) {
  const newBoard = []
  board.forEach(row => {
    row.forEach(e => {
      newBoard.push(Number(e))
    })
  })
  return newBoard;
}

// TODO consider writing this, a bit dirty.
async function getAgentMove(boardState, winnerState){
  const [board, setBoard]   = boardState;
  const [winner, setWinner] = winnerState;
  const transposedBoard = utils.transposeBoard(board)

  const newBoard = flattenBoard(transposedBoard);
  console.log(newBoard);

  const obj = {
    slots: newBoard
  }

  let win = checkWinner(transposedBoard)

  if (!win){
    const newBoardResponse = await fetch(URLs.MINIMAX_ENDPOINT, {
      method : "POST",
      body : JSON.stringify(obj),
      mode: 'no-cors'
    })
    newBoard = await newBoardResponse.json()
    win = checkWinner(newBoard) // Check before Transposing again.
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
