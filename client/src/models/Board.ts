import React from 'react';
import { Board, Cell } from './Types';
import { CellState } from './Cell';

const createRawBoard = (): Board => {
  let E = CellState.EMPTY;
  return [
    [E, E, E, E, E, E, E], // 1
    [E, E, E, E, E, E, E], // 2
    [E, E, E, E, E, E, E], // 3
    [E, E, E, E, E, E, E], // 4
    [E, E, E, E, E, E, E], // 5
    [E, E, E, E, E, E, E], // 6
  ]
}







export {
  createRawBoard,
}

