import React from 'react';
import {Cell, CellState} from '../models/Types';
import './styles/Cell.css'


function createCell(type: CellState, key:any, callback: any): Cell {
  return <div key={key} className={`cell-${type}`} onClick={callback}>_</div>
}

export {
  createCell,
}
