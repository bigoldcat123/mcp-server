
#[macro_export]
macro_rules! Package {
    ($visibility:vis struct $name:ident { $(
        $(#[$field_meta:meta])*
      $field_visibility:vis  $filed_name:ident:$type:ty
    ),* }) => {
        #[derive(Serialize,Deserialize,Debug)]
        #[serde(rename_all = "camelCase")]
        $visibility struct $name {
            pub jsonrpc: String,
            pub method: String,
            $(
                $(#[$field_meta])*
               $field_visibility $filed_name:$type,
            )*
        }
    };
}
#[macro_export]
macro_rules! Result {
    ( $visibility:vis struct $name:ident { $(
        $(#[$field_meta:meta])*
        $field_visibility:vis $filed_name:ident:$type:ty),* }) => {
        #[derive(Serialize,Deserialize,Debug)]
        #[serde(rename_all = "camelCase")]
        $visibility struct $name {
            jsonrpc: String,
            id: i32,
            $(
                $(#[$field_meta])*
                $field_visibility $filed_name:$type,
            )*
        }
    };
}

#[macro_export]
macro_rules! Request {
    ($visibility:vis struct $name:ident {
        $(
            $(#[$field_meta:meta])*
            $field_visibility:vis $filed_name:ident:$type:ty
        ),*
    }) => {
        Package!(
            $visibility struct $name {
                pub id:i32,
                $(
                    $(#[$field_meta])*
                    $field_visibility $filed_name: $type
                )*
            }
        );
    };
}


#[macro_export]
macro_rules! BaseMetadata {
    (pub struct $(#[$struct_meta:meta])* $name:ident {
        $(
            $(#[$field_meta:meta])*
            $filed_name:ident:$type:ty
        ),*
    }) => {
        #[derive(Deserialize,Serialize,Debug)]
        #[serde(rename_all = "camelCase")]
        $(#[$struct_meta])*
        pub struct $name {
            name:String,
            title:Option<String>,
            $(
                $(#[$field_meta])*
                $filed_name:$type,
            )*
        }
    };
}


macro_rules! testaa {
    ( $vis:vis struct $name:ident  {
        $(
            $(#[$meta:meta])*
            $filed_name:ident:$type:ty
        ),*
    }) => {
        $vis struct $name {
            $(
                $(#[$meta])*
                $filed_name:$type,
            )*
        }
    };
}
