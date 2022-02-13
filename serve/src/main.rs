//! Run with
//!
//! ```not_rust
//! cargo run -p example-multipart-form
//! ```

use axum::{
    extract::{ContentLengthLimit, Multipart},
    http::StatusCode,
    response::Html,
    routing::{get, get_service},
    Json, Router,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{net::SocketAddr, sync::Arc};
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};
mod sqlconnect;
use sqlconnect::{logininto,registinto};
mod utils;
use utils::*;

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
    // build our application with some routes
    let app = Router::new()
        .fallback(
            get_service(ServeDir::new("dist").append_index_html_on_directories(true)).handle_error(
                |error: std::io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                },
            ),
        )
        .route("/ws", get(show_form).post(accept_form))
        .route(
            "/login",
            get(show_form).post(|input: Json<ToLogin>| async move { login(input, &*topool).await }),
        )
        .route(
            "/register",
            get(show_form)
                .post(|input: Json<ToLogin>| async move { register(input, &*topool2).await }),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
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
                <form action="/" method="post" enctype="multipart/form-data">
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

async fn accept_form(
    ContentLengthLimit(mut multipart): ContentLengthLimit<
        Multipart,
        {
            250 * 1024 * 1024 /* 250mb */
        },
    >,
) -> String {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!(
            "Length of `{}` (`{}`: `{}`) is {} bytes",
            name,
            file_name,
            content_type,
            data.len()
        );
    }
    "sss".to_string()
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
