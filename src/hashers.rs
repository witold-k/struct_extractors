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
    let generics = ast.generics.clone();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let mut wrappers = quote! {};
    let mut methods = quote! {};

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
                let method_ident = format_ident!("hash_by_{}", field_ident);

                wrappers.extend(quote! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct #wrapper_ident<'a, #impl_generics>(
                        pub &'a #struct_ident #ty_generics
                    ) #where_clause;

                    impl<'a, #impl_generics> core::cmp::PartialEq
                        for #wrapper_ident<'a, #ty_generics>
                    #where_clause
                    {
                        #[inline]
                        fn eq(&self, other: &Self) -> bool {
                            self.0.#field_ident == other.0.#field_ident
                        }
                    }

                    impl<'a, #impl_generics> core::cmp::Eq
                        for #wrapper_ident<'a, #ty_generics>
                    #where_clause
                    {}

                    impl<'a, #impl_generics> core::hash::Hash
                        for #wrapper_ident<'a, #ty_generics>
                    #where_clause
                    {
                        #[inline]
                        fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                            core::hash::Hash::hash(&self.0.#field_ident, state);
                        }
                    }
                });

                methods.extend(quote! {
                    #[inline]
                    pub fn #method_ident(&self) -> #wrapper_ident<'_, #ty_generics> {
                        #wrapper_ident(self)
                    }
                });
            }
        }
    }

    quote! {
        #ast
        #wrappers

        impl #impl_generics #struct_ident #ty_generics #where_clause {
            #methods
        }
    }
    .into()
}
