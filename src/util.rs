use std::{collections::HashMap, fmt};

use serde::{de::{self, Visitor}, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
pub enum Unknown {
    String(String),
    Number(i32),
    Bool(bool),
    Object(HashMap<String, Unknown>),
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
    pub fn unwrap_as_bool(self) -> Option<bool> {
        match self {
            Unknown::Bool(b) => Some(b),
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
            Unknown::Object(m) => serializer.collect_map(m)
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
        }
        deserializer.deserialize_any(ObjectVisitor)
    }
}
