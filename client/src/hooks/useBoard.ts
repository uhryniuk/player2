import { Board } from "../models/Types"
import config from '../config';

let x = 'http://localhost:9988/api/connect4/make-move';

console.log(config)
const agent = () => {
  const getNextMove = async (board: Board) => {
    let newBoardResponse = await fetch(x, {
      method : "POST",
      mode: 'cors',
      headers: new Headers({'content-type': 'application/json'}),
      body : JSON.stringify({board: board, value: null}),
    })

    // Board is rendered in transposed format.
    return await newBoardResponse.json();
  }
  
  return {
    getNextMove,
  }
}

// NOTE implement this when WS is implemented server side.
const multiplayer = () => {}


export {
  agent,
  multiplayer,
}

