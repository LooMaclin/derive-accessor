extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::VariantData;

use syn::{MetaItem, Lit};
use quote::Tokens;
use syn::Ident;
use syn::Ty;
use syn::Field;
use syn::Path;

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
                    let new_struct_gen = NewStructGenerator { fields };
                    let gen = StructGenerator { fields };
                    let nameref = Ident::from(name.as_ref().to_string() + "Ref");
                    println!("nameref: {}", nameref);
                    quote! {

                        impl #name {
                            #gen
                        }

                        pub struct #nameref<'a> {
                            inner: &'a [u8]
                        }

                        impl<'a> #nameref<'a> {

                            pub fn new(slice: &'a [u8]) -> Self {
                                #nameref { inner: slice }
                            }

                            #new_struct_gen
                        }
                    }
                }
                _ => panic!("Only syn::VariantData::Struct supported!"),
            }
        }
        _ => panic!("Only syn::Body::Struct supported!"),
    }
}

struct NewStructGenerator<'a> {
    fields: &'a [syn::Field]
}

struct StructGenerator<'a> {
    fields: &'a [syn::Field]
}

#[derive(Debug)]
struct FieldInfo {
    pub type_name: String,
    pub explicit_size: Option<usize>,
    pub field_name: String,
    pub real_size: Option<usize>,
}

fn extract_explicit_size(field: &Field) -> Option<usize> {
    field
        .attrs
        .iter()
        .find(|attribute| attribute.name() == "explicit_size")
        .map(|attribute|
            match attribute.value {
                MetaItem::NameValue(_, ref literal) => {
                    match literal {
                        &Lit::Int(field_size, _) => {
                            field_size as usize
                        }
                        _ => panic!("Only syn::Lit::Int supported!"),
                    }
                }
                _ => panic!("Only syn::MetaItem:NameValue supported!"),
            })
}

fn extract_ty_name_from_path_ty(path: &Path) -> String {
    path.segments[0].ident.as_ref().to_string()
}

fn extract_type_name(ty: &Ty) -> String {
    match ty {
        &Ty::Path(_, ref path) => {
            extract_ty_name_from_path_ty(path)
        },
        &Ty::Rptr(_, ref ty) => {
            extract_type_name(&ty.as_ref().ty)
        }
        _ => panic!("Only syn::Ty::Path supported: {:#?}", ty),
    }
}

