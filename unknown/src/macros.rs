
    #[macro_export]
    macro_rules! String {
        ($($arg:tt)*) => {
            Unknown::String(format!($($arg)*))
        };
    }
    #[macro_export]
    macro_rules! Number {
        ($($arg:tt)*) => {
            Unknown::Number(($($arg)*) as i32)
        };
    }
    #[macro_export]
    macro_rules! Bool {
        ($arg:expr) => {
            Unknown::Bool($arg)
        };
    }
    #[macro_export]
    macro_rules! Array {
        ($($arg:expr),*) => {
           {
               let mut vec = vec![];
               $(
                   vec.push($arg.into_unknown());
               )*
               Unknown::Array(vec)
           }
        };
    }
    #[macro_export]
    macro_rules! Object {
        ($(
            $key:expr => $value:expr
        ),*) => {

            {
                let mut map = std::collections::HashMap::new();
                $(
                    map.insert($key.to_string(), $value.into_unknown());
                )*
                Unknown::Object(map)
            }
        };
    }
