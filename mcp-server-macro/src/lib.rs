use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, FnArg, Ident, ItemFn, Pat, PatType, Type, Error as SynError,
    Result as SynResult,
};

/// A procedural macro that generates wrapper functions for MCP tools.
/// 
/// This macro takes a function and generates a corresponding wrapper function
/// that can handle parameter conversion from `Unknown` types to the expected
/// function parameter types.
///
/// # Example
/// ```rust
/// #[tool]
/// pub fn my_tool(name: String, age: i32) -> String {
///     format!("Hello {}, you are {} years old", name, age)
/// }
/// ```
/// 
/// This will generate a wrapper function `my_tool_wrapper` that accepts
/// `Unknown` parameters and converts them to the appropriate types.
#[proc_macro_attribute]
pub fn tool(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    
    match generate_tool_wrapper(&input) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error().into(),
    }
}

/// Generate the tool wrapper function for the given input function
fn generate_tool_wrapper(input: &ItemFn) -> SynResult<TokenStream> {
    let vis = &input.vis;
    let name = &input.sig.ident;
    let wrapper_name = create_wrapper_name(name);
    
    let parameters = extract_function_parameters(input)?;
    let param_parsing = generate_parameter_parsing(&parameters)?;
    let param_names = generate_parameter_names(&parameters);
    
    let expanded = quote! {
        #input
        
        #vis fn #wrapper_name(params: Unknown) -> Result<Unknown, unknown::ConvertError> {
            let mut p = params.unwrap_as_map()
                .ok_or(ConvertError { message: "Expected parameters as map" })?;
            
            #param_parsing
            
            Ok(#name(#param_names).into_unknown())
        }
    };
    
    Ok(TokenStream::from(expanded))
}

/// Create the wrapper function name by appending "_wrapper" to the original function name
fn create_wrapper_name(name: &Ident) -> Ident {
    Ident::new(&format!("{}_wrapper", name), name.span())
}

/// Represents a function parameter with its name and type information
#[derive(Debug)]
struct Parameter {
    name: String,
    ident: Ident,
    type_info: TypeInfo,
}

/// Information about the parameter type and how it should be handled
#[derive(Debug)]
enum TypeInfo {
    Reference(Box<TypeInfo>),
    Vector(Box<TypeInfo>),
    Simple(String),
}

/// Extract and parse all function parameters
fn extract_function_parameters(input: &ItemFn) -> SynResult<Vec<Parameter>> {
    let mut parameters = Vec::new();
    
    for arg in &input.sig.inputs {
        match arg {
            FnArg::Receiver(_) => {
                return Err(SynError::new_spanned(
                    arg,
                    "Tool functions cannot have self parameters"
                ));
            }
            FnArg::Typed(pat_type) => {
                let parameter = parse_typed_parameter(pat_type)?;
                parameters.push(parameter);
            }
        }
    }
    
    Ok(parameters)
}

/// Parse a typed function parameter
fn parse_typed_parameter(pat_type: &PatType) -> SynResult<Parameter> {
    let name = extract_parameter_name(&pat_type.pat)?;
    let ident = Ident::new(&name, Span::call_site());
    let type_info = parse_type_info(&pat_type.ty)?;
    
    Ok(Parameter {
        name,
        ident,
        type_info,
    })
}

/// Extract the parameter name from a pattern
fn extract_parameter_name(pat: &Pat) -> SynResult<String> {
    match pat {
        Pat::Ident(pat_ident) => Ok(pat_ident.ident.to_string()),
        _ => Err(SynError::new_spanned(
            pat,
            "Only simple parameter names are supported"
        )),
    }
}

/// Parse type information from a syn::Type
fn parse_type_info(ty: &Type) -> SynResult<TypeInfo> {
    let type_string = ty.to_token_stream().to_string();
    
    if type_string.starts_with("&") {
        let inner_type = parse_reference_type(&type_string)?;
        return Ok(TypeInfo::Reference(Box::new(inner_type)));
    }
    
    if type_string.starts_with("Vec") {
        let inner_type = parse_vector_type(&type_string)?;
        return Ok(TypeInfo::Vector(Box::new(inner_type)));
    }
    
    Ok(TypeInfo::Simple(type_string))
}

