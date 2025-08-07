#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Notification {
    Initialized,
    Cancelled
}
impl From<&str> for Notification {
    fn from(value: &str) -> Self {
        match value {
            "initialized" => {
                Notification::Initialized
            }
            "cancelled" => {
                Notification::Cancelled
            }
            _ => {
                Notification::Initialized
            }
        }
    }
}
