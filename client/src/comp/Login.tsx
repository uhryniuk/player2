import React from "react";
import Modal from "./Modal";


const Login = ({isOpen, onClose}) => {
  return (
    <Modal isOpen={isOpen} onClose={onClose}>
      <form>
        <div className="field-row-stacked" style={{width: "200px"}}>
          <label htmlFor="text22" >Email</label>
          <input id="text22" type="email" value="admin@contoso.com"/>
        </div>
        <div className="field-row-stacked" style={{width: "200px"}}>
          <label htmlFor="text23">Password</label>
          <input id="text23" type="password" value="hunter2"/>
        </div>
        <div className="field-row-stacked" style={{width: "200px"}}>
          <label htmlFor="text24">Favorite Number</label>
          <input id="text24" type="number" value="98"/>
        </div>
      </form>

      <p>hello world!</p>
    </Modal> 
  )
}

export default Login;
