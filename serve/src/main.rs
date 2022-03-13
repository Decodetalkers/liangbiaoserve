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
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::fs::read;
use std::{net::SocketAddr, sync::Arc};
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};
mod sqlconnect;
use sqlconnect::{logininto, registinto};
mod utils;
use utils::*;
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
        //.connect("postgres://postgres:cht123456789@localhost/students")
        .connect(&students)
        .await
        .unwrap_or_else(|_| panic!("nosuch database"));
    let topool = Arc::new(pool);
    let topool2 = Arc::clone(&topool);
    //let statestart = State {
    //    id : "None".to_string(),
    //};
    //let state = Arc::new(tokio::sync::Mutex::new(statestart));
    //let state_change = Arc::clone(&state);
    // build our application with some routes
    let app = Router::new()
        .fallback(
            get_service(ServeDir::new("routes/upload").append_index_html_on_directories(true))
                .handle_error(|error: std::io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                })
                .post(accept_form),
        )
        .route("/ws", get(show_form).post(accept_form))
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
        .route("/image/:id", get(show_image))
        .route("/json/:id", get(show_json))
        .route("/viewimage", get(img_source))
        //.route("/preview", get(show_image_uploaded))
        //.route("/preview2", get(show_image_uploaded)
        //       .post(|Json(input): Json<State>| async move {
        //            let mut statepost = state_change.lock().await;
        //            statepost.id = input.id;
        //       }))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    // run it with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 4000));
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
) -> Json<Succeeded> {
    use std::fs::OpenOptions;
    let time = chrono::offset::Utc::now().to_string();
    let storagepath = base64::encode(time);
    let savedpath = format!("{}/Service/{}", home(), storagepath);
    let theresult: Result<(), Box<dyn std::error::Error>> = async {
        tokio::fs::create_dir_all(&savedpath).await?;
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(format!("{}/index.json", savedpath))?;
        let mut indexjson: Vec<Index> = vec![];

        while let Some(field) = multipart.next_field().await? {
            let name = field.name().unwrap().to_string();
            let file_name = field.file_name().unwrap().to_string();
            let content_type = field.content_type().unwrap().to_string();
            let data = field.bytes().await.unwrap();
            //let temp = savepath(file_name.clone());
            //println!("{temp}");
            indexjson.push(Index {
                filetype: "Image".to_string(),
                name: file_name.clone(),
            });
            tokio::fs::write(format!("{}/{}", &savedpath, file_name), &data)
                .await
                .map_err(|err| err.to_string())?;
            println!(
                "Length of `{}` (`{}`: `{}`) is {} bytes",
                name,
                file_name,
                content_type,
                data.len()
            );
        }
        serde_json::to_writer(&file, &indexjson)?;
        Ok(())
    }
    .await;
    match theresult {
        Ok(()) => Json(Succeeded {
            succeed: true,
            error: None,
        }),
        Err(e) => Json(Succeeded {
            succeed: false,
            error: Some(e.to_string()),
        }),
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
    println!("{ext_name}");
    let content_type = format!("image/{}", ext_name);
    println!("{}", content_type);
    let mut headers = HeaderMap::new();
    //let content_type = "image/akaling.png".to_string();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_str(&content_type).unwrap(),
    );
    let file_name = savepath(id);
    (headers, read(&file_name).unwrap())
}

async fn login(Json(input): Json<ToLogin>, pool: &Pool<Postgres>) -> Json<Logined> {
    let information = logininto(pool, input).await.unwrap_or(None);
    Json(Logined {
        logined: information.is_some(),
        message: information,
    })
}
async fn register(Json(input): Json<ToLogin>, pool: &Pool<Postgres>) -> Json<Logined> {
    let information = registinto(pool, input).await.unwrap();
    Json(Logined {
        logined: information.is_some(),
        message: information,
    })
}
