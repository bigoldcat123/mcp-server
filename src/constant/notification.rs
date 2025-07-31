#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Notification {
    Initialized
}
impl From<&str> for Notification {
    fn from(value: &str) -> Self {
        match value {
            "initialized" => {
                Notification::Initialized
            }
            _ => {
                Notification::Initialized
            }
        }
    }
}
