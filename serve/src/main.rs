//! Run with
//!
//! ```not_rust
//! cargo run -p example-multipart-form
//! ```

use axum::{
    extract::{ContentLengthLimit, Multipart},
    response::Html,
    routing::{get,get_service},
    Router,
    http::StatusCode,
    Json,
};
use std::net::SocketAddr;
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan,TraceLayer},
};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}
//trait Loginable{}
//
#[derive(Debug, Serialize, Deserialize)]
struct Logined {
    logined: bool,
    message: Option<Infomation>,
}
#[derive(Debug, Serialize, Deserialize)]
struct Infomation {
    name:String,
    icon: String,
}
//impl Loginable for Logined{}
//
//#[derive(Debug, Serialize, Deserialize)]
//struct Loginfailed {
//    logintype: String,
//    icon: String,
//}
//impl Loginable for Loginfailed{}
#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "example_multipart_form=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    // build our application with some routes
    let app = Router::new()
        .fallback(
            get_service(
                ServeDir::new("dist").append_index_html_on_directories(true),
            )
            .handle_error(|error: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            }),
        )
        .route("/ws", get(show_form).post(accept_form))
        .route("/login", get(show_form).post(login))
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
    return "sss".to_string();
}
async fn login(Json(input): Json<Claims>) -> Json<Logined> {
    println!("{},{},{}",input.exp,input.company,input.sub);
    if input.exp > 10 {
    //    Json(Box::new(Logined{
    //        icon:"test".to_string(),
    //        name:"test".to_string(),
    //    }))
    //
        Json(
            Logined{
                logined: true,
                message: Some(
                    Infomation{
                        name: "test".to_string(),
                        icon: "test".to_string(),
                })
            }
        )
    } else {
        Json(
            Logined{
                logined:false,
                message: None,
            }
        )
    //    Json(Box::new(Loginfailed{
    //        logintype: "Teacher".to_string(),
    //        icon:"failed".to_string()
    //    }))
    }
    //return Json(Claims{
    //    sub: "beta".to_string(),
    //    company: "beta".to_string(),
    //    exp:1,
    //});
}
