import utils from './utils'
import checkWinner from './check-winner';

const URLs = {
  MINIMAX_ENDPOINT : 'http://localhost:9988/api/connect4/make-move'
}


function convertBoardTypes(board) {
  for(let i = 0; i < board.length; i++) {
    for (let j = 0; j < board[i].length; j++){

      console.log(board[i][j])
      board[i][j] = Number(board[i][j]) 
    }
  }
  console.log(board)
  return board;
}

// TODO consider writing this, a bit dirty.
async function getAgentMove(boardState, winnerState){
  const [board, setBoard]   = boardState;
  const [winner, setWinner] = winnerState;
  
  let newBoard = utils.transposeBoard(convertBoardTypes(board))
  let win = checkWinner(newBoard)
  console.log(newBoard)
  if (!win){
    try {

      const newBoardResponse = await fetch(URLs.MINIMAX_ENDPOINT, {
        method : "POST",
        mode: 'cors',
        headers: new Headers({'content-type': 'application/json'}),
        body : JSON.stringify({board: newBoard, value: null}),
      })
      
      // Board is rendered in transposed format.
      newBoard = await newBoardResponse.json();
      newBoard = newBoard.board;
      win = checkWinner(newBoard) // Check before Transposing again.
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
