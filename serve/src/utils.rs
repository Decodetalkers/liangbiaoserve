use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Display;
#[derive(Debug, Serialize, Deserialize)]
pub struct ToLogin {
    pub name: String,
    pub passward: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Logined {
    pub logined: bool,
    pub message: Option<Infomation>,
    pub failed: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Infomation {
    pub name: String,
    pub icon: String,
}
impl Infomation {
    pub fn start(name: String) -> Self {
        Self {
            name,
            icon: "www.baidu.com".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Index {
    pub filetype: String,
    pub name: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Succeeded {
    pub succeed: bool,
    pub error: Option<String>,
}
#[derive(Debug)]
pub struct UploadFailed {
    pub location: String,
}

impl Error for UploadFailed {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
impl Display for UploadFailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let theerror = format!("location is {}", self.location);
        write!(f, "{theerror}")
    }
}
#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct FoldTable {
    pub id: String,
}
