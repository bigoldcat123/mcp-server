#[derive(Debug,Clone, Copy,PartialEq, Eq,Hash)]
pub enum Prompt {
    List,
    Get
}

impl From<&str> for Prompt {
    fn from(value: &str) -> Self {
        match value {
            "list" => {Self::List}
            "get" => {Self::Get}
            _ => {Self::List}
        }
    }
}
