import { useState } from "react";
import { adminloginurl } from "~/urls/remoteurl.ts";
import ToLogin from "~/interfaces/logins.d.ts";
export function LoginMessage(): [
  ToLogin | null,
  (input: ToLogin) => void,
] {
  const [logined, setlogined] = useState<ToLogin | null>(null);
  const trylogin = (input: ToLogin) => {
    fetch(adminloginurl, {
      headers: {
        "Content-Type": "application/json",
      },
      method: "POST",
      body: JSON.stringify(input),
    })
      .then((response) => response.json())
      .then((data) => {
        console.log(data);
        if (data["logined"] == true) {
          setlogined(input);
        }
      })
      .catch((error) => {
        console.log(error);
      });
  };
  return [logined, trylogin];
}
