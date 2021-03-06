use axum::{
    extract::{ContentLengthLimit, Multipart, Path},
    http::{
        header::{HeaderMap, HeaderName, HeaderValue},
        StatusCode,
        //    Request,
    },
    //response::Html,
    routing::{get, get_service},
    Json,
    Router,
    //response::IntoResponse,
    //middleware::{self,Next},
};
use hyper::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    Method,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::fs::read;
use std::{net::SocketAddr, sync::Arc};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};
mod sqlconnect;
use sqlconnect::{
    get_folds, get_forhelp_students, get_history, logininto, registinto, storage_score,
    storageinto, studenthelpinto, teacherlogininto,
};
mod utils;
use once_cell::sync::Lazy;
use utils::*;

use crate::sqlconnect::{adminlogininto, delete_student_for_help, get_all_history};
static HOME: Lazy<String> = Lazy::new(|| std::env::var("HOME").unwrap());
//#[inline]
//fn home() -> String {
//    std::env::var("HOME").unwrap()
//}
#[inline]
fn savepath(file: String) -> String {
    format!("{}/Service/{}", *HOME, file)
}
#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "example_multipart_form=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();
    let students =
        std::env::var("STUDENTS").unwrap_or_else(|_| panic!("plese set the variable of STUDENTS"));
    // Create a connection pool
    //  for MySQL, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&students)
        .await
        .unwrap_or_else(|_| panic!("nosuch database"));
    let topoollogin = Arc::new(pool);
    let topooladminlogin = Arc::clone(&topoollogin);
    let topoolregister = Arc::clone(&topoollogin);
    let topoolfolds = Arc::clone(&topoollogin);
    let topoolupload = Arc::clone(&topoollogin);
    let topoolreceive = Arc::clone(&topoollogin);
    let topoolhistory = Arc::clone(&topoollogin);
    let topoolgetallhistory = Arc::clone(&topoollogin);
    let topoolteacher = Arc::clone(&topoollogin);
    let topoolfindhelp = Arc::clone(&topoollogin);
    let topoolgethelps = Arc::clone(&topoollogin);
    let topoolhelpfinish = Arc::clone(&topoollogin);
    //let topooltest = Arc::clone(&topool);
    let app = Router::new()
        //.route("/table", uploadpage)
        //.route("/", tablepage)
        .fallback(
            get_service(ServeDir::new("routes/upload").append_index_html_on_directories(true))
                .handle_error(|error: std::io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                }),
        )
        .route(
            "/upload",
            get(|| async {}).post(
                |input: ContentLengthLimit<
                    Multipart,
                    {
                        250 * 1024 * 1024 /* 250mb */
                    },
                >| async move {
                    let (thepath, output) = accept_form(&*topoolupload, input).await;
                    if thepath.is_some() {
                        storageinto(&*topoolupload, thepath.unwrap()).await.unwrap();
                    };
                    output
                },
            ),
        )
        //.route_layer(middleware::from_fn(auth))
        .route(
            "/login",
            get(|| async {})
                .post(|input: Json<ToLogin>| async move { login(input, &*topoollogin).await }),
        )
        .route(
            "/adminlogin",
            get(|| async {}).post(|input: Json<ToLogin>| async move {
                adminlogin(input, &*topooladminlogin).await
            }),
        )
        .route(
            "/findhelp",
            get(|| async {}).post(|input: String| async move {
                student_find_for_help(&*topoolfindhelp, input).await
            }),
        )
        .route(
            "/teacherlogin",
            get(|| async {}).post(|input: Json<ToLogin>| async move {
                teacherlogin(input, &*topoolteacher).await
            }),
        )
        .route(
            "/register",
            get(|| async {}).post(|input: Json<ToLogin>| async move {
                register(input, &*topoolregister).await
            }),
        )
        .route(
            "/receive",
            get(|| async {}).post(|input: Json<Score>| async move {
                receivescore(input, &*topoolreceive).await
            }),
        )
        .route(
            "/history",
            get(|| async {})
                .post(|input: String| async move { posthistory(input, &*topoolhistory).await }),
        )
        .route(
            "/finishhelp",
            get(|| async {}).post(|input: String| async move {
                delete_student_for_help(&*topoolhelpfinish, input)
                    .await
                    .unwrap()
            }),
        )
        .route(
            "/allhistory",
            get(|| async move { Json(get_all_history(&*topoolgetallhistory).await.unwrap()) }),
        )
        .route(
            "/folds",
            get(|| async move { getfolders(&*topoolfolds).await }),
        )
        .route(
            "/gethelps",
            get(|| async move { get_students_for_help(&*topoolgethelps).await }),
        )
        .route("/image/:id", get(show_image))
        .route("/txt/:id", get(show_txt))
        .route("/json/:id", get(show_json))
        //.route("/viewimage", get(img_source))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_headers([AUTHORIZATION, CONTENT_TYPE])
                .allow_methods([Method::GET, Method::POST, Method::DELETE]),
        );

    // run it with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
