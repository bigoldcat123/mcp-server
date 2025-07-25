use std::fmt;

use serde::{de::{self, Visitor}, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
pub enum Object {
    String(String),
    Number(i32),
}

// 自定义序列化
impl Serialize for Object {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Object::String(s) => serializer.serialize_str(s),
            Object::Number(n) => serializer.serialize_i32(*n),
        }
    }
}

// 自定义反序列化
impl<'de> Deserialize<'de> for Object {
    fn deserialize<D>(deserializer: D) -> Result<Object, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObjectVisitor;

        impl<'de> Visitor<'de> for ObjectVisitor {
            type Value = Object;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string or an integer")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Object::String(v.to_owned()))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Object::Number(v as i32)) // 注意：可能要检查是否溢出
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Object::Number(v as i32)) // 注意：可能要检查是否溢出
            }
        }

        deserializer.deserialize_any(ObjectVisitor)
    }
}
