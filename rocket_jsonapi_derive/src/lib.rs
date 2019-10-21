extern crate proc_macro;

use crate::proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::error::Error;
use std::fmt;
use syn;
use syn::export::Formatter;
use syn::Lit::Str;
use syn::Meta::NameValue;
use syn::MetaNameValue;

type ErrorMessage = String;

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

// TODO pretty errors something like this
/*
#[derive(Debug)]
enum ResourceIdentifiableDeriveError {
    NotNamedStruct,
    NotStruct,
    InvalidStruct {
        struct_name: String,
        field_name: String,
    },
}

impl fmt::Display for ResourceIdentifiableDeriveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                ResourceIdentifiableDeriveError::NotNamedStruct => "TODO1",
                ResourceIdentifiableDeriveError::NotStruct => "TODO2",
                ResourceIdentifiableDeriveError::InvalidStruct {
                    ref struct_name,
                    ref field_name,
                } => "TODO3",
            }
        )
    }
}

impl Error for ResourceIdentifiableDeriveError {}
*/

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
    // TODO refactor, maybe share with `resource_ident_type`
    let resource_ident_type = name_values
        .iter()
        .find(|m| m.path.is_ident("resource_ident_type"))
        .map_or(ident_type_from(&name), |m| match &m.lit {
            Str(literal) => Ident::new(&literal.value(), Span::call_site()),
            _ => ident_type_from(&name),
        });
    let resource_ident_id = name_values
        .iter()
        .find(|m| m.path.is_ident("resource_ident_id"))
        .map_or(ident_id(), |m| match &m.lit {
            Str(literal) => Ident::new(&literal.value(), Span::call_site()),
            _ => ident_id(),
        });
    let id_field = match &ast.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields) => Ok(fields),
            _ => Err("ResourceIdentifiable must be derived from a named struct".to_string()),
        },
        _ => Err("ResourceIdentifiable must be derived from a struct".to_string()),
    }?
    .named
    .iter()
    .find(|f| {
        f.ident
            .as_ref()
            .map(|i| i.eq(&resource_ident_id))
            .eq(&Some(true))
    })
    .ok_or_else(|| {
        format!(
            "{} does not have an id field named {}",
            resource_ident_type, resource_ident_id
        )
    })?;
    let id_type = &id_field.ty;
    // Defining inner macro for each expansion is ugly
    let gen = quote! {
        impl rocket_jsonapi::data::ResourceIdentifiable for #name {
            type IdType = #id_type;

            fn get_type(&self) -> &'static str {
                &stringify!(#resource_ident_type)
            }
            fn get_id(&self) -> &Self::IdType {
                &self.#resource_ident_id
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