//async fn auth<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
//    println!("hello");
//    println!("{:?},{:?}",req.method(),req.headers());
//    let auth_header = req.headers()
//        .get(axum::http::header::AUTHORIZATION)
//        .and_then(|header| header.to_str().ok());
//
//    match auth_header {
//        Some(_auth_header) => {
//            Ok(next.run(req).await)
//        }
//        _ => Err(StatusCode::UNAUTHORIZED),
//    }
//}

async fn accept_form(
    pool: &Pool<Postgres>,
    ContentLengthLimit(mut multipart): ContentLengthLimit<
        Multipart,
        {
            250 * 1024 * 1024 /* 250mb */
        },
    >,
) -> (Option<String>, Json<Succeeded>) {
    //println!("succeed");
    use std::fs::OpenOptions;
    let time = chrono::offset::Utc::now().to_string();
    let storagepath = base64::encode(time);
    if let Some(test) = multipart.next_field().await.unwrap() {
        let head = test.content_type().unwrap().to_string();
        if &head == "application/json" {
            //let output = String::from_utf8(test.bytes().await.unwrap().to_vec()).unwrap();
            let alogin: ToLogin = serde_json::from_slice(&test.bytes().await.unwrap()).unwrap();
            println!("{},{}", alogin.name, alogin.passward);
            if adminlogininto(pool, alogin).await.is_err() {
                return (
                    None,
                    Json(Succeeded {
                        succeed: false,
                        error: Some("Cannot login".to_string()),
                    }),
                );
            }
        } else {
            return (
                None,
                Json(Succeeded {
                    succeed: false,
                    error: Some("Please login first".to_string()),
                }),
            );
        }
        //println!("{output}");
    }
    let name = match multipart.next_field().await.unwrap() {
        Some(test) => {
            let head = test.content_type().unwrap().to_string();
            let output = String::from_utf8(test.bytes().await.unwrap().to_vec()).unwrap();
            if &head == "text/plain" {
                output
            } else {
                return (
                    None,
                    Json(Succeeded {
                        succeed: false,
                        error: Some("Unkown type".to_string()),
                    }),
                );
            }
        }
        None => {
            return (
                None,
                Json(Succeeded {
                    succeed: false,
                    error: Some("Unkown type".to_string()),
                }),
            );
        }
    };

    let savedpath = format!("{}/Service/{}", *HOME, storagepath);
    let theresult: Result<(), Box<dyn std::error::Error>> = async {
        let mut indexjson: Vec<Index> = vec![];
        let mut file_data: Vec<(String, axum::body::Bytes)> = vec![];
        while let Some(field) = multipart.next_field().await? {
            //let name = field.name().unwrap().to_string();
            let file_name = field.file_name().unwrap().to_string();
            let content_type = field.content_type().unwrap().to_string();
            let data = field.bytes().await.unwrap();
            //let temp = savepath(file_name.clone());
            //println!("{temp}");
            match content_type.as_str() {
                "application/octet-stream" => {
                    indexjson.push(Index {
                        filetype: "TXT".to_string(),
                        name: file_name.clone(),
                    });
                }
                "video/mp4" => {
                    indexjson.push(Index {
                        filetype: "Video".to_string(),
                        name: file_name.clone(),
                    });
                }
                "image/png" | "image/jpeg" => {
                    indexjson.push(Index {
                        filetype: "Image".to_string(),
                        name: file_name.clone(),
                    });
                }
                _ => {
                    let theerror: Box<dyn std::error::Error> = Box::new(UploadFailed {
                        location: file_name,
                    });
                    return Err(theerror);
                }
            }
            file_data.push((file_name, data));
        }
        let menu = FileMenu {
            tabletype: name,
            menu: indexjson,
        };
        tokio::fs::create_dir_all(&savedpath).await?;
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(format!("{}/index.json", savedpath))?;
        for (name, data) in file_data.iter() {
            tokio::fs::write(format!("{}/{}", &savedpath, name), data)
                .await
                .map_err(|err| err.to_string())?;
        }
        serde_json::to_writer(&file, &menu)?;
        Ok(())
    }
    .await;
    match theresult {
        Ok(()) => (
            Some(storagepath),
            Json(Succeeded {
                succeed: true,
                error: None,
            }),
        ),
        Err(e) => (
            None,
            Json(Succeeded {
                succeed: false,
                error: Some(e.to_string()),
            }),
        ),
    }
}
async fn show_json(Path(id): Path<String>) -> Json<Option<FileMenu>> {
    //???????????????
    //let id = id.replace('$', "/");
    //println!("{id}");
    let show_json_prew: Result<FileMenu, Box<dyn std::error::Error>> = async {
        let file_path = format!("{}/Service/{id}/index.json", *HOME);
        let file = std::fs::File::open(file_path)?;
        Ok(serde_json::from_reader(file)?)
    }
    .await;
    match show_json_prew {
        Ok(json) => Json(Some(json)),
        Err(_) => Json(None),
    }
}
async fn show_image(Path(id): Path<String>) -> (HeaderMap, Vec<u8>) {
    //???????????????
    let id = id.replace('$', "/");
    //println!("{id}");
    let index = id.find('.').unwrap_or(usize::max_value());
    //???????????????
    let mut ext_name = "xxx";
    if index != usize::max_value() {
        ext_name = &id[index + 1..];
    }
    //println!("{ext_name}");
    let content_type = format!("image/{}", ext_name);
    //println!("{}", content_type);
    let mut headers = HeaderMap::new();
    //let content_type = "image/akaling.png".to_string();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_str(&content_type).unwrap(),
    );
    let file_name = savepath(id);
    (headers, read(&file_name).unwrap())
}
async fn show_txt(Path(id): Path<String>) -> String {
    //???????????????
    let id = id.replace('$', "/");
    let file_path = format!("{}/Service/{id}", *HOME);
    std::fs::read_to_string(file_path).unwrap_or_else(|_| "None of this path".to_string())
    //println!("{id}");
}
async fn adminlogin(Json(input): Json<ToLogin>, pool: &Pool<Postgres>) -> Json<Logined> {
    match adminlogininto(pool, input).await {
        Ok(information) => Json(Logined {
            logined: true,
            message: information,
            failed: None,
        }),
        Err(e) => Json(Logined {
            logined: false,
            message: None,
            failed: Some(e.to_string()),
        }),
    }
}
async fn teacherlogin(Json(input): Json<ToLogin>, pool: &Pool<Postgres>) -> Json<Logined> {
    match teacherlogininto(pool, input).await {
        Ok(information) => Json(Logined {
            logined: true,
            message: information,
            failed: None,
        }),
        Err(e) => Json(Logined {
            logined: false,
            message: None,
            failed: Some(e.to_string()),
        }),
    }
}
async fn login(Json(input): Json<ToLogin>, pool: &Pool<Postgres>) -> Json<Logined> {
    match logininto(pool, input).await {
        Ok(information) => Json(Logined {
            logined: true,
            message: information,
            failed: None,
        }),
        Err(e) => Json(Logined {
            logined: false,
            message: None,
            failed: Some(e.to_string()),
        }),
    }
}
async fn register(Json(input): Json<ToLogin>, pool: &Pool<Postgres>) -> Json<Logined> {
    match registinto(pool, input).await {
        Ok(Some(information)) => Json(Logined {
            logined: true,
            message: Some(information),
            failed: None,
        }),
        Ok(None) => Json(Logined {
            logined: false,
            message: None,
            failed: Some("Has already logined".to_string()),
        }),
        Err(e) => Json(Logined {
            logined: false,
            message: None,
            failed: Some(e.to_string()),
        }),
    }
}
async fn getfolders(pool: &Pool<Postgres>) -> Json<Option<Vec<FoldTable>>> {
    match get_folds(pool).await {
        Ok(information) => Json(Some(information)),
        Err(_) => Json(None),
    }
}
async fn student_find_for_help(pool: &Pool<Postgres>, id: String) -> Json<bool> {
    match studenthelpinto(pool, id).await {
        Ok(()) => Json(true),
        Err(_) => Json(false),
    }
}
async fn get_students_for_help(pool: &Pool<Postgres>) -> Json<Option<Vec<StudentForHelp>>> {
    match get_forhelp_students(pool).await {
        Ok(information) => Json(Some(information)),
        Err(_) => Json(None),
    }
}
async fn receivescore(Json(input): Json<Score>, pool: &Pool<Postgres>) {
    storage_score(pool, input).await.unwrap();
}
async fn posthistory(name: String, pool: &Pool<Postgres>) -> Json<Vec<Score>> {
    Json(get_history(pool, name).await.unwrap())
}
