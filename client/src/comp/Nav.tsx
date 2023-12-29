import React, { useState } from 'react'
import Login from './Login'

const Nav = (props: any) => {
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
				<div className='nav-child left title-bar-text'>connect4</div>
				<div className='nav-child right title-bar-controls'>
					<button aria-label='Minimize'></button>
					<button aria-label='Maximize'></button>
					<button aria-label='Close'></button>
				</div>
			</header>
			{/* Make this look like the 'File Edit View' */}
			<button onClick={openLogin}>login</button>
			{<Login isOpen={isLoginOpen} onClose={closeLogin} children={null}></Login>}
		</>
	)
}

export default Nav
