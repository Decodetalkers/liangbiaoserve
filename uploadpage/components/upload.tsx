import { useState } from "react";
import React from "react";
import { uploadurl } from "~/urls/remoteurl.ts";
import ToLogin from "~/interfaces/logins.d.ts";
interface Layout {
  video: {
    url: string;
    file: File;
  } | null;
  img: {
    url: string;
    file: File;
  } | null;
  txt: {
    source: string;
    file: File;
  } | null;
}
enum type {
  video,
  img,
  txt,
}

export default function Upload({ login }: { login: ToLogin }) {
  const [state, setState] = useState<Array<Layout>>([]);
  const [value, setvalue] = useState<string>("");
  const [selected, setselected] = useState<string>("a");
  const [hasfile, sethasfile] = useState(true);
  const [postfile, selectpostfile] = useState(0);
  const posttypes = ["PICTURE", "VIDEO", "TEXT"];
  const handleChange = (event: React.ChangeEvent<HTMLTextAreaElement>) => {
    setvalue(event.target.value);
  };
  const handleselectedchange = (
    event: React.ChangeEvent<HTMLSelectElement>,
  ) => {
    setselected(event.target.value);
  };
  const onChangeImg = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file: FileList | null = e.target.files; // 取得選中檔案們的一個檔案
    if (file != null) {
      const temp = URL.createObjectURL(file[0]);
      Add(temp, file[0], type.img);
    }
  };
  const onChangeVideo = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file: FileList | null = e.target.files; // 取得選中檔案們的一個檔案
    if (file != null) {
      const temp = URL.createObjectURL(file[0]);
      Add(temp, file[0], type.video);
    }
  };
  const onChangeTxt = () => {
    const fileContext = new File([value], "temp.txt", { type: "" });
    Add(value, fileContext, type.txt);
    setvalue("");
  };

  const uploadFile = () => {
    const formData = new FormData();
    //const login = {
    //	name: "admin",
    //	passward: "cht123456789"
    //};
    if (state.length == 0) {
      sethasfile(false);
      return;
    } else {
      sethasfile(true);
    }
    formData.append(
      "login",
      new Blob([JSON.stringify(login)], {
        type: "application/json",
      }),
    );
    formData.append("tabletype", new Blob([selected], { type: "text/plain" }));
    state.map((e, index) => {
      if (e.img != null) {
        const name = e.img!.file.name;
        const ind = name.lastIndexOf(".");
        const ext = name.substring(ind + 1);
        const finalname = `${index}.${ext}`;
        formData.append("files", e.img!.file, finalname);
      } else if (e.video != null) {
        const name = e.video!.file.name;
        const ind = name.lastIndexOf(".");
        const ext = name.substring(ind + 1);
        const finalname = `${index}.${ext}`;
        formData.append("files", e.video!.file, finalname);
      } else {
        const name = e.txt!.file.name;
        const ind = name.lastIndexOf(".");
        const ext = name.substring(ind + 1);
        const finalname = `${index}.${ext}`;
        formData.append("files", e.txt!.file, finalname);
      }
    });

    fetch(uploadurl, {
      method: "post",
      body: formData,
    })
      .then((response) => response.json())
      .then((data) => console.log(data))
      .catch((error) => {
        console.log(`this is what happened: ${error}`);
      });
  };
  const Add = (url: string, file: File, thetype: type) => {
    const temp: Array<Layout> = [...state];
    let test: Layout = {
      video: null,
      img: null,
      txt: null,
    };
    switch (thetype) {
      case type.img:
        test = {
          video: null,
          img: {
            url: url,
            file: file,
          },
          txt: null,
        };
        temp.push(test);
        break;
      case type.video:
        test = {
          video: {
            url: url,
            file: file,
          },
          img: null,
          txt: null,
        };
        temp.push(test);
        break;
      case type.txt:
        test = {
          video: null,
          img: null,
          txt: {
            source: url,
            file: file,
          },
        };
        temp.push(test);
        break;
    }
    setState(temp);
  };
  const list: Array<JSX.Element> = [];
  state.map((item, index) => {
    if (item.img != null) {
      list.push(<img src={item.img!.url} key={index} />);
    } else if (item.video != null) {
      list.push(
        <video controls key={index}>
          <source src={item.video!.url} type="video/mp4" />
        </video>,
      );
    } else {
      list.push(<p key={index}>{item.txt?.source}</p>);
    }
  });
  const selectedpost: Array<JSX.Element> = [];
  posttypes.map((item, index) => {
    if (postfile == index) {
      selectedpost.push(
        <a style={{ background: "gray" }} key={index}>{item}</a>,
      );
    } else {
      selectedpost.push(
        <a
          key={index + 11}
          onClick={() => {
            selectpostfile(index);
          }}
        >
          {item}
        </a>,
      );
    }
  });
  return (
    <>
      <div className="navbar">
        <a>{login.name}</a>
        <select
          value={selected}
          onChange={handleselectedchange}
        >
          <option value="a">A</option>
          <option value="b">B</option>
          <option value="c">C</option>
          <option value="d">D</option>
        </select>
      </div>
			{!hasfile && <p style={{
					marginTop: 60,
					position : "fixed",
					top: 0,
			}}>you have not post a file</p>}
      <div className="preview">
        {list}
      </div>
      <div className="select-item">
        <div className="switchcase">
          {selectedpost}
        </div>
        {postfile == 0 && (
          <div className="forupload">
            Png:
            <input
              type="file"
              accept="image/png, image/jpeg"
              onChange={onChangeImg}
            />
          </div>
        )}
        {postfile == 1 && (
          <div className="forupload">
            Video
            <input type="file" accept=".mp4" onChange={onChangeVideo} />
          </div>
        )}
        {postfile == 2 && (
          <div className="forupload">
            <textarea
              id="noter-text-area"
              name="textarea"
              value={value}
              onChange={handleChange}
            />
            <button onClick={onChangeTxt}>UploadTxt</button>
          </div>
        )}
      </div>
      <button className="postbtn" onClick={uploadFile}>UPLOAD</button>
    </>
  );
}
