pub mod macros;

use std::{collections::HashMap, fmt};

use serde::{de::{self, Visitor}, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
pub enum Unknown {
    String(String),
    Number(i32),
    Float(f64),
    Bool(bool),
    Object(HashMap<String, Unknown>),
    Array(Vec<Unknown>)
}
pub trait IntoUnknown {
     fn into_unknown(self) -> Unknown;
}
impl IntoUnknown for String {
    fn into_unknown(self) -> Unknown {
        Unknown::String(self)
    }
}
impl IntoUnknown for &str {
    fn into_unknown(self) -> Unknown {
        Unknown::String(self.to_string())
    }
}
impl IntoUnknown for i32 {
    fn into_unknown(self) -> Unknown {
        Unknown::Number(self)
    }
}
impl IntoUnknown for f64 {
    fn into_unknown(self) -> Unknown {
        Unknown::Float(self)
    }
}
impl IntoUnknown for bool {
    fn into_unknown(self) -> Unknown {
        Unknown::Bool(self)
    }
}
impl IntoUnknown for HashMap<String, Unknown> {
    fn into_unknown(self) -> Unknown {
        Unknown::Object(self)
    }
}
impl IntoUnknown for Vec<Unknown> {
    fn into_unknown(self) -> Unknown {
        Unknown::Array(self)
    }
}
impl IntoUnknown for Unknown {
    fn into_unknown(self) -> Unknown {
        self
    }
}
impl Unknown {
    pub fn unwrap_as_map(self) -> Option<HashMap<String, Unknown>> {
        match self {
            Unknown::Object(map) => Some(map),
            _ => None,
        }
    }
    pub fn unwrap_as_string(self) -> Option<String> {
        match self {
            Unknown::String(s) => Some(s),
            _ => None,
        }
    }
    pub fn unwrap_as_number(self) -> Option<i32> {
        match self {
            Unknown::Number(n) => Some(n),
            _ => None,
        }
    }
    pub fn unwrap_as_float(self) -> Option<f64> {
        match self {
            Unknown::Float(n) => Some(n),
            _ => None,
        }
    }
    pub fn unwrap_as_bool(self) -> Option<bool> {
        match self {
            Unknown::Bool(b) => Some(b),
            _ => None,
        }
    }
    pub fn unwrap_as_array(self) -> Option<Vec<Unknown>> {
        match self {
            Unknown::Array(arr) => Some(arr),
            _ => None,
        }
    }
}

// 自定义序列化
impl Serialize for Unknown {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Unknown::String(s) => serializer.serialize_str(s),
            Unknown::Number(n) => serializer.serialize_i32(*n),
            Unknown::Bool(b) => serializer.serialize_bool(*b),
            Unknown::Object(m) => serializer.collect_map(m),
            Unknown::Array(a) => serializer.collect_seq(a),
            Unknown::Float(f) => serializer.serialize_f64(*f),
        }
    }
}

// 自定义反序列化
impl<'de> Deserialize<'de> for Unknown {
    fn deserialize<D>(deserializer: D) -> Result<Unknown, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObjectVisitor;

        impl<'de> Visitor<'de> for ObjectVisitor {
            type Value = Unknown;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string, an integer, or a map")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Unknown::String(v.to_owned()))
            }
            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Unknown::Bool(v))
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Unknown::Number(v as i32)) // 注意：可能要检查是否溢出
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Unknown::Number(v as i32)) // 注意：可能要检查是否溢出
            }
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Unknown::Float(v))
            }
            fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Unknown::Float(v as f64))
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut hash_map = HashMap::new();

                while let Some((key, value)) = map.next_entry::<String, Unknown>()? {
                    hash_map.insert(key, value);
                }

                Ok(Unknown::Object(hash_map))
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut vec = Vec::new();

                while let Some(value) = seq.next_element()? {
                    vec.push(value);
                }

                Ok(Unknown::Array(vec))
            }
        }
        deserializer.deserialize_any(ObjectVisitor)
    }
}


#[cfg(test)]
mod test {
    use super::{Array, Bool, Number, Object, String};
    use crate::Unknown;
    use crate::IntoUnknown;

    #[test]
    fn test_str () {

        let x = "example";
        let a = String!("hello {} {} {x}", "asd", x);
        println!("{:?}",a);
        let a = Number!({100 + 20 - 29 * 29 + {20}} - {
            if 1 == 0 {
                20
            }else {
                20
            }
        });
        println!("{:?}",a);
        let a = Array!(1i32,2i32,"as");
        println!("{:?}",a);
        let a = Bool!(1 == 1);
        println!("{:?}",a);

        let x = String::from("eee");
        let a = Object! {
            x => "John",
            "age" => 30,
            "city" => "New York",
            "house" => Object!(x => "eee", "y" => "yyy", "z" => "zzz", "w" => "www")
        };
        println!("{:?}",a);

    }
}
