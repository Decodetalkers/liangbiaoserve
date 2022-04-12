import React from "react";
import { useState } from "react";
import Uploader from "~/components/upload.tsx";
import { LoginMessage } from "~/lib/tologin.ts";
import ToLogin from "~/interfaces/logins.d.ts";
export default function Home() {
  const [logined, trylogin] = LoginMessage();
  const [name, setname] = useState("");
  const [passward, setpassward] = useState("");
  const updatename = (event: React.ChangeEvent<HTMLTextAreaElement>) => {
    setname(event.target.value);
  };
  const updatepassward = (event: React.ChangeEvent<HTMLTextAreaElement>) => {
    setpassward(event.target.value);
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
      </head>
      {logined == null && (
        <>
          <textarea
            id="noter-text-area"
            name="textarea"
            value={name}
            onChange={updatename}
          />
          <br />
          <textarea
            id="noter-text-area"
            name="textarea"
            value={passward}
            onChange={updatepassward}
          />
          <button onClick={GetLogin}>Login</button>
        </>
      )}
      <br />
      {logined != null && <Uploader login={logined as ToLogin} />}
    </div>
  );
}
