import React, { SyntheticEvent, useEffect, useState } from 'react';
import { Board, Cell } from '../models/Types';
import { CellState } from '../models/Cell';
import { makeMove } from '../models/Board';
import './styles/GameBoard.css';
import './styles/Cell.css';


const renderBoard = (boardState: any, type: CellState): Cell[] => {
    let [board, setBoardState] = boardState;
    let boardCells: Cell[] = []; 

    board.forEach((_: CellState[], i: number) => {
      const tempCells: Cell[] = []; 

      board[i].forEach((state: CellState, j: number) => {
        // onClick callback that sets color of cell and refresh state.
        let callback = () => setBoardState(makeMove(board, type, j));
        
        // The actuall cell on the board.
        tempCells.push(<div key={j} className={`cell-${state}`} onClick={callback}>_</div>) 
      })

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

