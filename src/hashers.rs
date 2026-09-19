// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput};

pub(crate) fn extract_hash_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

pub(crate) fn extract_hashers_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    let struct_ident = &ast.ident;
    let mut generated = quote! {};

    if let Data::Struct(ref mut data) = ast.data {
        for field in &mut data.fields {
            let mut marked = false;

            field.attrs.retain(|attr| {
                if attr.path().is_ident("extract_hash") {
                    marked = true;
                    false
                } else {
                    true
                }
            });

            if marked {
                let field_ident = field.ident.as_ref().expect("named fields only");
                let wrapper_ident = format_ident!("{}HashBy{}", struct_ident, field_ident);

                generated.extend(quote! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct #wrapper_ident<'a>(pub &'a #struct_ident);

                    impl core::cmp::PartialEq for #wrapper_ident<'_> {
                        #[inline]
                        fn eq(&self, other: &Self) -> bool {
                            self.0.#field_ident == other.0.#field_ident
                        }
                    }

                    impl core::cmp::Eq for #wrapper_ident<'_> {}

                    impl core::hash::Hash for #wrapper_ident<'_> {
                        #[inline]
                        fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                            self.0.#field_ident.hash(state);
                        }
                    }

                    impl #struct_ident {
                        #[inline(always)]
                        pub fn hash(&self) -> #wrapper_ident {
                            #wrapper_ident(self)
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
