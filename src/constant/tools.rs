
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tool {
    List
}

impl From<&str> for Tool {
    fn from(value: &str) -> Self {
        match value {
            "list" => {Self::List}
            _ => {Self::List}
        }
    }
}
