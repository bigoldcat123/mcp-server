#[derive(Debug, Clone,Copy, PartialEq, Eq, Hash)]
pub enum Resource {
    List,
    Read
}


impl From<&str> for Resource {
    fn from(value: &str) -> Self {
        match value {
            "list" => {Self::List}
            "read" => {Self::Read}
            _ => {Self::List}
        }
    }
}
