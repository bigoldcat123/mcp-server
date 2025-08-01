
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tool {
    List,
    Call
}

impl From<&str> for Tool {
    fn from(value: &str) -> Self {
        match value {
            "list" => {Self::List}
            "call" => {Self::Call}
            _ => {
                unreachable!()
            }
        }
    }
}
