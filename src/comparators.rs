// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn extract_compare(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

/* -------------------------------------------------------------------------
   STRUCT-LEVEL MACRO
   ------------------------------------------------------------------------- */

#[proc_macro_attribute]
pub fn extract_comparators(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    let struct_ident = &ast.ident;

    let mut generated = quote! {};

    if let syn::Data::Struct(ref mut data) = ast.data {
        for field in &mut data.fields {
            let mut comparator_name = None;

            // Extract #[extract_compare(MyCmp)]
            field.attrs.retain(|attr| {
                if attr.path().is_ident("extract_compare") {
                    let args = attr.parse_args::<syn::Path>().unwrap();
                    comparator_name = args.get_ident().cloned();
                    false
                } else {
                    true
                }
            });

            if let Some(cmp_ident) = comparator_name {
                let field_ident = field.ident.as_ref().unwrap();

                generated.extend(quote! {
                    impl #struct_ident {
                        #[inline(always)]
                        pub fn #cmp_ident(a: &Self, b: &Self) -> core::cmp::Ordering {
                            a.#field_ident.cmp(&b.#field_ident)
                        }
                    }
                });
            }
        }
    }

    quote! {
        #ast
        #generated
    }
    .into()
}
