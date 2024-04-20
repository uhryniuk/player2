import React, { useState } from 'react'
import Login from './Login'
import "./styles/Nav.css"

const Nav = () => {
	const [isLoginOpen, setIsLoginOpen] = useState(false)

	const openLogin = () => {
		setIsLoginOpen(true)
	}

	const closeLogin = () => {
		setIsLoginOpen(false)
	}

	return (
		<>
			<header className='nav title-bar'>
				<div className='left title-bar-text'>connect4</div>
				<div className='right title-bar-controls'>
					<button aria-label='Minimize'></button>
					<button aria-label='Maximize'></button>
					<button aria-label='Close'></button>
				</div>
			</header>
      
      <section className='nav-button-group'>
			  <div onClick={openLogin} className='nav-button' ><u>L</u>ogin</div>
			  <div onClick={openLogin} className='nav-button' ><u>H</u>elp</div>
      </section>

			{<Login isOpen={isLoginOpen} onClose={closeLogin}></Login>}
		</>
	)
}

export default Nav
