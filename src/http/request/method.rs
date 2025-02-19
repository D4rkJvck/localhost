use std::fmt::{
    Display,
    Formatter,
    Result,
};

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Self::GET,
            "POST" => Self::POST,
            _ => Self::Uninitialized,
        }
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Method::GET => write!(f, "GET"),
            Method::POST => write!(f, "POST"),
            Method::Uninitialized => write!(f, "Uninitialized"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

impl Resource {
    pub fn path(&self) -> &str {
        match self {
            Resource::Path(path) => path.as_str(),
        }
    }
}
