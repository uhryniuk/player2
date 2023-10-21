import React from "react";


const Nav = (props: any) => {

  /*
   * <header> 
   *    - wrapper around all content inside the navigation.
   *    - Control it's max dimensions and flow of children.
   *
   * NOTE the header should be the blue window bar.
   * Make the whole site one big windows 90 screen.
   */
  return (
    <header className='nav title-bar'>
      {/* This class semantic stuff may go haywire. */}
      <div className='nav-child left title-bar-text'>connect4</div>
      <div className='nav-child right title-bar-control'>
        <button>login</button>
      </div>

    </header>
  )

}

export default Nav;
