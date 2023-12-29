import React, { useState } from 'react'
import './styles/Leaderboard.css'

/*
 * NOTE Need to figure out the categories for the leadeboard and what is being measured.
 *
 * Leaderboard for wins/losses versus an AI.
 * Leaderboard for wins/losses against opponents.
 * Leaderboard for lowest number of moves to win.
 * Leaderboard for fastest game too.
 *
 * I think I should track all of the above, then it's easy to add stuff later to the leaderboards.
 *
 * Metric to use:
 *  - Wins/Losses and ratio of the two for each player.
 *  - categories include:
 *    - player versus AI (filter by difficulty level)
 *    - palyer versus player (just plain w/l ratio)
 * */

function createEntry(user: string, difficulty: number, moveCount: number) {
	return { user, difficulty, moveCount }
}

function Entry({ user, difficulty, moveCount }) {
	return (
		<tr key={`${user}${difficulty}${moveCount}`}>
			<td>{user}</td>
			<td>{difficulty}</td>
			<td>{moveCount}</td>
		</tr>
	)
}

const Leaderboard = ({ className }) => {
	const defaultSamples = [
		createEntry('Dilly', 1, 2),
		createEntry('Johnny', 2, 2),
		createEntry('Lanie', 2, 2),
		createEntry('Bong', 2, 2),
		createEntry('Doyoungie', 2, 2),
		createEntry('Hedongie', 2, 2),
		createEntry('Monkey', 2, 2),
		createEntry('ookies', 2, 2),
		createEntry('ananas', 2, 2),
		createEntry('ineapple', 2, 2),
		createEntry('pples', 2, 2),
		createEntry('ears', 2, 2),
		createEntry('alker', 2, 2),
		createEntry('anie', 2, 2),
		createEntry('ong', 2, 2),
		createEntry('oyoungie', 2, 2),
		createEntry('edongie', 2, 2),
		createEntry('onkey', 2, 2),
		createEntry('okies', 2, 2),
		createEntry('nie', 2, 2),
		createEntry('ng', 2, 2),
		createEntry('youngie', 2, 2),
		createEntry('dongie', 2, 2),
		createEntry('nkey', 2, 2),
	]
	// NOTE as some point should update these entries in real time.
	const [entries, setEntries] = useState(defaultSamples)

	// TODO implement somethign for this later on...
	return (
		<div id='leaderboard' className={`window ${className}`}>
			<div id='table-container' className='sunken-panel'>
				<table className='interactive'>
					<thead>
						<tr>
							<th>User</th>
							<th>Difficulty</th>
							<th># of Moves</th>
						</tr>
					</thead>
					<tbody>
						{entries.map((obj: any) => {
							return Entry(obj)
						})}
					</tbody>
				</table>
			</div>
		</div>
	)
}

export default Leaderboard
