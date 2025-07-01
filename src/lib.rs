use std::collections::HashMap;

///``` typescript
/// export interface Request {
///     method: string;
///     params?: {
///         _meta?: {
///             progressToken?: ProgressToken;
///             [key: string]: unknown;
///         };
///         [key: string]: unknown;
///     };
/// }
///```
macro_rules! request {
    (struct $name:ident { $($filed_name:ident:$type:ty),* }) => {
        struct $name {
            method: String,
            params:Option<Params>,
            $(
                $filed_name:$type,
            )*
        }
    };
}

struct Params {
    _meta: Option<HashMap<String, String>>,
}

request!(
    struct Hello {
        asd: String
    }
);
