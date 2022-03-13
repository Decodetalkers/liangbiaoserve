import {useState} from 'react'
import React from 'react'
interface Layout {
	video : {
		url: string ,
		file : File 
	} | null,
	img: {
		url:string ,
		file: File 
	} | null
}
enum type {
	video,
	img,
}

export default function Upload(){
	const [state,setState] = useState<Array<Layout>>([
	])
	const onChangeImg = (e:React.ChangeEvent<HTMLInputElement>) => {
    const file : FileList | null = e.target.files; // 取得選中檔案們的一個檔案
		if(file != null) {
			const temp = URL.createObjectURL(file[0])
			Add(temp,file[0],type.img)
		}
  };
	const onChangeVideo = (e:React.ChangeEvent<HTMLInputElement>) => {
    const file : FileList | null = e.target.files; // 取得選中檔案們的一個檔案
		if(file != null) {
			const temp = URL.createObjectURL(file[0])
			Add(temp,file[0],type.video)
		}
  };


	const uploadFile = (_: React.MouseEvent<HTMLSpanElement, MouseEvent>) => {

		const fromData = new FormData();
		state.forEach((e) => {
			if(e.img!=null) {
				fromData.append("files",e.img!.file,e.img!.file.name)
			}else {
				fromData.append("files",e.video!.file,e.video!.file.name)
			}
		})
		const str = "test"
		const fileContext = new File([str],"1.txt",{type:''})
		fromData.append('files',fileContext)
    fetch("/ws", {
        method: "post",
        body: fromData,
    }).catch((error) => (error));
	};
	const Add = (url:string,file: File,thetype:type) => {
		const temp: Array<Layout> = [...state]
		let test : Layout = {
			video:null,
			img: null,
		}
		switch (thetype) {
			case type.img:
				test = {
					video: null,
					img : {
						url:url,
						file: file
					}
				};
				temp.push(test)
				break;
			case type.video:
				test = {
					video: {
						url:url,
						file:file
					},
					img : null,
				};
				temp.push(test)
				break;
			default:
				break;
		}
		setState(temp)
	}
	const list:Array<JSX.Element> = []
	state.map((item,index)=> {
		if(item.video ==null) {
			list.push(<img src={item.img!.url} key = {index}/>) 
			}else {
			list.push(
			<video controls key = {index}>
				<source src={item.video!.url}  type="video/mp4"/>
			</video>) 
		}
	})
	return  (
		<>
			<button onClick={uploadFile}> UPLOAD </button>
			<form action="/ws" method="post" encType="multipart/form-data">
    	    <label>
    	        Upload file:
					<input type="file" accept=".png" onChange={onChangeImg} />
					<input type="file" accept=".mp4" onChange={onChangeVideo} />
    	    <input type="file" name="file" multiple />
    	    </label>
    	  <input type="submit" value="Upload files" />
    	</form>
			{list}
		</>
	)
}
