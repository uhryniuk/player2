import React, { useState } from 'react'
import GameBoard from '../comp/GameBoard'
import Chat from '../comp/Chat'
import Leaderboard from '../comp/Leaderboard'
import Nav from '../comp/Nav'
import { createRawBoard } from '../models/Board'
import './Game.css'

function Game(props: any) {
	const boardState = useState(createRawBoard())

	return (
		// Window ID corresponds to the entire window of the main frame.
		<div id='window' className='window'>
			<Nav />
			<div id='layout'>
				<div id='container' className=''>
					<section id={'col-1'} className={'col'}>
						<GameBoard className={'widget'} boardState={boardState} />
					</section>

					<section id={'col-2'} className={'col'}>
						<Leaderboard className={'widget'} />
						<Chat className={'widget'} />
					</section>
				</div>
			</div>
		</div>
	)
}

export default Game
