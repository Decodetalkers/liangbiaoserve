use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: usize,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ToLogin {
    pub name: String,
    pub passward: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Logined {
    pub logined: bool,
    pub message: Option<Infomation>,
}
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Infomation {
    pub name: String,
    pub icon: String,
}
impl Infomation {
    pub fn start(name:String) -> Self {
        Self {
            name,
            icon: "www.baidu.com".to_string(),
        }
    }
}
