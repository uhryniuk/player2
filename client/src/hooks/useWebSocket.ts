// Low Level Interface with Global Websocket.


// Create Socket for the connect4 game.
const socket = new WebSocket("ws://localhost:9988/api/connect4/ws");

// Defaults, but subcribe can be on any key.
const channels = {
  open: [],
  message: [],
  close: [],
}


function subscribe(key: any, callback: Function) {
  if (!channels[key]) {
    channels[key] = []
  }
  channels[key].push(callback);
}

function unsubscribe(key: any, callback: Function) {
  if (!channels[key]) return;
  channels[key] = channels[key].filter((x: Function) => x != callback)
}

function clear(key: any) {
  delete channels[key]
}

// Connection opened
socket.addEventListener("open", (event) => {
  socket.send(JSON.stringify({board: [], value: null}));
});

// Listen for messages
socket.addEventListener("message", (event) => {
  console.log("Message from server:");
  console.log(JSON.parse(event.data))
});

socket.addEventListener('close', (event) => {
  console.warn("Socket Closed")
})

socket.addEventListener('error', (event) => {
  console.error("WebSocket Error")
})

//socket.close()


const subscriber = {
  subscribe,
  unsubscribe,
  clear,
}

export default subscriber;

