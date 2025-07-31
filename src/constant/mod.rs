#![allow(unused)]
pub mod tools;
pub mod notification;
use notification::Notification;
use tools::Tool;

use crate::log;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RequestMethod {
    Initialize,
    Notifications(Notification),
    Tools(Tool),
}

impl From<&str> for RequestMethod {
    fn from(value: &str) -> Self {
        let (method,sub_method) = value.split_once("/").or(Some((value,""))).unwrap();
        match method {
            "initialize" => Self::Initialize,
            "notifications" => Self::Notifications(sub_method.into()),
            "tools" => Self::Tools(sub_method.into()),
            _ => {
                log("GGG");
                Self::Initialize
            }
        }
    }
}

mod test {
    use crate::constant::{tools::Tool, Notification, RequestMethod};

    #[test]
    fn test_notification_from_str() {
        assert_eq!(Notification::from("initialized"), Notification::Initialized);
    }

    #[test]
    fn test_request_method_from_str() {
        assert_eq!(RequestMethod::from("initialize"), RequestMethod::Initialize);
        assert_eq!(RequestMethod::from("notifications/initialized"), RequestMethod::Notifications(Notification::Initialized));
    }

    #[test]
    fn test_tool_from_str() {
        assert_eq!(Tool::from("list"), Tool::List);
    }

    #[test]
    fn test_tool_from_str_with_invalid_input() {
        assert_eq!(RequestMethod::from("tools/list"), RequestMethod::Tools(Tool::List));
    }
}
