import React from 'react';
import { Board, Cell } from '../models/Types';
import { CellState } from '../models/Cell';
import { createCell } from './Cell';
import { createRawBoard } from '../models/Board';
import './styles/Board.css';



// TODO move this into a models file, probably for Cell.ts.
const placeChip = (board: Board, type: CellState, i: number) => {
  // Scan the board, usng index replace the next available cell with the type chip.
}




const makeMove = (callback) => {

}

const buildBoard = (board: Board): Cell[] => {
    let boardCells: Cell[] = []; 
    board.forEach((_: CellState[], i: number) => {
      const tempCells: Cell[] = []; 
      board[i].forEach((state: CellState, j: number) => {
        tempCells.push(createCell(state, j, (e) => {
          console.log(e)
        }));
      })
      boardCells.push((<div key={i} style={{display: 'flex'}}>{tempCells}</div>));
    })
    return boardCells;
}



const GameBoard = (props: any) => {

  const board: Board = createRawBoard();
  return (<div>{buildBoard(board)}</div>)
}




export default GameBoard;

