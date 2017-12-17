extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::VariantData;
use quote::ToTokens;
use syn::{MetaItem, Lit};
use quote::Tokens;
use syn::Ident;

#[proc_macro_derive(Accessor)]
pub fn generate_accessor(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = accessors(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn accessors(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    match ast.body {
        syn::Body::Struct(ref vdata) => {
            match vdata {
                &VariantData::Struct(ref fields) => {
                    let gen = StructGenerator { fields };
                    quote! {
                        impl #name {
                            #gen
                        }
                    }
                },
                _ => panic!("Only syn::VariantData::Struct supported!"),
            }
        },
        _ => panic!("Only syn::Body::Struct supported!"),
    }
}

struct StructGenerator<'a> {
    fields: &'a [syn::Field]
}

impl<'a> quote::ToTokens for StructGenerator<'a> {
    fn to_tokens(&self, tokens: &mut quote::Tokens) {
        for field in self.fields {
            let field_name = &field.ident.as_ref().expect("Struct field should have name!");
            let field_type = &field.ty;
            if let Some(attribute) = field.attrs.iter().find(|attribute| attribute.name() == "accessor_size") {
                let field_size = match attribute.value {
                    MetaItem::NameValue(_, ref literal) => {
                        match literal {
                            &Lit::Int(field_size, _) => {
                                field_size
                            },
                            _ => panic!("Only syn::Lit::Int supported!"),
                        }
                    },
                    _ => panic!("Only syn::MetaItem:NameValue supported!"),
                };
                let accessor_function_name = Ident::from("get_".to_owned()+field_name.as_ref());
                tokens.append(
                    quote! {
                        const #field_name : u64 = #field_size;

                        pub fn #accessor_function_name(value: &[u8]) -> #field_type {
                            1
                        }
                });
            } else {
                panic!("Field {} should have `hp_size` attribute!", field_name);
            }
        }
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