/// Parse reference type (e.g., "&str", "&Vec<String>")
fn parse_reference_type(type_string: &str) -> SynResult<TypeInfo> {
    let inner_type_str = type_string.strip_prefix("&")
        .unwrap_or(type_string)
        .trim();
    
    if inner_type_str.starts_with("Vec") {
        let inner_type = parse_vector_type(inner_type_str)?;
        Ok(inner_type)
    } else {
        Ok(TypeInfo::Simple(inner_type_str.to_string()))
    }
}

/// Parse vector type (e.g., "Vec<String>", "Vec<i32>")
fn parse_vector_type(type_string: &str) -> SynResult<TypeInfo> {
    // Extract the inner type from Vec<T>
    if let Some(start) = type_string.find('<') {
        if let Some(end) = type_string.rfind('>') {
            let inner_type_str = &type_string[start + 1..end].trim();
            return Ok(TypeInfo::Vector(Box::new(TypeInfo::Simple(inner_type_str.to_string()))));
        }
    }
    
    // Fallback for simple Vec without explicit type
    Ok(TypeInfo::Vector(Box::new(TypeInfo::Simple("Unknown".to_string()))))
}

/// Generate parameter parsing code for all parameters
fn generate_parameter_parsing(parameters: &[Parameter]) -> SynResult<proc_macro2::TokenStream> {
    let mut parsing_code = quote! {};
    
    for param in parameters {
        let param_parsing = generate_single_parameter_parsing(param)?;
        parsing_code = quote! {
            #parsing_code
            #param_parsing
        };
    }
    
    Ok(parsing_code)
}

/// Generate parsing code for a single parameter
fn generate_single_parameter_parsing(param: &Parameter) -> SynResult<proc_macro2::TokenStream> {
    let name = &param.name;
    let ident = &param.ident;
    
    match &param.type_info {
        TypeInfo::Reference(_) => {
            Ok(quote! {
                let #ident = p.get(#name)
                    .ok_or(ConvertError { message: concat!("Missing parameter: ", #name) })?
                    .try_into()?;
            })
        }
        TypeInfo::Vector(_) => {
            Ok(quote! {
                let #ident = p.remove(#name)
                    .ok_or(ConvertError { message: concat!("Missing parameter: ", #name) })?
                    .try_into()
                    .and_then(|x: Vec<Unknown>| {
                        x.into_iter()
                            .map(|item| item.try_into())
                            .collect::<Result<Vec<_>, _>>()
                            .map_err(|_| ConvertError { message: concat!("Invalid vector parameter: ", #name) })
                    })?;
            })
        }
        TypeInfo::Simple(_) => {
            Ok(quote! {
                let #ident = p.remove(#name)
                    .ok_or(ConvertError { message: concat!("Missing parameter: ", #name) })?
                    .try_into()?;
            })
        }
    }
}

/// Generate the parameter names for the function call
fn generate_parameter_names(parameters: &[Parameter]) -> proc_macro2::TokenStream {
    let names = parameters.iter().map(|p| &p.ident);
    quote! { #(#names),* }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn test_create_wrapper_name() {
        let name = Ident::new("test_function", Span::call_site());
        let wrapper = create_wrapper_name(&name);
        assert_eq!(wrapper.to_string(), "test_function_wrapper");
    }

    #[test]
    fn test_parse_simple_type() {
        let ty: Type = parse_quote!(String);
        let type_info = parse_type_info(&ty).unwrap();
        match type_info {
            TypeInfo::Simple(s) => assert_eq!(s, "String"),
            _ => panic!("Expected Simple type"),
        }
    }

    #[test]
    fn test_parse_reference_type() {
        let ty: Type = parse_quote!(&str);
        let type_info = parse_type_info(&ty).unwrap();
        match type_info {
            TypeInfo::Reference(inner) => match *inner {
                TypeInfo::Simple(s) => assert_eq!(s, "str"),
                _ => panic!("Expected Simple inner type"),
            },
            _ => panic!("Expected Reference type"),
        }
    }

    #[test]
    fn test_parse_vector_type() {
        let ty: Type = parse_quote!(Vec<String>);
        let type_info = parse_type_info(&ty).unwrap();
        match type_info {
            TypeInfo::Vector(_) => {},
            _ => panic!("Expected Vector type"),
        }
    }
}