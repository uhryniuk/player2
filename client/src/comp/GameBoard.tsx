import React from 'react'
import { Cell } from '../models/Types'
import { agent } from '../hooks/useBoard'
import { CellState } from '../models/Cell'
import { makeMove } from '../models/Board'
import './styles/GameBoard.css'

// TODO this needs to be remade... slopy and bad design.
const renderBoard = (boardState: any, type: CellState): Cell[] => {
	const [board, setBoardState] = boardState
	const boardCells: Cell[] = []

	board.forEach((_: CellState[], i: number) => {
		const colCells: Cell[] = []
		board[i].forEach((state: CellState, j: number) => {
			const callback = async (e) => {
        e.preventDefault()
				let updatedboard = makeMove(board, type, j)
				setBoardState(updatedboard)

				// NOTE this is the time between renders.
				// TODO should include a LOCK in the board state.
				updatedboard = await agent().getNextMove(updatedboard)
				setBoardState(updatedboard.board)
			}
			colCells.push(<div key={j} className={`cell sunken-panel cell-${state}`} onClick={callback}></div>)
		})
		boardCells.push(
			<div key={i} className={'cell-col'}>
				{colCells}
			</div>,
		)
	})

	return boardCells
}

const GameBoard = ({ className, boardState }) => {
	// NOTE gameMode should be a prop passed to GameBoard.

	// Default, but we can swap this depending on the game mode.
	const type = CellState.PLAYER

	return (
		<div id='board-container' className={`window ${className}`}>
			<div id='board-frame' className={'sunken-panel'}>
				{renderBoard(boardState, type)}
			</div>
		</div>
	)
}

export default GameBoard
