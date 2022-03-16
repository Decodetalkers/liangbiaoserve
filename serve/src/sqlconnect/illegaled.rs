use crate::utils::ToLogin;
use std::error::Error;
use std::fmt::Display;
static KEY: [char; 2] = [';', ' '];
pub trait IsIllegal {
    fn checklegal(&self) -> Result<(), StringIllegal>;
}
impl IsIllegal for ToLogin {
    fn checklegal(&self) -> Result<(), StringIllegal> {
        for key in KEY {
            if self.name.contains(key) || self.passward.contains(key) {
                return Err(StringIllegal {
                    location: key.to_string(),
                });
            }
        }
        Ok(())
    }
}
#[derive(Debug)]
pub struct StringIllegal {
    pub location: String,
}

impl Error for StringIllegal {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
impl Display for StringIllegal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let theerror = format!("location is {}", self.location);
        write!(f, "{theerror}")
    }
}
