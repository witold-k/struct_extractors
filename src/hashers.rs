// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_attribute]
pub fn extract_hash(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

/* -------------------------------------------------------------------------
   STRUCT-LEVEL MACRO
   ------------------------------------------------------------------------- */

#[proc_macro_attribute]
pub fn extract_hashers(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    let struct_ident = &ast.ident;

    let mut generated = quote! {};

    if let Data::Struct(ref mut data) = ast.data {
        for field in &mut data.fields {
            let mut is_extract_hash = false;

            // Suche nach #[extract_hash] ohne Argumente
            field.attrs.retain(|attr| {
                if attr.path().is_ident("extract_hash") {
                    is_extract_hash = true;
                    false // Entferne das Attribut vom Feld
                } else {
                    true
                }
            });

            if is_extract_hash {
                let field_ident = field.ident.as_ref().expect("Named fields only");

                // Generiert Namen wie: UserHashById
                let wrapper_ident = format_ident!("{}HashBy{}", struct_ident, field_ident);
                // Name der Methode am Struct: id_hasher (oder einfach das Feld-Ident)
                let method_ident = format_ident!("hash");

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
                        pub fn #method_ident(&self) -> #wrapper_ident {
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
