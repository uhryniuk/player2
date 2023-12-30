import { Subscription } from '../models/Types'

const socket = new WebSocket('ws://localhost:8080/api/connect4/ws')

// Defaults, but subcribe can be on any key.
const channels = {
	open: [],
	message: [],
	close: [],
	error: [],
}

function subscribe(key: string, callback: Function) {
	if (!channels[key]) {
		channels[key] = []
	}
	channels[key].push(callback)
}

function unsubscribe(key: any, callback: Function) {
	if (!channels[key]) return
	channels[key] = channels[key].filter((x: Function) => x != callback)
}

function clear(key: any) {
	delete channels[key]
}

const rawBoard = [
	[0, 0, 0, 0, 0, 0, 0], // 1
	[0, 0, 0, 0, 0, 0, 0], // 2
	[0, 0, 0, 0, 0, 0, 0], // 3
	[0, 0, 0, 0, 0, 0, 0], // 4
	[0, 0, 0, 0, 0, 0, 0], // 5
	[0, 0, 0, 0, 0, 0, 0], // 6
]

// TODO FUCK THESE SHOULD BE ASYNC/AWAIT...

// Connection opened
socket.addEventListener('open', (event: any) => {
	socket.send(JSON.stringify({ key: 'board', data: { board: rawBoard, value: 123 } }))
	channels.open.forEach((f: Function) => f(socket))
})

// Listen for messages
socket.addEventListener('message', (event: any) => {
	console.log('Message from server:')
	console.log(JSON.parse(event.data))

	try {
		const wsObj = JSON.parse(event.data)
		// TODO read from the key in Subscription object from server.
		// Then call function based on key and pass the data.
	} catch (err: any) {
		console.error(`Error occured: ${err}. Cannot read from WS host: ${event.data}`)
	}

	channels.message.forEach((f: Function) => f(socket))
})

socket.addEventListener('close', (event) => {
	console.warn('Socket Closed')
	channels.close.forEach((f: Function) => f(socket))
})

socket.addEventListener('error', (event) => {
	console.error('WebSocket Error')
	channels.error.forEach((f: Function) => f(socket))
})

//socket.close()

const socketManager = {
	subscribe,
	unsubscribe,
	clear,
}

export default socketManager
