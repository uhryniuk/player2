import { Board } from '../models/Types'
import config from '../lib/config'

const x = 'http://localhost:8080/api/connect4/make-move'

console.log(config)
const agent = () => {
	const getNextMove = async (board: Board) => {
		const newBoardResponse = await fetch(x, {
			method: 'POST',
			mode: 'cors',
			headers: new Headers({ 'content-type': 'application/json' }),
			body: JSON.stringify({ board: board, value: null }),
		})

		// Board is rendered in transposed format.
		return await newBoardResponse.json()
	}

	return {
		getNextMove,
	}
}

// NOTE implement this when WS is implemented server side.
const multiplayer = () => {}

export { agent, multiplayer }
