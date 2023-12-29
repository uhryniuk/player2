import React, { useEffect, useRef, useState } from 'react'
import './styles/Chat.css'

function createPlayerMessage(user: string, message: string) {
	return { user, message, timestamp: new Date().toISOString().split('T')[0] }
}

function createEventMessage(message: string) {
	// NOTE Timestamp should be created serverside...
	return { message, timestamp: new Date().toISOString().split('T')[0] }
}

function PlayerMessage({ user, message, timestamp }) {
	// TODO User composition to turn this into a event message
	return (
		<section key={`${user}${message}${timestamp}`} className={'user-message'}>
			<div>
				<strong>
					{user} - {timestamp}
				</strong>
			</div>
			<div>{message}</div>
		</section>
	)
}

function EventMessage({ message, timestamp }) {
	return (
		<section key={`${message}${timestamp}`} className={'user-message'}>
			<div style={{ color: 'grey' }}>{message}</div>
		</section>
	)
}

const Chat = ({ className }) => {
	const autoscroll = useRef(null as any) // Some React/TS shit about null references.
	let autoscrollInterval = -1

	const [messages, setMessages] = useState([] as Array<any>)
	const [inputValue, setInputValue] = useState('')

	const execAutoscroll = () => {
		if (autoscroll.current) {
			autoscroll.current.scrollTop = autoscroll.current.scrollHeight
			if (autoscrollInterval !== -1) {
				clearInterval(autoscrollInterval)
			}
		}
	}

	const handleSubmit = (e: any) => {
		e.preventDefault()
		// NOTE consider putting in a message guard so lowers spam rate.
		if (inputValue !== '') {
			setMessages(messages.concat([createPlayerMessage('You', inputValue)]))
			setInputValue('')
		}
		autoscrollInterval = setInterval(execAutoscroll, 50)
	}

	useEffect(() => {
		execAutoscroll()
	}, [])

	return (
		<div id={'chat'} className={`window ${className}`}>
			<section id='thread' ref={autoscroll} className={'sunken-panel'}>
				{EventMessage(createEventMessage('Connected...'))}
				{messages.map((obj) => {
					return PlayerMessage(obj)
				})}
			</section>

			<form id='chatbox' className='' onSubmit={handleSubmit}>
				<input
					id='message-field'
					type='text'
					placeholder='hey dude...'
					value={inputValue}
					onChange={(e) => {
						setInputValue(e.target.value)
					}}
					autoComplete='off'
				/>
				<button type='submit'>send</button>
			</form>
		</div>
	)
}

export default Chat
