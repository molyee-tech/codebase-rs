extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(IntoAny)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_into_any(&ast)
}

fn impl_into_any(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl ::convert::any::IntoAny for #name {
            #[inline]
            fn into_any(self: Box<Self>) -> Box<dyn ::core::any::Any> {
                self
            }
        }
    };
    gen.into()
}