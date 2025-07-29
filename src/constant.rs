#![allow(unused)]

pub enum RequestMethod {
    Initialize,
}

impl From<&str> for RequestMethod {
    fn from(value: &str) -> Self {
        match value {
            "initialize" => Self::Initialize,
            _ => Self::Initialize
        }
    }
}
