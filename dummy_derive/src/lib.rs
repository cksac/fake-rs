extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate darling;

use syn::{Ident, Type};

use darling::{ast, util, FromDeriveInput};
use proc_macro::TokenStream;

#[derive(Debug, FromField)]
#[darling(attributes(dummy))]
struct DummyField {
    ident: Option<Ident>,
    ty: Type,
    #[darling(default)]
    faker: Option<String>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(dummy), supports(struct_named))]
struct Dummy {
    ident: Ident,
    data: ast::Data<util::Ignored, DummyField>,
}

#[proc_macro_derive(Dummy, attributes(dummy))]
pub fn hello_world(input: TokenStream) -> TokenStream {
    let parsed = syn::parse(input).unwrap();
    let receiver = Dummy::from_derive_input(&parsed).unwrap();

    let struct_name = &receiver.ident;
    let expanded = match receiver.data {
        darling::ast::Data::Struct(darling::ast::Fields { ref fields, .. }) => {
            let struct_fields: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();

            let let_statements: Vec<_> = fields
                .iter()
                .map(|f| {
                    let field_name = f.ident.as_ref().unwrap();
                    let field_ty = &f.ty;
                    let faker = if let Some(ref expr) = f.faker {
                        syn::parse_str::<syn::Expr>(expr).unwrap()
                    } else {
                        syn::parse_str::<syn::Expr>(&"fake::Faker").unwrap()
                    };
                    quote! {
                        let #field_name: #field_ty = (#faker).fake_with_rng(rng);
                    }
                })
                .collect();

            let impl_dummy = quote! {
                impl fake::Dummy<fake::Faker> for #struct_name {
                    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &fake::Faker, rng: &mut R) -> Self {
                        #(#let_statements)*
                        #struct_name {
                            #(#struct_fields),*
                        }
                    }
                }
            };
            impl_dummy
        }
        _ => panic!("Dummy custom derive is not implemented for union or enum type."),
    };
    expanded.into()
}
