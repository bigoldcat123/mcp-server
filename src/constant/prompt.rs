#[derive(Debug,Clone, Copy,PartialEq, Eq,Hash)]
pub enum Prompt {
    List
}

impl From<&str> for Prompt {
    fn from(value: &str) -> Self {
        match value {
            "list" => {Self::List}
            _ => {Self::List}
        }
    }
}
