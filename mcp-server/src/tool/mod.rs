use std::{collections::HashMap, fmt::Display};

use unknown::{ConvertError, Unknown};


pub type ToolFn = Box<dyn Fn(Unknown) -> Result<Unknown,ConvertError>  + 'static + Send + Sync>;
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
    use unknown::{ConvertError, IntoUnknown, Unknown};




    use super::ToolDispatcher;


#[test]
fn test_say_hello() {

    fn say_hello(name: Vec<String>) -> String {
        format!("Hello, {:?}!", name)
    }
    fn say_hello_wrapper(params:Unknown) -> Result<Unknown,ConvertError> {
        let mut p = params.unwrap_as_map().ok_or(ConvertError{message:"Invalid parameters"})?;
        // let name = p.remove("name").ok_or(Error{msg:"Missing 'name' parameter"})?.unwrap_as_string().ok_or(Error{msg:"Invalid 'name' parameter"})?;
        // let name = p.remove("name ").ok_or(ConvertError { message : "Invalid parameters" })?.try_into()?;
        let likes =
        p.remove("likes ").ok_or(ConvertError { message : "Invalid parameters" }) ?
        .try_into().and_then(| x : Vec < Unknown > |
        {
            let e =
            x.into_iter().map(| x |
            x.try_into().expect("Invalid parameter")).collect(); Ok(e)
        }) ? ;
        Ok(say_hello(likes,).into_unknown())
    }
    let mut d = ToolDispatcher::new();
    d.register("name".to_string(), Box::new(say_hello_wrapper));
}
}
