import React from 'react'
import ReactDOM from 'react-dom/client'
import Game from './pages/Game'
import { BrowserRouter, Route, Routes } from "react-router-dom";
import './index.css'
import '98.css';

ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <BrowserRouter>
      <Routes>
        <Route path='/' element={<Game />} />
      </Routes>
    </BrowserRouter>
  </React.StrictMode>
)



