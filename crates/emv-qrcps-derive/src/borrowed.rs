use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

use crate::{field_is_option, field_is_stringy, ident_from_type, import_from_crate, FieldKind};

fn pre_tokenizer(field: &FieldKind) -> (TokenStream2, TokenStream2) {
    let name = &field.name;
    let id = &field.br_code_id;

    let name_id = Ident::new(&*format!("{}_{}", field.name, field.br_code_id), field.name.span());
    let kind = &field.kind;
    let kind_ident = ident_from_type(&field.kind);

    let is_option = field_is_option(kind);
    let is_stringy = field_is_stringy(kind);

    let pre = match (is_option, is_stringy) {
        (true, false) => quote! {
            let #name_id: Option<_> = #kind_ident::from_str(map.remove(#id));
        },
        (false, false) => quote! {
            let #name_id = #kind_ident::from_str(map.remove(#id).unwrap());
        },
        (false, true) => quote! {
            let #name_id = map.remove(#id).unwrap();
            let #name_id: Cow<_> = Cow::from(&*#name_id);
        },
        (true, true) => quote! {
            let #name_id: Option<_> = map.remove(#id).map(|x: &str| Cow::from(x));
        },
    };

    let pos = quote! { #name: #name_id };

    (pre, pos)
}

/// Generates the no alloc parser implementation.
pub(crate) fn generate_parser_impl(struct_name: &Ident, fields: &[FieldKind]) -> TokenStream2 {
    let (pre, pos): (Vec<_>, Vec<_>) = fields.iter().map(|field| pre_tokenizer(field)).unzip();

    // for custom struct types, we deserialize it first, with its own from_str
    // for options, we don't unwrap

    // each needs to have a "pre" token and a final token
    // pre for any operation before the Self constructor
    // final for inside the Self constructor

    let parsed_trait_token = import_from_crate(quote! {Parsed});
    let base_parser_token = import_from_crate(quote! {base_parser});

    quote! {

        impl<'a> #parsed_trait_token<'a> for #struct_name<'a> {
           fn from_lookup(map: &mut ::std::collections::HashMap<&str, &'a str>) -> Self {
                #(#pre)*

                Self {
                    #(#pos),*
                }
           }
        }

        impl<'a> #struct_name<'a> {

            #[doc = "Deserializes the source string as this struct."]
            pub fn from_str(source_str: &'a str) -> Self {
                #base_parser_token(source_str)
            }

        }
    }
}
