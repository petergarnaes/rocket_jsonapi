extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::{MetaNameValue, Path};
use syn::Meta::{NameValue};
use syn::Lit::{Verbatim, Str};
use proc_macro2::{Ident, Span};

fn impl_linkify(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl rocket_jsonapi::links::Linkify for #name {
            fn get_links() -> Vec<rocket_jsonapi::links::LinksObject> {
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

// TODO refactor, better names
fn ident_id() -> Ident {
    Ident::new("id", Span::call_site())
}

fn ident_type_from(name: &Ident) -> Ident {
    Ident::new(&format!("{}", name), name.span())
}

fn impl_resource_identifiable(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let mut name_values = &ast.attrs.iter().filter_map(|attr| {
        attr.parse_meta().ok()
    }).filter_map(|m| match m {
        NameValue(meta) => Some(meta),
        _ => None
    }).collect::<Vec<MetaNameValue>>();
    // TODO refactor, maybe share with `resource_ident_type`
    let resource_ident_id = name_values.iter().find(|m| m.path.is_ident("resource_ident_id"))
        .map_or(ident_id(), |m| match &m.lit {
            Str(literal) => Ident::new(&literal.value(), Span::call_site()),
            _ => ident_id()
        });
    let resource_ident_type = name_values.iter().find(|m| m.path.is_ident("resource_ident_type"))
        .map_or(ident_type_from(&name), |m| match &m.lit {
            Str(literal) => Ident::new(&literal.value(), Span::call_site()),
            _ => ident_type_from(&name)
        });
    // TODO resource_ident_id should look at the id field type, and properly convert if possible
    let gen = quote! {
        impl rocket_jsonapi::data::ResourceIdentifiable for #name {
            fn get_type(&self) -> &'static str {
                &stringify!(#resource_ident_type)
            }
            fn get_id(&self) -> String {
                self.#resource_ident_id.to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(ResourceIdentifiable, attributes(resource_ident_id, resource_ident_type))]
pub fn resource_identifiable_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_resource_identifiable(&ast)

}