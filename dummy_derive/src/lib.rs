extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate darling;

use syn::{parse_quote, GenericParam, Generics, Ident, Type};

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
    expr: Option<String>,
    #[darling(default)]
    default: bool,
    #[darling(default)]
    from: Option<String>,
    #[darling(default)]
    wrapper: Option<String>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(dummy), supports(struct_any, enum_any))]
struct Dummy {
    ident: Ident,
    generics: Generics,
    data: ast::Data<DummyVariant, DummyField>,
}

#[proc_macro_derive(Dummy, attributes(dummy))]
pub fn derive_dummy(input: TokenStream) -> TokenStream {
    let parsed = syn::parse(input).expect("syn::parse ok");
    let receiver = Dummy::from_derive_input(&parsed).expect("Dummy::from_derive_input ok");

    let generics = add_trait_bounds(receiver.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let receiver_name = &receiver.ident;
    let expanded = match receiver.data {
        darling::ast::Data::Struct(darling::ast::Fields {
            ref fields,
            ref style,
            ..
        }) => match style {
            ast::Style::Unit => {
                let impl_dummy = quote! {
                    impl #impl_generics ::fake::Dummy<::fake::Faker> for #receiver_name #ty_generics #where_clause {
                        fn dummy_with_rng<R: ::fake::Rng + ?Sized>(_: &::fake::Faker, rng: &mut R) -> Self {
                            #receiver_name
                        }
                    }
                };
                impl_dummy
            }
            ast::Style::Tuple => {
                let tuple_fields: Vec<_> = fields.iter().map(expose_field).collect();

                let impl_dummy = quote! {
                    impl #impl_generics ::fake::Dummy<::fake::Faker> for #receiver_name #ty_generics #where_clause {
                        fn dummy_with_rng<R: ::fake::Rng + ?Sized>(_: &::fake::Faker, rng: &mut R) -> Self {
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
                        let stream = expose_field(f);
                        quote! {
                            let #field_name: #field_ty = #stream;
                        }
                    })
                    .collect();

                let impl_dummy = quote! {
                    impl #impl_generics ::fake::Dummy<::fake::Faker> for #receiver_name #ty_generics #where_clause  {
                        fn dummy_with_rng<R: ::fake::Rng + ?Sized>(_: &::fake::Faker, rng: &mut R) -> Self {
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
                                let tuple_fields: Vec<_> =
                                    f.fields.fields.iter().map(expose_field).collect();

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

                                let let_statements: Vec<_> = f
                                    .fields
                                    .fields
                                    .iter()
                                    .map(|f| {
                                        let field_name = f.ident.as_ref().unwrap();
                                        let field_ty = &f.ty;
                                        let stream = expose_field(f);
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
                    impl #impl_generics ::fake::Dummy<::fake::Faker> for #receiver_name #ty_generics #where_clause {
                        fn dummy_with_rng<R: ::fake::Rng + ?Sized>(_: &::fake::Faker, rng: &mut R) -> Self {
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
                    impl #impl_generics ::fake::Dummy<::fake::Faker> for #receiver_name #ty_generics #where_clause {
                        fn dummy_with_rng<R: ::fake::Rng + ?Sized>(_: &::fake::Faker, rng: &mut R) -> Self {
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
        quote! {
            Default::default()
        }
    } else if let Some(ref expr_str) = f.expr {
        let expr = syn::parse_str::<syn::Expr>(expr_str).unwrap();
        quote! {
            #expr
        }
    } else {
        let field_ty = &f.ty;
        if let Some(ref expr) = f.faker {
            let faker = syn::parse_str::<syn::Expr>(expr).unwrap();

            if let Some(ref from) = f.from {
                let from_ty = syn::parse_str::<syn::Type>(from).unwrap();
                quote! {
                    std::convert::Into::<#field_ty>::into(::fake::Fake::fake_with_rng::<#from_ty, _>(&(#faker), rng))
                }
            } else if let Some(ref wrapper) = f.wrapper {
                let wrapper_ty = syn::parse_str::<syn::Type>(wrapper).unwrap();
                quote! {
                    ::fake::utils::IntoInner::into_inner(::fake::Fake::fake_with_rng::<#wrapper_ty<#field_ty>, _>(&(#faker), rng))
                }
            } else {
                quote! {
                    ::fake::Fake::fake_with_rng::<#field_ty, _>(&(#faker), rng)
                }
            }
        } else {
            quote! {
                ::fake::Fake::fake_with_rng::<#field_ty, _>(&::fake::Faker, rng)
            }
        }
    }
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(::fake::Dummy<::fake::Faker>));
        }
    }
    generics
}
