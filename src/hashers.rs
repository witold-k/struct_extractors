// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, GenericParam};

pub(crate) fn extract_hash_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

pub(crate) fn extract_hashers_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    let struct_ident = &ast.ident;
    let generics = ast.generics.clone();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let params: Vec<_> = generics.params.iter().collect();
    let type_args = generics.params.iter().map(|param| match param {
        GenericParam::Type(param) => {
            let ident = &param.ident;
            quote!(#ident)
        }
        GenericParam::Lifetime(param) => {
            let lifetime = &param.lifetime;
            quote!(#lifetime)
        }
        GenericParam::Const(param) => {
            let ident = &param.ident;
            quote!(#ident)
        }
    });
    let type_args: Vec<_> = type_args.collect();

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
                let wrapper_params = &params;
                let wrapper_args = &type_args;

                wrappers.extend(quote! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct #wrapper_ident<'__hash, #(#wrapper_params),*>(
                        pub &'__hash #struct_ident #ty_generics
                    ) #where_clause;

                    impl<'__hash, #(#wrapper_params),*> core::cmp::PartialEq
                        for #wrapper_ident<'__hash, #(#wrapper_args),*>
                    #where_clause
                    {
                        #[inline]
                        fn eq(&self, other: &Self) -> bool {
                            self.0.#field_ident == other.0.#field_ident
                        }
                    }

                    impl<'__hash, #(#wrapper_params),*> core::cmp::Eq
                        for #wrapper_ident<'__hash, #(#wrapper_args),*>
                    #where_clause
                    {}

                    impl<'__hash, #(#wrapper_params),*> core::hash::Hash
                        for #wrapper_ident<'__hash, #(#wrapper_args),*>
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
                    pub fn #method_ident(&self) -> #wrapper_ident<'_, #(#wrapper_args),*> {
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