impl<'a> quote::ToTokens for NewStructGenerator<'a> {
    fn to_tokens(&self, tokens: &mut quote::Tokens) {
        let fields_info: Vec<FieldInfo> =
            self
                .fields
                .iter()
                .map(|field| {
                    FieldInfo {
                        type_name: extract_type_name(&field.ty),
                        explicit_size: extract_explicit_size(field),
                        field_name: field.ident.as_ref().expect("Struct field should have name!").as_ref().to_string(),
                        real_size: None,
                    }
                })
                .map(|item|
                    FieldInfo {
                        real_size: Some(match item.type_name.as_ref() {
                            "u8" => 1,
                            "u16" => 2,
                            "u32" => 4,
                            "u64" => 8,
                            "str" | "String" => item.explicit_size.expect("For str or String expected explicit size!"),
                            _ => panic!("This field type not supported: {:#?}", item.type_name),
                        }),
                        ..item
                })
                .collect();

        let mut elp = 0;
        let gen: Tokens = fields_info.iter().fold(Tokens::new(),|mut acc, item| {
            let field_name = Ident::from(item.field_name.clone());
            let field_type = Ident::from(item.type_name.clone());
            let accessor_fn_name = Ident::from("get_".to_string()+field_name.as_ref());
            acc.append(match item.type_name.as_ref() {
                "u8" => {
                    let res =
                    quote!{ pub fn #accessor_fn_name(&self) -> #field_type {
                    use ::byteorder::{BigEndian, ByteOrder, WriteBytesExt};
                        self.inner[#elp]
                    }};
                    elp+=1;
                    res
                },
                "u16" => {
                    let res =
                    quote!{ pub fn #accessor_fn_name(&self) -> #field_type {
                    use ::byteorder::{BigEndian, ByteOrder, WriteBytesExt};
                        BigEndian::read_u16(&self.inner[#elp..#elp+2])
                    }};
                    elp+=2;
                    res
                },
                "u32" => {
                    let res =
                    quote!{ pub fn #accessor_fn_name(&self) -> #field_type {
                    use ::byteorder::{BigEndian, ByteOrder, WriteBytesExt};
                        BigEndian::read_u32(&self.inner[#elp..#elp+4])
                    }};
                    elp+=4;
                    res
                },
                "u64" => {let res =
                    quote!{ pub fn #accessor_fn_name(&self) -> #field_type {
                    use ::byteorder::{BigEndian, ByteOrder, WriteBytesExt};
                        BigEndian::read_u64(&self.inner[#elp..#elp+8])
                    }};
                    elp+=8;
                    res
                },
                "str" | "String" => {
                    let real_size = item.real_size.unwrap();
                    let res =
                        quote!{ pub fn #accessor_fn_name(&self) -> &str {
                            unsafe { ::std::str::from_utf8_unchecked(&self.inner[#elp..#elp+#real_size]) }
                        }};
                    elp += real_size;
                    res
                }
                _ => panic!("not supported!"),
            });
            acc
        });
        tokens.append(gen);
    }
}


impl<'a> quote::ToTokens for StructGenerator<'a> {

    fn to_tokens(&self, tokens: &mut quote::Tokens) {

        let fields_info: Vec<FieldInfo> =
            self
                .fields
                .iter()
                .map(|field| {
                    FieldInfo {
                        type_name: extract_type_name(&field.ty),
                        explicit_size: extract_explicit_size(field),
                        field_name: field.ident.as_ref().expect("Struct field should have name!").as_ref().to_string(),
                        real_size: None,
                    }
                })
                .map(|item|
                    FieldInfo {
                        real_size: Some(match item.type_name.as_ref() {
                            "u8" => 1,
                            "u16" => 2,
                            "u32" => 4,
                            "u64" => 8,
                            "str" | "String" => item.explicit_size.expect("For str or String expected explicit size!"),
                            _ => panic!("This field type not supported: {:#?}", item.type_name),
                        }),
                        ..item
                    })
                .collect();

        let output_array_size: usize = fields_info.iter().map(|item| item.real_size.unwrap()).sum();

        let mut elp = 0;

        let gen: Tokens = fields_info.iter().fold(Tokens::new(),|mut acc, item| {
            let field_name = Ident::from(item.field_name.clone());

            acc.append(match item.type_name.as_ref() {
                "u8" => {
                    let res = quote!{ result[#elp] = self.#field_name; };
                    elp+=1;
                    res
                },
                "u16" => {
                    let res = quote!{ BigEndian::write_u16(&mut result[#elp..#elp+2], self.#field_name); };
                    elp+=2;
                    res
                },
                "u32" => {
                    let res = quote!{ BigEndian::write_u32(&mut result[#elp..#elp+4], self.#field_name); };
                    elp+=4;
                    res
                },
                "u64" => {
                    let res = quote!{ BigEndian::write_u64(&mut result[#elp..#elp+8], self.#field_name); };
                    elp+=8;
                    res
                },
                "str" | "String" => {
                    let real_size = item.real_size.unwrap();
                    let res = quote!{ result[#elp..#elp+#real_size].copy_from_slice(self.#field_name.as_bytes()); };
                    elp += real_size;
                    res
                }
                _ => panic!("not supported!"),
            });
            acc
        });

        tokens.append(quote!{

            pub fn to_array(&self) -> [u8; #output_array_size] {
                use ::byteorder::{BigEndian, ByteOrder, WriteBytesExt};
                let mut result : [u8; #output_array_size] = [0; #output_array_size];
                #gen
                result
            }

        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
