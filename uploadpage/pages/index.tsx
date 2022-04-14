import React from "react";
import { useState } from "react";
import Uploader from "~/components/upload.tsx";
import { LoginMessage } from "~/lib/tologin.ts";
import ToLogin from "~/interfaces/logins.d.ts";
export default function Home() {
  const [logined, trylogin] = LoginMessage();
  const [name, setname] = useState("");
  const [passward, setpassward] = useState("");
  const [ispasswardshow, setpasswardshow] = useState(false);
  const updatename = (event: React.ChangeEvent<HTMLInputElement>) => {
    setname(event.target.value);
  };
  const updatepassward = (event: React.ChangeEvent<HTMLInputElement>) => {
    setpassward(event.target.value);
  };
  const ChangeShow = () => {
    setpasswardshow(!ispasswardshow);
  };
  const GetLogin = () => {
    trylogin({
      name: name,
      passward: passward,
    } as ToLogin);
  };
  return (
    <div className="page">
      <head>
        <title>量表管理平臺</title>
				<meta name="viewport" content="width=device-width, initial-scale=1"/>
        <link rel="stylesheet" href="../style/index.css" />
      </head>
      {logined == null && (
        <div className="login">
          <span className="login100-form-title p-b-26">
            Welcome
            <br />
            <br />
          </span>
					<br/>
          <input
            className="input100"
            id="name-area"
            name="namearea"
            value={name}
            onChange={updatename}
          />
          <br />
          <input
            className="inputpassward"
            type={ispasswardshow ? "text" : "password"}
            id="passward-area"
            name="passwardarea"
            placeholder="Passward"
            value={passward}
            onChange={updatepassward}
          />
          <button
            className="password-icon"
            onClick={ChangeShow}
          >
            View
          </button>
          <br />
          <br />
          <br />
          <button className="login100-form-btn" onClick={GetLogin}>
            Login
          </button>
        </div>
      )}
      {logined != null && <Uploader login={logined as ToLogin} />}
    </div>
  );
}
