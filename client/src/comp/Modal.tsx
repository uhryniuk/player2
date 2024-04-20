import React, {useEffect} from 'react'
import './styles/Modal.css'

const Modal = ({ isOpen, onClose, children }) => {

  useEffect(() => {
    const handleKeyPress = (e: any) => {
      if (e.key === 'Escape') {
        onClose();
      }
    };

    if (isOpen) {
      document.addEventListener('keydown', handleKeyPress);
    }

    return () => {
      document.removeEventListener('keydown', handleKeyPress);
    };
  }, [isOpen, onClose]);

	if (!isOpen) {
		return null
	}

	return (
		<div id='modal-container' onClick={onClose}>
			<div className='window' onClick={(e) => e.stopPropagation()}>
				<div className='title-bar'>
					<div className='title-bar-text'>A Window With Stuff In It</div>
					<div className='title-bar-controls'>
						<button aria-label='Minimize'></button>
						<button aria-label='Maximize'></button>
						<button aria-label='Close' onClick={onClose}></button>
					</div>
				</div>
				<div className='window-body'>
					{children}
				</div>
			</div>
		</div>
	)
}

export default Modal
