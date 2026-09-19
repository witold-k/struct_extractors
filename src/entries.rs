// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemEnum, Path, punctuated::Punctuated, Token};

pub(crate) fn base_entries_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemEnum);
    let enum_ident = &input.ident;
    let module_ident = format_ident!(
        "__base_entries_{}",
        enum_ident.to_string().to_lowercase()
    );
    let variants = input.variants.iter().map(|variant| &variant.ident);

    quote! {
        #input

        #[doc(hidden)]
        pub mod #module_ident {
            #(pub struct #variants;)*

            pub trait ContainsAll<Other> {}
        }
    }
    .into()
}

pub(crate) fn same_entries_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let bases = parse_macro_input!(attr with Punctuated::<Path, Token![,]>::parse_terminated);
    let input = parse_macro_input!(item as ItemEnum);
    let enum_ident = &input.ident;
    let check_module = format_ident!(
        "__same_entries_check_{}",
        enum_ident.to_string().to_lowercase()
    );
    let variants: Vec<_> = input.variants.iter().map(|variant| variant.ident.clone()).collect();

    let checks = bases.iter().flat_map(|base| {
        let base_ident = base
            .get_ident()
            .unwrap_or_else(|| panic!("expected enum identifier in #[same_entries(...)]"));
        let base_module = format_ident!(
            "__base_entries_{}",
            base_ident.to_string().to_lowercase()
        );

        variants.iter().map(move |variant| {
            let check_fn = format_ident!(
                "_check_{}_from_{}",
                variant.to_string().to_lowercase(),
                base_ident.to_string().to_lowercase()
            );
            quote! {
                fn #check_fn() {
                    let _: super::#base_module::#variant;
                }
            }
        })
    });

    quote! {
        #input

        #[doc(hidden)]
        mod #check_module {
            #(#checks)*
        }
    }
    .into()
}
