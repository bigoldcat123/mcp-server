use std::{collections::HashMap, fmt::Display, hash::Hash};

use unknown::{String, Unknown};
#[derive(Debug)]
pub struct Error {
    msg:&'static str
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.msg)
    }
}
impl std::error::Error for Error{}

pub type ToolFn = Box<dyn Fn(Unknown) -> Result<Unknown,Error>  + 'static + Send + Sync>;
pub struct ToolDispatcher{
    tools:HashMap<String,ToolFn>
}
impl ToolDispatcher {
    pub fn new() -> Self {
        Self { tools:HashMap::new() }
    }
    pub fn register(&mut self, name:String, tool:ToolFn) {
        self.tools.insert(name, tool);
    }
}
#[cfg(test)]
mod test {
    use unknown::{IntoUnknown, String, Unknown};


    use crate::tool::Error;

    use super::ToolDispatcher;


#[test]
fn test_say_hello() {

    fn say_hello(name: &str) -> String {
        format!("Hello, {}!", name)
    }
    fn say_hello_wrapper(params:Unknown) -> Result<Unknown,Error> {
        let mut p = params.unwrap_as_map().ok_or(Error{msg:"Invalid parameters"})?;
        let name = p.remove("name").ok_or(Error{msg:"Missing 'name' parameter"})?.unwrap_as_string().ok_or(Error{msg:"Invalid 'name' parameter"})?;
        Ok(say_hello(name.as_str()).into_unknown())
    }
    let mut d = ToolDispatcher::new();
    d.register("name".to_string(), Box::new(say_hello_wrapper));
}
}
