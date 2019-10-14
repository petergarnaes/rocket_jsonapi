extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

fn impl_linkify(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Linkify for #name {
            fn get_links() -> Vec<LinksObject> {
                vec![]
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Linkify)]
pub fn linkify_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_linkify(&ast)
}
