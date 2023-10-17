import React, { useState } from "react";
import GameBoard from '../comp/GameBoard'
import { createRawBoard } from "../models/Board";
import './Game.css'
import './pages.css'


const Game = () => {

  
  let boardState = useState(createRawBoard());
  
  return (
    <div id="container">
      {/*
        Container holds the layout of the widgets
          - Container should have no limiting dimensions usch as height or width
        Layout orchestrates the layout/direction of the widgets
          - Has limiting dimensions and controls flow on mobile too.
        Each widget is a composite component: GameBoard, Leaderboard, Chat, etc.
          - Controls it's child flow for mobile.
          - Controls it's state and all others.
      */}
      
      <div id='layout' className='window'>
        
        <section className="col">
          <GameBoard boardState={boardState}/>
        </section>

        <section className="col">
          <div>Leaderboards</div>
          <div>Chat</div>
        </section>

      </div>

    </div>
  )
}

export default Game;
