extern crate proc_macro;

use crate::proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn;
use syn::Lit::Str;
use syn::Meta::NameValue;
use syn::{MetaNameValue};

type ErrorMessage = &'static str;

fn impl_linkify(ast: syn::DeriveInput) -> Result<proc_macro2::TokenStream, ErrorMessage> {
    let name = &ast.ident;
    Ok(quote! {
        impl rocket_jsonapi::links::Linkify for #name {
            fn get_links() -> Vec<rocket_jsonapi::links::LinksObject> {
                vec![]
            }
        }
    })
}

#[proc_macro_derive(Linkify)]
pub fn linkify_derive(input: TokenStream) -> TokenStream {
    expand_proc_macro(input, impl_linkify)
}

// TODO refactor, better names
fn ident_id() -> Ident {
    Ident::new("id", Span::call_site())
}

fn ident_type_from(name: &Ident) -> Ident {
    Ident::new(&format!("{}", name), name.span())
}

const ID: &'static str = "id";

fn impl_resource_identifiable(
    ast: syn::DeriveInput,
) -> Result<proc_macro2::TokenStream, ErrorMessage> {
    let name = &ast.ident;
    let mut name_values = &ast
        .attrs
        .iter()
        .filter_map(|attr| attr.parse_meta().ok())
        .filter_map(|m| match m {
            NameValue(meta) => Some(meta),
            _ => None,
        })
        .collect::<Vec<MetaNameValue>>();
    let id_field = match &ast.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields) => Ok(fields),
            _ => Err(
                "ResourceIdentifiable can only be derived from a named struct, not tuple \
                 struct or unit struct",
            ),
        },
        _ => Err("ResourceIdentifiable can only be derived from a struct, not enum or union"),
    }?
    .named
    .iter()
    .find(|f| format!("{}", f.ident.as_ref().unwrap()).as_str() == ID)
    .ok_or(
        "If ud not specified through `resource_ident_id` attribute, there must be a \
         field in struct named: `id`",
    )?;
    // TODO handle id_field type
    /*
    match &id_field.ty {
        syn::Type::Path(p) => match format!("{}", p.path.get_ident().unwrap()).as_str() {
            "String" => ,
        },
        _ => {println!("Something else");}
    }
    */
    // TODO refactor, maybe share with `resource_ident_type`
    let resource_ident_id = name_values
        .iter()
        .find(|m| m.path.is_ident("resource_ident_id"))
        .map_or(ident_id(), |m| match &m.lit {
            Str(literal) => Ident::new(&literal.value(), Span::call_site()),
            _ => ident_id(),
        });
    let resource_ident_type = name_values
        .iter()
        .find(|m| m.path.is_ident("resource_ident_type"))
        .map_or(ident_type_from(&name), |m| match &m.lit {
            Str(literal) => Ident::new(&literal.value(), Span::call_site()),
            _ => ident_type_from(&name),
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
    Ok(gen)
}

#[proc_macro_derive(
    ResourceIdentifiable,
    attributes(resource_ident_id, resource_ident_type)
)]
pub fn resource_identifiable_derive(input: TokenStream) -> TokenStream {
    expand_proc_macro(input, impl_resource_identifiable)
}

// Thanks to diesel
fn expand_proc_macro<T: syn::parse::Parse>(
    input: TokenStream,
    f: fn(T) -> Result<proc_macro2::TokenStream, ErrorMessage>,
) -> TokenStream {
    let item = syn::parse(input).unwrap();
    match f(item) {
        Ok(x) => x.into(),
        Err(e) => {
            panic!("{}", e);
            "".parse().unwrap()
        }
    }
}
