use axum::{
    extract::{ContentLengthLimit, Multipart, Path},
    http::{
        header::{HeaderMap, HeaderName, HeaderValue},
        StatusCode,
    },
    response::Html,
    routing::{get, get_service},
    Json, Router,
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
use sqlconnect::{get_folds, logininto, registinto};
mod utils;
use utils::*;

use crate::sqlconnect::storageinto;
#[inline]
fn home() -> String {
    std::env::var("HOME").unwrap()
}
#[inline]
fn savepath(file: String) -> String {
    format!("{}/Service/{}", home(), file)
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
    //  etc.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&students)
        .await
        .unwrap_or_else(|_| panic!("nosuch database"));
    let topool = Arc::new(pool);
    let topool2 = Arc::clone(&topool);
    let topool3 = Arc::clone(&topool);
    let topool4 = Arc::clone(&topool);
    let app = Router::new()
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
            "/ws",
            get(show_form).post(
                |input: ContentLengthLimit<
                    Multipart,
                    {
                        250 * 1024 * 1024 /* 250mb */
                    },
                >| async move {
                    let (thepath, output) = accept_form(input).await;
                    if thepath.is_some() {
                        storageinto(&*topool4, thepath.unwrap()).await.unwrap();
                    };
                    output
                },
            ),
        )
        .route(
            "/login",
            get(|| async {})
                .post(|input: Json<ToLogin>| async move { login(input, &*topool).await }),
        )
        .route(
            "/register",
            get(show_form)
                .post(|input: Json<ToLogin>| async move { register(input, &*topool2).await }),
        )
        .route("/folds", get(|| async move { getfolders(&*topool3).await }))
        .route("/image/:id", get(show_image))
        .route("/txt/:id", get(show_txt))
        .route("/json/:id", get(show_json))
        .route("/viewimage", get(img_source))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_headers([AUTHORIZATION, CONTENT_TYPE])
                .allow_methods([Method::GET, Method::POST, Method::POST, Method::DELETE]),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    // run it with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/ws" method="post" enctype="multipart/form-data">
                    <label>
                        Upload file:
                        <input type="file" name="file" multiple>
                    </label>

                    <input type="submit" value="Upload files">
                </form>
            </body>
        </html>
        "#,
    )
}
async fn img_source() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <img src="/image/akalin.png" />
            </body>
        </html>
        "#,
    )
}
async fn accept_form(
    ContentLengthLimit(mut multipart): ContentLengthLimit<
        Multipart,
        {
            250 * 1024 * 1024 /* 250mb */
        },
    >,
) -> (Option<String>, Json<Succeeded>) {
    use std::fs::OpenOptions;
    let time = chrono::offset::Utc::now().to_string();
    let storagepath = base64::encode(time);
    let savedpath = format!("{}/Service/{}", home(), storagepath);
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
        serde_json::to_writer(&file, &indexjson)?;
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
async fn show_json(Path(id): Path<String>) -> Json<Option<Vec<Index>>> {
    //文件扩展名
    //let id = id.replace('$', "/");
    //println!("{id}");
    let show_json_prew: Result<Vec<Index>, Box<dyn std::error::Error>> = async {
        let file_path = format!("{}/Service/{id}/index.json", home());
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
    //文件扩展名
    let id = id.replace('$', "/");
    //println!("{id}");
    let index = id.find('.').unwrap_or(usize::max_value());
    //文件扩展名
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
    //文件扩展名
    let id = id.replace('$', "/");
    let file_path = format!("{}/Service/{id}", home());
    std::fs::read_to_string(file_path).unwrap_or_else(|_| "None of this path".to_string())
    //println!("{id}");
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
async fn getfolders(pool: &Pool<Postgres>) -> Json<Option<Vec<FoldTable>>> {
    match get_folds(pool).await {
        Ok(information) => Json(Some(information)),
        Err(_) => Json(None),
    }
}
