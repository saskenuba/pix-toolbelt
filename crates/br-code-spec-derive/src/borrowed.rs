use quote::quote;
use syn::Ident;
use syn::__private::TokenStream2;

use crate::FieldKind;

/// Generates a borrowed kind of the input struct, intended to be used by the no alloc parser.
pub(crate) fn generate_borrowed_struct(struct_name: &Ident, fields: &[FieldKind]) -> TokenStream2 {
    let struct_name_ref = Ident::new(&format!("{}Ref", struct_name), struct_name.span());
    let (id, name): (Vec<_>, Vec<_>) = fields.iter().map(|x| (&x.br_code_id, &x.name)).unzip();

    let borrowed_struct_tokens = quote! {
        #[derive(Debug, Clone, PartialEq)]
        struct #struct_name_ref<'a> {
                #(#name: &'a str),*
        }

        impl<'a> #struct_name_ref<'a> {
           fn from_lookup(map: HashMap<&str, &'a str>) -> Self {
                Self {
                    #(#name: map.get(#id).unwrap()),*
                }
           }
        }

        impl<'a> #struct_name {

            pub fn from_str(source_str: &'a str) -> #struct_name_ref<'a> {
                use crate::lexer::header_length_remaining;
                use crate::lexer::HasChildren;
                use strum::IntoEnumIterator;
                use ::std::str::FromStr;

                let mut cursor = source_str;
                let mut lookup = HashMap::new();

                while let Some((header_id, content_length, rest)) = header_length_remaining(cursor) {
                    println!("{:?}{:?}", header_id, content_length);

                    let length_index = usize::from_str(content_length).unwrap();
                    let content = &rest[..length_index];
                    let remaining = &rest[length_index..];

                    lookup.insert(header_id, content);

                    println!("content: {:?}", content);
                    println!("remaining: {:?}", remaining);

                    if HasChildren::iter()
                        .map(|str| str.into())
                        .any(|header_with_son: &str| header_with_son == header_id)
                    {
                        let mut inner_content = content;
                        println!("Header has son.");

                        while let Some((header, length, remaining)) = header_length_remaining(inner_content) {
                            println!("{:?}{:?}", header, length);

                            let length_index = usize::from_str(length).unwrap();
                            let content = &remaining[..length_index];
                            let remaining = &remaining[length_index..];
                            println!("content: {:?}", content);
                            println!("remaining: {:?}", remaining);

                            inner_content = remaining;
                        }
                    }
                    cursor = remaining;
                }
                #struct_name_ref::from_lookup(lookup)
            }
        }
    };

    borrowed_struct_tokens
}
