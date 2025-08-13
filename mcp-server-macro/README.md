# MCP Server Macro

A procedural macro for generating MCP (Model Context Protocol) tool wrapper functions that handle parameter conversion from `Unknown` types to strongly-typed function parameters.

## Overview

The `#[tool]` macro automatically generates wrapper functions that can convert loosely-typed parameters into the expected function parameter types, making it easy to create MCP tools with type safety.

## Usage

```rust
use mcp_server_macro::tool;
use unknown::{Unknown, IntoUnknown, ConvertError};

#[tool]
pub fn my_tool(name: String, age: i32, tags: Vec<String>) -> String {
    format!("Hello {}, age {}, tags: {:?}", name, age, tags)
}

// The macro generates a wrapper function called `my_tool_wrapper`
// that can be called with Unknown parameters:
let result = my_tool_wrapper(Object!{
    "name" => String!("Alice"),
    "age" => Number!(25),
    "tags" => Array!("rust", "programming")
}).unwrap();
```

## Supported Parameter Types

The macro supports the following parameter types:

### Simple Types
- `String`, `i32`, `f64`, etc. - Any type that implements `TryFrom<Unknown>`
- Custom structs that implement `TryFrom<Unknown>`

### Reference Types
- `&str`, `&String`, etc. - References are handled by borrowing from the parameter map

### Vector Types
- `Vec<T>` where `T` implements `TryFrom<Unknown>`
- Each element in the vector is individually converted

## Generated Code

For a function like:
```rust
#[tool]
fn example(name: String, count: i32, items: Vec<String>) -> String {
    // function implementation
}
```

The macro generates:
```rust
fn example(name: String, count: i32, items: Vec<String>) -> String {
    // original function implementation
}

fn example_wrapper(params: Unknown) -> Result<Unknown, ConvertError> {
    let mut p = params.unwrap_as_map()
        .ok_or(ConvertError { message: "Expected parameters as map" })?;
    
    let name = p.remove("name")
        .ok_or(ConvertError { message: "Missing parameter: name" })?
        .try_into()?;
    
    let count = p.remove("count")
        .ok_or(ConvertError { message: "Missing parameter: count" })?
        .try_into()?;
    
    let items = p.remove("items")
        .ok_or(ConvertError { message: "Missing parameter: items" })?
        .try_into()
        .and_then(|x: Vec<Unknown>| {
            x.into_iter()
                .map(|item| item.try_into())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| ConvertError { message: "Invalid vector parameter: items" })
        })?;
    
    Ok(example(name, count, items).into_unknown())
}
```

## Error Handling

The macro provides clear error messages for common issues:

- **Missing parameters**: "Missing parameter: {parameter_name}"
- **Invalid parameter map**: "Expected parameters as map"
- **Vector conversion errors**: "Invalid vector parameter: {parameter_name}"

## Limitations

- Functions with `self` parameters are not supported
- Only simple parameter patterns are supported (no destructuring)
- Complex generic types may require manual implementation of `TryFrom<Unknown>`

## Architecture

The refactored macro follows these principles:

1. **Separation of Concerns**: Logic is split into focused helper functions
2. **Proper Error Handling**: Uses `syn::Result` for compile-time errors
3. **AST-based Parsing**: Avoids fragile string manipulation
4. **Extensibility**: Easy to add support for new parameter types
5. **Testing**: Includes unit tests for core functionality

## Migration from Previous Version

The refactored macro is backward compatible. No changes are required for existing `#[tool]` annotated functions.

## Testing

The macro includes comprehensive tests for:
- Wrapper name generation
- Type parsing (simple, reference, vector types)
- Parameter extraction
- Error cases

Run tests with:
```bash
cargo test -p mcp-server-macro
```
