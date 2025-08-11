#[macro_export]
macro_rules! ResourceContents {
    ( $p:vis struct $name:ident {

        $(
            $(#[$field_meta:meta])*
            $field_vis:vis $field:ident: $type:ty
        ),*
    } ) => {
        #[derive(Debug,Serialize,Deserialize)]
        #[serde(rename_all = "camelCase")]
        $p struct $name {
            uri:String,
            #[serde(skip_serializing_if = "Option::is_none")]
            mime_type:Option<String>,
            _meta:Option<HashMap<String,Unknown>>,
            $(
                $(#[$field_meta])*
                $field:$type,
            )*
        }
    };
}
