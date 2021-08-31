use darling::FromField;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields, GenericArgument, Ident, PathArguments, Token, Type};

use crate::borrowed::generate_parser_impl;

mod borrowed;

type StructFields = Punctuated<syn::Field, Token![,]>;

#[derive(Debug, FromField)]
struct FieldKind {
    pub br_code_id: String,
    pub name: syn::Ident,
    pub(crate) kind: Type,
    pub(crate) args: EncodeArgs,
}

#[derive(Debug, FromField)]
#[darling(attributes(encoder))]
struct EncodeArgs {
    id: String,
    #[darling(default)]
    default: Option<String>,
}

#[proc_macro_derive(EmvEncoder, attributes(encoder))]
/// Serialize each field with the proper annotated ID, and when serialized, outputs the size of its field in bytes.
pub fn derive_helper_attr(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let fields = get_struct_fields(&input);
    let struct_name = &input.ident;

    match &input.data {
        Data::Struct(_) => {}
        _ => {
            return Error::new(struct_name.span(), "Derive only available for structs.")
                .to_compile_error()
                .into()
        }
    };

    let fields = fields.unwrap();

    let field: Vec<_> = fields
        .iter()
        .map(|field| {
            let args = EncodeArgs::from_field(field).unwrap();
            let code_id = args.id.clone();
            let ident = field.ident.as_ref().unwrap();

            FieldKind {
                args,
                br_code_id: code_id,
                name: ident.clone(),
                kind: field.ty.clone(),
            }
        })
        .collect();

    let push_output_tokens = field.iter().map(format_serializer_token).collect::<Vec<_>>();
    let parser_impl = generate_parser_impl(struct_name, &*field);

    let output = quote! {

        impl<'a> #struct_name<'a> {

            fn serialize(&self) -> String {
                use emv_qrcps::{Size, Encode};

                let mut output = String::with_capacity(150);
                #(#push_output_tokens)*
                output
            }

            pub fn serialize_with_src(&self) -> String {
                use emv_qrcps::{Size, Encode};

                let mut output = String::with_capacity(150);
                #(#push_output_tokens)*
                output.push_str("6304");
                let crc = emv_qrcps::helpers::calculate_crc16(&*output);
                output.push_str(&*format!("{:X}", crc));
                output
            }
        }

        #parser_impl
    };
    output.into()

    // Goes through each field in order
    // Write its ID, serialize children to string, check string size, write length, writes children

    // if option, the is_some thing
    // if non string type needs to write serialize on children
}

fn format_serializer_token(field_info: &FieldKind) -> TokenStream2 {
    let is_stringy = field_is_stringy(&field_info.kind);
    let is_option = field_is_option(&field_info.kind);

    let name = &field_info.name;
    let encoder_id = &field_info.br_code_id;

    let inner_token = quote! {
        let inner = &self.#name;
    };

    match (is_stringy, is_option) {
        (true, true) => {
            quote! {
                #inner_token
                if let Some(inner) = inner.as_ref() {
                    output.push_str(&*format!("{}{:02}", #encoder_id, inner.char_count()));
                    output.push_str(&*inner);
                }
            }
        }
        (true, false) => {
            quote! {
                #inner_token
                output.push_str(&*format!("{}{:02}", #encoder_id, inner.char_count()));
                output.push_str(&*inner);
            }
        }
        (false, true) => {
            quote! {
                #inner_token
                if let Some(inner) = inner.as_deref() {
                    let inner_serialized = inner.serialize();
                    output.push_str(&*format!("{}{:02}", #encoder_id, inner_serialized.len()));
                    output.push_str(&*inner_serialized);
                }
            }
        }
        (false, false) => {
            quote! {
                #inner_token
                let inner_serialized = inner.serialize();
                output.push_str(&*format!("{}{:02}", #encoder_id, inner_serialized.len()));
                output.push_str(&*inner_serialized);
            }
        }
    }
}

fn get_struct_fields(derive_input: &syn::DeriveInput) -> Option<&StructFields> {
    if let Data::Struct(data_struct) = &derive_input.data {
        if let Fields::Named(fields) = &data_struct.fields {
            Some(&fields.named)
        } else {
            None
        }
    } else {
        None
    }
}

fn is_string_from_arguments(arg: &GenericArgument) -> bool {
    match arg {
        GenericArgument::Type(kind) => {
            return match kind {
                Type::Path(token) => match token.path.segments.first() {
                    Some(p) => p.ident == "String" || p.ident == "Cow" || p.ident == "str",
                    _ => false,
                },
                _ => false,
            }
        }
        _ => false,
    }
}

/// Return true for a type of String, or Option<String>.
fn field_is_stringy(kind: &syn::Type) -> bool {
    match kind {
        Type::Path(token) => token.path.segments.iter().any(|path_segment| {
            path_segment.ident == "String"
                || path_segment.ident == "str"
                || path_segment.ident == "Cow"
                || match &path_segment.arguments {
                    PathArguments::AngleBracketed(br) => {
                        br.args.iter().any(|generic_arg| is_string_from_arguments(generic_arg))
                    }
                    _ => false,
                }
        }),
        _ => false,
    }
}

fn ident_from_type(kind: &syn::Type) -> Option<&Ident> {
    match kind {
        syn::Type::Path(t) => match t.path.segments.first() {
            Some(t) => Some(&t.ident),
            _ => None,
        },
        _ => None,
    }
}

fn field_is_option(kind: &syn::Type) -> bool {
    match kind {
        syn::Type::Path(t) => match t.path.segments.first() {
            Some(t) => t.ident == "Option",
            _ => false,
        },
        _ => false,
    }
}
