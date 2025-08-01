use std::{fs::File, io::Read};

use mcp_server::{request::{init::InitializeRequest, CommonRequest}, result::{InitializeResult, ServerCapabilities}, util::Unknown, Implementation};
use serde::{Deserialize, Serialize};

#[test]
fn test_json() {
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct Stu {
        name: Unknown,
        home_number: i32,
    }
    let stu = Stu {
        name: Unknown::Number(111),
        home_number: 111,
    };
    let res = serde_json::to_string(&stu).unwrap();
    println!("{}", res);
    let e = serde_json::from_str::<Stu>(&res.as_str()).unwrap();
    println!("{:?}", e);
}

#[test]
fn test_de() {
    let mut s = String::new();
    let _ = File::open("/Users/dadigua/Desktop/mcp-server/log.json")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    println!("{:?}", s);
    let e = serde_json::from_str::<CommonRequest>(&s).unwrap();
    let e = InitializeRequest::try_from(e).unwrap();
    println!("{:?}",e);
}


#[test]
fn seri_result() {
    let req = InitializeResult::new("2.0".to_string(), 1, "2025-06-18".to_string(), ServerCapabilities::new(None, None, None, None, None), Implementation::new("ExampleServer".to_string(), Some("Example Server Display Name".to_string()), "222.222".to_string()), Some("this is a instruction!".to_string()));
    let res = serde_json::to_string_pretty(&req).unwrap();
    println!("{}", res);
}

#[test]
fn test_object_map_deserialization() {
    // Test deserializing a JSON object into Object::Map
    let json_str = r#"{"name": "John", "age": 30, "nested": {"city": "New York", "zip": 10001}}"#;
    let obj: Unknown = serde_json::from_str(json_str).unwrap();

    match obj {
        Unknown::Object(map) => {
            assert_eq!(map.len(), 3);

            // Check string value
            if let Some(Unknown::String(name)) = map.get("name") {
                assert_eq!(name, "John");
            } else {
                panic!("Expected name to be a string");
            }

            // Check number value
            if let Some(Unknown::Number(age)) = map.get("age") {
                assert_eq!(*age, 30);
            } else {
                panic!("Expected age to be a number");
            }

            // Check nested map
            if let Some(Unknown::Object(nested)) = map.get("nested") {
                assert_eq!(nested.len(), 2);

                if let Some(Unknown::String(city)) = nested.get("city") {
                    assert_eq!(city, "New York");
                } else {
                    panic!("Expected city to be a string");
                }

                if let Some(Unknown::Number(zip)) = nested.get("zip") {
                    assert_eq!(*zip, 10001);
                } else {
                    panic!("Expected zip to be a number");
                }
            } else {
                panic!("Expected nested to be a map");
            }
        }
        _ => panic!("Expected Object::Map but got {:?}", obj),
    }

    println!("Object::Map deserialization test passed!");
}

#[test]
fn test_object_map_roundtrip() {
    use std::collections::HashMap;

    // Create a complex Object::Map structure
    let mut inner_map = HashMap::new();
    inner_map.insert("city".to_string(), Unknown::String("New York".to_string()));
    inner_map.insert("population".to_string(), Unknown::Number(8000000));

    let mut outer_map = HashMap::new();
    outer_map.insert("name".to_string(), Unknown::String("John".to_string()));
    outer_map.insert("age".to_string(), Unknown::Number(30));
    outer_map.insert("location".to_string(), Unknown::Object(inner_map));

    let original = Unknown::Object(outer_map);

    // Serialize to JSON
    let json_str = serde_json::to_string(&original).unwrap();
    println!("Serialized: {}", json_str);

    // Deserialize back from JSON
    let deserialized: Unknown = serde_json::from_str(&json_str).unwrap();

    // Verify the structure is preserved
    match deserialized {
        Unknown::Object(map) => {
            assert_eq!(map.len(), 3);

            // Check all values are correct
            if let Some(Unknown::String(name)) = map.get("name") {
                assert_eq!(name, "John");
            } else {
                panic!("Expected name to be a string");
            }

            if let Some(Unknown::Number(age)) = map.get("age") {
                assert_eq!(*age, 30);
            } else {
                panic!("Expected age to be a number");
            }

            if let Some(Unknown::Object(location)) = map.get("location") {
                assert_eq!(location.len(), 2);

                if let Some(Unknown::String(city)) = location.get("city") {
                    assert_eq!(city, "New York");
                } else {
                    panic!("Expected city to be a string");
                }

                if let Some(Unknown::Number(population)) = location.get("population") {
                    assert_eq!(*population, 8000000);
                } else {
                    panic!("Expected population to be a number");
                }
            } else {
                panic!("Expected location to be a map");
            }
        }
        _ => panic!("Expected Object::Map but got {:?}", deserialized),
    }

    println!("Object::Map roundtrip test passed!");
}



#[test]
fn from_common_to_specific() {
    let mut s = String::new();
    File::open("/Users/dadigua/Desktop/mcp-server/log.json").unwrap().read_to_string(&mut s).unwrap();
    let c = serde_json::from_str::<Unknown>(&s).unwrap();
    let x:CommonRequest = c.try_into().unwrap();
    let x:InitializeRequest = x.try_into().unwrap();
    println!("{:#?}",x);
}
