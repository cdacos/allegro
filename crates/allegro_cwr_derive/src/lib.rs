use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, LitInt, LitStr, parse_macro_input};

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

    let test_data = extract_test_data(&input.attrs).expect("CwrRecord requires #[cwr(test_data = \"...\")] attribute");
    let record_codes = extract_record_codes(&input.attrs, &name);
    let registry_variant = generate_registry_variant(&name);
    let field_parsers = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let (title, start, len, skip_parse) = extract_field_attrs(&field.attrs);

        let field_name_str = field_name.to_string();

        if field_name_str == "record_type" {
            // For record_type field, use the actual record type from the line
            quote! {
                let #field_name = line[0..3].to_string();
            }
        } else if skip_parse {
            quote! {
                let #field_name = <#field_type as Default>::default();
            }
        } else {
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
        }
    });

    let field_names = fields.iter().map(|f| &f.ident);
    let test_mod_name = quote::format_ident!("{}_generated_tests", name.to_string().to_lowercase());

    let _test_module = {
        quote! {
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
        }
    };

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

            /// Compatibility method for existing parser
            pub fn from_cwr_line(line: &str) -> Result<crate::error::CwrParseResult<Self>, crate::error::CwrParseError> {
                // Validate record type matches what we expect
                if line.len() < 3 {
                    return Err(crate::error::CwrParseError::BadFormat(
                        "Line too short to contain record type".to_string()
                    ));
                }

                let (record, warnings) = Self::parse(line);

                // Convert CwrWarning to String for compatibility
                let string_warnings: Vec<String> = warnings.into_iter()
                    .map(|w| format!("{}: {}", w.field_title, w.description))
                    .collect();

                // Check for critical errors
                let has_critical = string_warnings.iter().any(|w| w.contains("Critical"));
                if has_critical {
                    return Err(crate::error::CwrParseError::BadFormat(
                        string_warnings.join("; ")
                    ));
                }

                Ok(crate::error::CwrParseResult {
                    record,
                    warnings: string_warnings,
                })
            }

        }

        // Generate RecordType trait implementation
        impl crate::records::RecordType for #name {
            fn record_type(&self) -> &str {
                &self.record_type
            }
        }

        // Generate CwrRecord trait implementation
        impl crate::records::CwrRecord for #name {
            fn record_codes() -> &'static [&'static str] {
                #record_codes
            }

            fn from_cwr_line(line: &str) -> Result<crate::records::ParseResult<Self>, crate::error::CwrParseError> {
                // Validate record type matches what we expect
                if line.len() < 3 {
                    return Err(crate::error::CwrParseError::BadFormat(
                        "Line too short to contain record type".to_string()
                    ));
                }

                let (record, warnings) = Self::parse(line);

                // Convert CwrWarning to String for compatibility
                let string_warnings: Vec<String> = warnings.into_iter()
                    .map(|w| format!("{}: {}", w.field_title, w.description))
                    .collect();

                // Check for critical errors
                let has_critical = string_warnings.iter().any(|w| w.contains("Critical"));
                if has_critical {
                    return Err(crate::error::CwrParseError::BadFormat(
                        string_warnings.join("; ")
                    ));
                }

                Ok(crate::records::ParseResult {
                    record,
                    warnings: string_warnings,
                })
            }

            fn into_registry(self) -> crate::cwr_registry::CwrRegistry {
                #registry_variant
            }
        }

        #_test_module
    };

    TokenStream::from(expanded)
}

fn extract_test_data(attrs: &[syn::Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("cwr") {
            let result: Result<CwrAttribute, _> = attr.parse_args();
            if let Ok(cwr_attr) = result {
                if let Some(test_data) = cwr_attr.test_data {
                    return Some(test_data.value());
                }
            }
        }
    }
    None
}

fn extract_record_codes(attrs: &[syn::Attribute], name: &syn::Ident) -> quote::__private::TokenStream {
    // First check for explicit codes attribute
    for attr in attrs {
        if attr.path().is_ident("cwr") {
            let result: Result<CwrAttribute, _> = attr.parse_args();
            if let Ok(cwr_attr) = result {
                if let Some(codes) = cwr_attr.codes {
                    let code_strings: Vec<_> = codes.iter().map(|s| s.value()).collect();
                    return quote! { &[#(#code_strings),*] };
                }
            }
        }
    }

    // Fallback: infer from struct name
    // HdrRecord -> ["HDR"], SpuRecord -> ["SPU"], etc.
    let name_str = name.to_string();
    if let Some(prefix) = name_str.strip_suffix("Record") {
        let code = prefix.to_uppercase();
        return quote! { &[#code] };
    }

    panic!("Could not determine record codes for struct: {}", name_str);
}

fn generate_registry_variant(name: &syn::Ident) -> quote::__private::TokenStream {
    let name_str = name.to_string();
    if let Some(prefix) = name_str.strip_suffix("Record") {
        let variant_ident = quote::format_ident!("{}", prefix);
        return quote! { crate::cwr_registry::CwrRegistry::#variant_ident(self) };
    }

    panic!("Could not determine registry variant for struct: {}", name_str);
}

fn extract_field_attrs(attrs: &[syn::Attribute]) -> (String, usize, usize, bool) {
    for attr in attrs {
        if attr.path().is_ident("cwr") {
            let result: Result<CwrFieldAttribute, _> = attr.parse_args();
            if let Ok(field_attr) = result {
                return (field_attr.title.value(), field_attr.start.base10_parse().unwrap(), field_attr.len.base10_parse().unwrap(), field_attr.skip_parse);
            }
        }
    }
    panic!("Field requires #[cwr(title = \"...\", start = ..., len = ...)]");
}

struct CwrAttribute {
    test_data: Option<LitStr>,
    codes: Option<Vec<LitStr>>,
}

impl syn::parse::Parse for CwrAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut test_data = None;
        let mut codes = None;

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;

            if ident == "test_data" {
                test_data = Some(input.parse()?);
            } else if ident == "codes" {
                // Parse array of strings: ["HDR", "NWR", ...]
                let content;
                syn::bracketed!(content in input);
                let mut code_list = Vec::new();
                while !content.is_empty() {
                    code_list.push(content.parse()?);
                    if !content.is_empty() {
                        content.parse::<syn::Token![,]>()?;
                    }
                }
                codes = Some(code_list);
            } else {
                return Err(syn::Error::new(ident.span(), "Unknown attribute"));
            }

            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(CwrAttribute { test_data, codes })
    }
}

struct CwrFieldAttribute {
    title: LitStr,
    start: LitInt,
    len: LitInt,
    skip_parse: bool,
}

impl syn::parse::Parse for CwrFieldAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut title = None;
        let mut start = None;
        let mut len = None;
        let mut skip_parse = false;

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;

            match ident.to_string().as_str() {
                "title" => {
                    input.parse::<syn::Token![=]>()?;
                    title = Some(input.parse()?);
                }
                "start" => {
                    input.parse::<syn::Token![=]>()?;
                    start = Some(input.parse()?);
                }
                "len" => {
                    input.parse::<syn::Token![=]>()?;
                    len = Some(input.parse()?);
                }
                "skip_parse" => {
                    skip_parse = true;
                }
                _ => return Err(syn::Error::new(ident.span(), "Unknown field attribute")),
            }

            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(CwrFieldAttribute { title: title.ok_or_else(|| input.error("Missing 'title' attribute"))?, start: start.ok_or_else(|| input.error("Missing 'start' attribute"))?, len: len.ok_or_else(|| input.error("Missing 'len' attribute"))?, skip_parse })
    }
}
