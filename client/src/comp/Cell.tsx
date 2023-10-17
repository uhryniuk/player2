import React from 'react';
import { Cell } from '../models/Types';
import { CellState } from '../models/Cell';
import './styles/Cell.css'


const makeMove = (board: Board, type: CellState, i: number) => {
  for (let idx = board.length - 1; idx >= 0; idx--) {
    if (board[idx][i] == CellState.EMPTY) {
      board[idx][i] = type;
      break;
    }
  }
  
  return JSON.parse(JSON.stringify(board));
}


function createCell(type: CellState, key: String, callback: any): Cell {
  return <div key={key} className={`cell-${type}`} onClick={callback}>_</div>
}





export {
  createCell,
}
