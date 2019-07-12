extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use serde_derive_internals as serdei;
use std::error::Error;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(FieldSelector, attributes(field_selector))]
pub fn derive_field_selector(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_derive_field_selector(&input).unwrap().into()
}

fn expand_derive_field_selector(input: &DeriveInput) -> Result<TokenStream2, Box<dyn Error>> {
    let ctx = serdei::Ctxt::new();
    let cont = serdei::ast::Container::from_ast(&ctx, &input, serdei::Derive::Deserialize);
    ctx.check()?;
    let field_output: Vec<proc_macro2::TokenStream> = match cont.data {
        serdei::ast::Data::Struct(serdei::ast::Style::Struct, fields) => {
            fields.iter().map(selector_for_field).collect()
        }
        _ => return Err("Only able to derive FieldSelector for plain Struct".into()),
    };

    let ident = cont.ident;
    let (impl_generics, ty_generics, where_clause) = cont.generics.split_for_impl();
    let dummy_const = syn::Ident::new(
        &format!("_IMPL_FIELD_SELECTOR_FOR_{}", ident),
        proc_macro2::Span::call_site(),
    );
    Ok(quote! {
        const #dummy_const: () = {
            impl #impl_generics FieldSelector for #ident #ty_generics #where_clause {
                fn field_selector_with_ident(ident: &str, selector: &mut String) {
                    match selector.chars().rev().nth(0) {
                        Some(',') | None => {},
                        _ => selector.push_str(","),
                    }
                    #(#field_output)*
                }
            }
        };
    })
}

fn selector_for_field<'a>(field: &serdei::ast::Field<'a>) -> TokenStream2 {
    enum AttrOverride {
        ContainerOf(syn::ExprPath),
        Leaf,
    }
    let syn_field = field.original;
    let attr_override = syn_field.attrs.iter().find_map(|attr| {
        let metalist = match attr.parse_meta() {
            Ok(meta @ syn::Meta::List(_)) => meta,
            _ => return None,
        };
        if metalist.name() != "field_selector" {
            return None;
        }
        let nestedlist = match metalist {
            syn::Meta::List(syn::MetaList { nested, .. }) => nested,
            _ => return None,
        };
        for meta in nestedlist.iter() {
            match meta {
                syn::NestedMeta::Meta(syn::Meta::NameValue(syn::MetaNameValue {
                    ident: name,
                    lit: syn::Lit::Str(value),
                    ..
                })) if name == "container_of" => {
                    if let Ok(typ_path) = value.parse() {
                        return Some(AttrOverride::ContainerOf(typ_path));
                    }
                }
                syn::NestedMeta::Meta(syn::Meta::Word(word)) if word == "leaf" => {
                    return Some(AttrOverride::Leaf);
                }
                _ => {}
            }
        }
        None
    });

    let field_name = field.attrs.name().deserialize_name();
    match attr_override {
        Some(AttrOverride::ContainerOf(typ_path)) => {
            quote! {
                match selector.chars().rev().nth(0) {
                    Some(',') | None => {},
                    _ => selector.push_str(","),
                }
                if ident.is_empty() {
                    selector.push_str(#field_name);
                } else {
                    let mut ident = ident.to_owned();
                    ident.push_str("/");
                    ident.push_str(#field_name);
                    selector.push_str(&ident);
                }
                let mut inner_selector = String::new();
                #typ_path::field_selector_with_ident("", &mut inner_selector);
                if !inner_selector.is_empty() {
                    selector.push_str("(");
                    selector.push_str(&inner_selector);
                    selector.push_str(")");
                }
            }
        }
        Some(AttrOverride::Leaf) => {
            quote! {
                match selector.chars().rev().nth(0) {
                    Some(',') | None => {},
                    _ => selector.push_str(","),
                }
                if ident.is_empty() {
                    selector.push_str(#field_name);
                } else {
                    let mut ident = ident.to_owned();
                    ident.push_str("/");
                    ident.push_str(#field_name);
                    selector.push_str(&ident);
                }
            }
        }
        None => {
            let typ = field.ty;
            if field.attrs.flatten() {
                quote! {
                    <#typ>::field_selector_with_ident(ident, selector);
                }
            } else {
                quote! {
                    <#typ>::field_selector_with_ident(&{
                        if ident.is_empty() {
                            #field_name.to_owned()
                        } else {
                            let mut ident = ident.to_owned();
                            ident.push_str("/");
                            ident.push_str(#field_name);
                            ident
                        }},
                        selector);
                }
            }
        }
    }
}
