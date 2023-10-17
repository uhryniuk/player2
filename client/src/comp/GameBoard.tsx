import React from 'react';
import { Board, Cell } from '../models/Types';
import { agent } from '../hooks/useBoard';
import { CellState } from '../models/Cell';
import { makeMove } from '../models/Board';
import './styles/GameBoard.css';


const renderBoard = (boardState: any, type: CellState): Cell[] => {
  let [board, setBoardState] = boardState;
  let boardCells: Cell[] = []; 

  board.forEach((_: CellState[], i: number) => {
    const tempCells: Cell[] = []; 

    board[i].forEach((state: CellState, j: number) => {

      let callback = async () => {
        let updatedboard = makeMove(board, type, j);
        setBoardState(updatedboard);

        // NOTE this is the time between renders.
        // TODO should include a LOCK in the board state.
        updatedboard = await agent().getNextMove(updatedboard);
        setBoardState(updatedboard.board);
      };

      // Add cell, set it's colour
      tempCells.push(<div key={j} className={`cell-${state}`} onClick={callback}>_</div>) 
    })

    // Add column, ensure it's column-wise with flex.
    boardCells.push((<div key={i} style={{display: 'flex'}}>{tempCells}</div>));
  })

  return boardCells;
}

const GameBoard = (props: any) => {
  // NOTE gameMode should be a prop passed to GameBoard.
  
  // Default, but we can swap this depending on the game mode.
  const type = CellState.PLAYER; 

  return (
    <div className='window'>
      {renderBoard(props.boardState, type)}
    </div>
  )
}

export default GameBoard;

