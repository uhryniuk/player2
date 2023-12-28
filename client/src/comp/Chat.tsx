import React, { useState } from "react";
import './styles/Chat.css'


function createPlayerMessage(user: string, message: string) {
  return { user, message, timestamp: new Date().toISOString().split('T')[0] }
}

function createEventMessage(message: string) {
  // NOTE Timestamp should be created serverside...
  return { message, timestamp: new Date().toISOString().split('T')[0] }
}

function PlayerMessage({user, message, timestamp}) {

  // TODO User composition to turn this into a event message
  return (
    <section key={`${user}${message}${timestamp}`} className={"user-message"}>
      <div><strong>{user} - {timestamp}</strong></div>
      <div>{message}</div>
    </section>
  )
}

function EventMessage({message, timestamp}) {
  return (
    <section key={`${message}${timestamp}`} className={"user-message"}>
      <div style={{color: 'grey'}}>{message}</div>
    </section>
  ) 
}


const Chat = ({className}) => {
  let [inputValue, setInputValue] = useState("")
  let [messages, setMessages] = useState([] as any)

  const handleSubmit = (e: any) => {
    e.preventDefault();
    // NOTE consider putting in a message guard so lowers spam rate.
    if (inputValue !== '') {
      setMessages(messages.concat([createPlayerMessage("You", inputValue)]))
      setInputValue("")
    }
  }

  return (
    <div id={"chat"} className={`window ${className}`}>
      <section id="thread" className={"sunken-panel"}>
        {EventMessage(createEventMessage("Connected..."))}
        {messages.map((obj) => {
          return PlayerMessage(obj)
        })}
      </section>
      
      <form id="chatbox" className="" onSubmit={handleSubmit}>
        <input 
          id="message-field"
          type="text"
          placeholder="hey dude..."
          value={inputValue}
          onChange={(e) => {setInputValue(e.target.value)}}
        />
        <button type="submit">send</button>
      </form>
    </div>
  )
}

export default Chat
