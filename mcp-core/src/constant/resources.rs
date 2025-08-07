#[derive(Debug, Clone,Copy, PartialEq, Eq, Hash)]
pub enum Resource {
    List
}


impl From<&str> for Resource {
    fn from(value: &str) -> Self {
        match value {
            "list" => {Self::List}
            _ => {Self::List}
        }
    }
}
