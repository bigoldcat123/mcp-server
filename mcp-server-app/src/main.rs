
use mcp_core::result::tools::{InputSchema, ToolDescription};
use mcp_server::McpServer;
use mcp_server_macro::tool;
use unknown::{IntoUnknown, Object, String, Unknown,Array,Number,ConvertError};

pub struct Stu {
    pub name:String,
    pub age:i32
}
impl TryFrom<Unknown> for Stu {
    type Error = ConvertError;
    fn try_from(value: Unknown) -> Result<Self, Self::Error> {
        let mut map = value.unwrap_as_map().ok_or(ConvertError{message:"nono"})?;
        let name = map.remove("name").ok_or(ConvertError { message: "e" })?.try_into()?;
        let age = map.remove("age").ok_or(ConvertError { message: "e" })?.try_into()?;
        Ok(Self {
            name,
            age
        })
    }
}
#[tool]
fn test2(name:String,age:i32,likes:Vec<i32>) -> String{
    format!("Name: {}, Age: {}, Likes: {:?}", name, age, likes)
}

#[tool]
fn t2(stu:Vec<Stu>) -> String {
    format!("Name: {}, Age: {}", stu[0].name, stu[0].age)
}

fn main() {


    let a = test2("John".into(), 30, vec![1,2]);
    let e =  test2_wrapper(Object!{
        "name" => String!("jhone"),
        "age" => Number!(30),
        "likes" => Array!(1i32,2i32,3i32,4i32)
    }).unwrap();
    println!("{}",e.unwrap_as_string().unwrap());

    let a = t2(vec![Stu { name: "()".into(), age: 22 }]);
    let a = t2_wrapper(Object!{
        "stu" => Array![Object!{
            "name" => String!("jhone"),
            "age" => Number!(30)
        }]
    }).unwrap().unwrap_as_string().unwrap();
    println!("{}",a);
    // let server = McpServer::builder()
    //     .tools(vec![
    //         ToolDescription::new("add", Some("title"), Some("description"), InputSchema::new(Some(Object!{
    //             "field_name" => Object!{
    //                 "type" => "string",
    //                 "description" => "Field description"
    //             }
    //         }), Some(vec![])), None, None)
    //     ]).build();
    // if let Err(e) = server.run() {
    //     eprintln!("Error: {}", e);
    // }
}
