import React from 'react'
import { Board, Cell } from './Types'
import { CellState } from './Cell'

const createRawBoard = (): Board => {
	const E = CellState.EMPTY
	return [
		[E, E, E, E, E, E, E], // 1
		[E, E, E, E, E, E, E], // 2
		[E, E, E, E, E, E, E], // 3
		[E, E, E, E, E, E, E], // 4
		[E, E, E, E, E, E, E], // 5
		[E, E, E, E, E, E, E], // 6
	]
}

const makeMove = (board: Board, type: CellState, i: number) => {
	for (let idx = board.length - 1; idx >= 0; idx--) {
		if (board[idx][i] == CellState.EMPTY) {
			board[idx][i] = type
			break
		}
	}

	return JSON.parse(JSON.stringify(board))
}

export { createRawBoard, makeMove }
