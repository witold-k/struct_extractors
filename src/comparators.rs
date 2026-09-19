// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub(crate) fn extract_compare_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

pub(crate) fn extract_comparators_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    let struct_ident = &ast.ident;
    let generics = ast.generics.clone();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let mut methods = quote! {};

    if let syn::Data::Struct(ref mut data) = ast.data {
        for field in &mut data.fields {
            let mut comparator_name = None;

            field.attrs.retain(|attr| {
                if attr.path().is_ident("extract_compare") {
                    let path = attr
                        .parse_args::<syn::Path>()
                        .expect("invalid #[extract_compare(name)] syntax");
                    comparator_name = Some(
                        path.get_ident()
                            .cloned()
                            .expect("expected identifier in #[extract_compare(name)]"),
                    );
                    false
                } else {
                    true
                }
            });

            if let Some(cmp_ident) = comparator_name {
                let field_ident = field.ident.as_ref().expect("named fields only");
                methods.extend(quote! {
                    #[inline]
                    pub fn #cmp_ident(a: &Self, b: &Self) -> core::cmp::Ordering {
                        a.#field_ident.cmp(&b.#field_ident)
                    }
                });
            }
        }
    }

    quote! {
        #ast

        impl #impl_generics #struct_ident #ty_generics #where_clause {
            #methods
        }
    }
    .into()
}
