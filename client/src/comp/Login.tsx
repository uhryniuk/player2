import React from 'react'
import './styles/Login.css'

// TODO create EventListener to close when ESC key is pressed.

const Login = ({ isOpen, onClose, children }) => {
	if (!isOpen) {
		return null
	}

	return (
		<div id='modal-container' onClick={onClose}>
			<div className='window'>
				<div className='title-bar'>
					<div className='title-bar-text'>A Window With Stuff In It</div>
					<div className='title-bar-controls'>
						<button aria-label='Minimize'></button>
						<button aria-label='Maximize'></button>
						<button aria-label='Close' onClick={onClose}></button>
					</div>
				</div>
				<div className='window-body'>
					<p>There's so much room for activities!</p>
					{children}
				</div>
			</div>
		</div>
	)
}

export default Login
