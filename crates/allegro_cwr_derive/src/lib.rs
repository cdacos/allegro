use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, LitStr, LitInt};

#[proc_macro_derive(CwrRecord, attributes(cwr))]
pub fn derive_cwr_record(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let name = &input.ident;
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("CwrRecord only supports structs with named fields"),
        },
        _ => panic!("CwrRecord can only be derived for structs"),
    };
    
    let test_data = extract_test_data(&input.attrs);
    let field_parsers = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let (title, start, len) = extract_field_attrs(&field.attrs);
        
        quote! {
            let (#field_name, field_warnings) = {
                let end = #start + #len;
                if line.len() < end {
                    let mut warnings = vec![
                        CwrWarning {
                            field_name: stringify!(#field_name),
                            field_title: #title,
                            source_str: std::borrow::Cow::Borrowed(""),
                            level: WarningLevel::Critical,
                            description: format!(
                                "Line too short: expected at least {} characters, got {}", 
                                end, 
                                line.len()
                            ),
                        }
                    ];
                    let default_value = <#field_type as Default>::default();
                    (default_value, warnings)
                } else {
                    let field_slice = &line[#start..end];
                    <#field_type as CwrFieldParse>::parse_cwr_field(
                        field_slice,
                        stringify!(#field_name),
                        #title
                    )
                }
            };
            warnings.extend(field_warnings);
        }
    });
    
    let field_names = fields.iter().map(|f| &f.ident);
    let test_mod_name = quote::format_ident!("{}_generated_tests", name.to_string().to_lowercase());
    
    let expanded = quote! {
        impl #name {
            pub fn parse(line: &str) -> (Self, Vec<CwrWarning<'static>>) {
                let mut warnings = Vec::new();
                
                #(#field_parsers)*
                
                let record = Self {
                    #(#field_names,)*
                };
                
                (record, warnings)
            }
        }
        
        #[cfg(test)]
        mod #test_mod_name {
            use super::*;
            
            #[test]
            fn test_parse_from_test_data() {
                let test_line = #test_data;
                let (record, warnings) = #name::parse(test_line);
                
                for warning in &warnings {
                    eprintln!("Warning in {:?}: {:?}", warning.field_name, warning.description);
                }
                
                assert!(
                    warnings.iter().all(|w| !w.is_critical()),
                    "Critical warnings found in test data"
                );
            }
        }
    };
    
    TokenStream::from(expanded)
}

fn extract_test_data(attrs: &[syn::Attribute]) -> String {
    for attr in attrs {
        if attr.path().is_ident("cwr") {
            let result: Result<CwrAttribute, _> = attr.parse_args();
            if let Ok(cwr_attr) = result {
                if let Some(test_data) = cwr_attr.test_data {
                    return test_data.value();
                }
            }
        }
    }
    panic!("CwrRecord requires #[cwr(test_data = \"...\")] attribute on the struct");
}

fn extract_field_attrs(attrs: &[syn::Attribute]) -> (String, usize, usize) {
    for attr in attrs {
        if attr.path().is_ident("cwr") {
            let result: Result<CwrFieldAttribute, _> = attr.parse_args();
            if let Ok(field_attr) = result {
                return (
                    field_attr.title.value(),
                    field_attr.start.base10_parse().unwrap(),
                    field_attr.len.base10_parse().unwrap(),
                );
            }
        }
    }
    panic!("Field requires #[cwr(title = \"...\", start = ..., len = ...)]");
}

struct CwrAttribute {
    test_data: Option<LitStr>,
}

impl syn::parse::Parse for CwrAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut test_data = None;
        
        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;
            
            if ident == "test_data" {
                test_data = Some(input.parse()?);
            } else {
                return Err(syn::Error::new(ident.span(), "Unknown attribute"));
            }
            
            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }
        
        Ok(CwrAttribute { test_data })
    }
}

struct CwrFieldAttribute {
    title: LitStr,
    start: LitInt,
    len: LitInt,
}

impl syn::parse::Parse for CwrFieldAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut title = None;
        let mut start = None;
        let mut len = None;
        
        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;
            
            match ident.to_string().as_str() {
                "title" => title = Some(input.parse()?),
                "start" => start = Some(input.parse()?),
                "len" => len = Some(input.parse()?),
                _ => return Err(syn::Error::new(ident.span(), "Unknown field attribute")),
            }
            
            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }
        
        Ok(CwrFieldAttribute {
            title: title.ok_or_else(|| input.error("Missing 'title' attribute"))?,
            start: start.ok_or_else(|| input.error("Missing 'start' attribute"))?,
            len: len.ok_or_else(|| input.error("Missing 'len' attribute"))?,
        })
    }
}