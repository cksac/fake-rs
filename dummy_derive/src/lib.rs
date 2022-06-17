extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate darling;

use syn::{Ident, Type};

use darling::{ast, FromDeriveInput};
use proc_macro::TokenStream;

#[derive(Debug, FromVariant)]
#[darling(from_ident, attributes(dummy))]
struct DummyVariant {
    ident: Ident,
    fields: darling::ast::Fields<DummyField>,
}

impl From<Ident> for DummyVariant {
    fn from(ident: Ident) -> Self {
        DummyVariant {
            ident,
            fields: darling::ast::Style::Unit.into(),
        }
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(dummy))]
struct DummyField {
    ident: Option<Ident>,
    ty: Type,
    #[darling(default)]
    faker: Option<String>,
    #[darling(default)]
    fixed: Option<String>,
    #[darling(default)]
    default: bool,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(dummy), supports(struct_any, enum_any))]
struct Dummy {
    ident: Ident,
    data: ast::Data<DummyVariant, DummyField>,
}

#[proc_macro_derive(Dummy, attributes(dummy))]
pub fn hello_world(input: TokenStream) -> TokenStream {
    let parsed = syn::parse(input).expect("syn::parse ok");
    let receiver = Dummy::from_derive_input(&parsed).expect("Dummy::from_derive_input ok");

    let receiver_name = &receiver.ident;
    let expanded = match receiver.data {
        darling::ast::Data::Struct(darling::ast::Fields {
            ref fields,
            ref style,
            ..
        }) => match style {
            ast::Style::Unit => {
                let impl_dummy = quote! {
                    impl fake::Dummy<fake::Faker> for #receiver_name {
                        fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &fake::Faker, rng: &mut R) -> Self {
                            #receiver_name
                        }
                    }
                };
                impl_dummy
            }
            ast::Style::Tuple => {
                let tuple_fields: Vec<_> = fields
                    .iter()
                    .map(|f| {
                        expose_field(f)
                    })
                    .collect();

                let impl_dummy = quote! {
                    impl fake::Dummy<fake::Faker> for #receiver_name {
                        fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &fake::Faker, rng: &mut R) -> Self {
                            #receiver_name(#(#tuple_fields),*)
                        }
                    }
                };
                impl_dummy
            }
            ast::Style::Struct => {
                let struct_fields: Vec<_> =
                    fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();

                let let_statements: Vec<_> = fields
                    .iter()
                    .map(|f| {
                        let field_name = f.ident.as_ref().unwrap();
                        let field_ty = &f.ty;
                        let stream = expose_field(&f);
                        quote! {
                            let #field_name: #field_ty = #stream;
                        }
                    })
                    .collect();

                let impl_dummy = quote! {
                    impl fake::Dummy<fake::Faker> for #receiver_name {
                        fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &fake::Faker, rng: &mut R) -> Self {
                            #(#let_statements)*
                            #receiver_name {
                                #(#struct_fields),*
                            }
                        }
                    }
                };
                impl_dummy
            }
        },
        darling::ast::Data::Enum(variants) => {
            let variant_count = variants.len();
            if variant_count > 0 {
                let match_statements: Vec<_> = variants
                    .iter()
                    .enumerate()
                    .map(|(i, f)| {
                        let variant_name = &f.ident;
                        match f.fields.style {
                            ast::Style::Unit => {
                                quote! {
                                    #i => { #receiver_name::#variant_name }
                                }
                            }
                            ast::Style::Tuple => {
                                let tuple_fields: Vec<_> = f
                                    .fields
                                    .fields
                                    .iter()
                                    .map(|f| {
                                        expose_field(&f)
                                    })
                                    .collect();

                                quote! {
                                    #i => {
                                        #receiver_name::#variant_name(#(#tuple_fields),*)
                                    }
                                }
                            }
                            ast::Style::Struct => {
                                let struct_fields: Vec<_> = f
                                    .fields
                                    .fields
                                    .iter()
                                    .map(|f| f.ident.as_ref().unwrap())
                                    .collect();

                                let let_statements: Vec<_> = f.fields.fields
                        .iter()
                        .map(|f| {
                            let field_name = f.ident.as_ref().unwrap();
                            let field_ty = &f.ty;
                            let stream = expose_field(&f);
                            quote! {
                                let #field_name: #field_ty = #stream;
                            }
                        })
                        .collect();

                                quote! {
                                    #i => {
                                        #(#let_statements)*
                                        #receiver_name::#variant_name {
                                            #(#struct_fields),*
                                        }
                                    }
                                }
                            }
                        }
                    })
                    .collect();

                let impl_dummy = quote! {
                    impl fake::Dummy<fake::Faker> for #receiver_name {
                        fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &fake::Faker, rng: &mut R) -> Self {
                            match rng.gen_range(0..#variant_count) {
                                #(#match_statements)*
                                _ => {
                                    unreachable!()
                                }
                            }
                        }
                    }
                };

                impl_dummy
            } else {
                let impl_dummy = quote! {
                    impl fake::Dummy<fake::Faker> for #receiver_name {
                        fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &fake::Faker, rng: &mut R) -> Self {
                            panic!("can not create an empty enum")
                        }
                    }
                };

                impl_dummy
            }
        }
    };
    expanded.into()
}

fn expose_field(f: &DummyField) -> proc_macro2::TokenStream {
    if f.default {
        quote!{
            Default::default()
        }
    } else if let Some(ref expr) = f.fixed {
        let fixed = syn::parse_str::<syn::Expr>(expr).unwrap();
        quote!{
            #fixed
        }
    } else {
        let faker = if let Some(ref expr) = f.faker {
            syn::parse_str::<syn::Expr>(expr).unwrap()
        } else {
            syn::parse_str::<syn::Expr>("fake::Faker").unwrap()
        };
        let field_ty = &f.ty;
        quote! {
            (#faker).fake_with_rng::<#field_ty, _>(rng)
        }
    }
}
