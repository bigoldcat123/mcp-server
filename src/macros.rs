
#[macro_export]
macro_rules! Package {
    (pub struct $name:ident { $($filed_name:ident:$type:ty),* }) => {
        #[derive(Serialize,Deserialize,Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            jsonrpc: String,
            method: String,
            $(
                $filed_name:$type,
            )*
        }
    };
}
#[macro_export]
macro_rules! Result {
    (pub struct $name:ident { $($filed_name:ident:$type:ty),* }) => {
        #[derive(Serialize,Deserialize,Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            jsonrpc: String,
            id: i32,
            $(
                $filed_name:$type,
            )*
        }
    };
}

#[macro_export]
macro_rules! Request {
    (pub struct $name:ident { $($filed_name:ident:$type:ty),* }) => {
        Package!(
            pub struct $name {
                id:i32,
                $(
                    $filed_name: $type
                )*
            }
        );
    };
}


#[macro_export]
macro_rules! BaseMetadata {
    (pub struct $name:ident { $( $filed_name:ident:$type:ty ),* }) => {
        #[derive(Deserialize,Serialize,Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            name:String,
            title:Option<String>,
            $(
                $filed_name:$type,
            )*
        }
    };
}
