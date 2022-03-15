import { useState } from "react";
import React from "react";
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

export default function Upload() {
  const [state, setState] = useState<Array<Layout>>([]);
  const [value, setvalue] = useState<string>("");
	const handleChange = (event:React.ChangeEvent<HTMLTextAreaElement>) => {
    setvalue(event.target.value);
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
		Add(value,fileContext,type.txt)
		setvalue('')
	}

  const uploadFile = () => {
    const fromData = new FormData();
    state.map((e,index) => {
      if (e.img != null) {
				const name = e.img!.file.name;
				const ind = name.lastIndexOf(".")
				const ext = name.substring(ind + 1)
				const finalname = `${index}.${ext}`
        fromData.append("files", e.img!.file, finalname);
      }
			else if(e.video !=null ) {
				const name = e.video!.file.name;
				const ind = name.lastIndexOf(".")
				const ext = name.substring(ind + 1)
				const finalname = `${index}.${ext}`
        fromData.append("files", e.video!.file, finalname);
      }
			else {
				const name = e.txt!.file.name;
				const ind = name.lastIndexOf(".")
				const ext = name.substring(ind + 1)
				const finalname = `${index}.${ext}`
        fromData.append("files", e.txt!.file, finalname);
			}
    });
    fetch("/ws", {
      method: "post",
      body: fromData,
    }).catch((error) => (error));
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
					txt:null,
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
					txt:null,
        };
        temp.push(test);
        break;
      case type.txt:
			test = {
          video: null,
          img: null,
					txt: {
						source:url,
						file:file,
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
    } 
		else if (item.video !=null ){
      list.push(
        <video controls key={index}>
          <source src={item.video!.url} type="video/mp4" />
        </video>,
      );
    }
		else {
			list.push(<p>{item.txt?.source}</p>)
		}
  });
  return (
    <>
			{list}
      <button onClick={uploadFile}>UPLOAD</button>
			<br/>
			Png:
      <input type="file" accept="image/png, image/jpeg" onChange={onChangeImg} />
			<br/>
			File
      <input type="file" accept=".mp4" onChange={onChangeVideo} />
			<br/>
			<textarea id="noter-text-area" name="textarea" value={value} onChange={handleChange} />
      <br/>
			<button onClick={onChangeTxt}>UploadTxt</button>
    </>
  );
}
