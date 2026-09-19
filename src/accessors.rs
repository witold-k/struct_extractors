// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub(crate) fn access_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

pub(crate) fn extract_accessors_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    let struct_ident = &ast.ident;
    let mut methods = quote! {};

    if let syn::Data::Struct(ref mut data) = ast.data {
        for field in &mut data.fields {
            let mut accessor = None;

            field.attrs.retain(|attr| {
                if attr.path().is_ident("access") {
                    accessor = Some(
                        attr.parse_args::<syn::Meta>()
                            .expect("invalid #[access(...)] syntax"),
                    );
                    false
                } else {
                    true
                }
            });

            if let Some(meta) = accessor {
                let field_ident = field.ident.as_ref().expect("named fields only");
                let ty = &field.ty;

                match meta {
                    syn::Meta::Path(path) => {
                        let ident = path
                            .get_ident()
                            .expect("expected accessor identifier")
                            .to_string();

                        match ident.as_str() {
                            "get" => {
                                let method_name =
                                    syn::Ident::new(&format!("get_{}", field_ident), field_ident.span());
                                methods.extend(quote! {
                                    pub fn #method_name(&self) -> #ty
                                    where
                                        #ty: Copy,
                                    {
                                        self.#field_ident
                                    }
                                });
                            }
                            "get_ref" => {
                                let method_name = syn::Ident::new(
                                    &format!("get_ref_{}", field_ident),
                                    field_ident.span(),
                                );
                                methods.extend(quote! {
                                    pub fn #method_name(&self) -> &#ty {
                                        &self.#field_ident
                                    }
                                });
                            }
                            "get_mut" => {
                                let method_name = syn::Ident::new(
                                    &format!("get_mut_{}", field_ident),
                                    field_ident.span(),
                                );
                                methods.extend(quote! {
                                    pub fn #method_name(&mut self) -> &mut #ty {
                                        &mut self.#field_ident
                                    }
                                });
                            }
                            other => panic!("unknown accessor: {other}"),
                        }
                    }
                    syn::Meta::NameValue(nv) if nv.path.is_ident("get") => {
                        let syn::Expr::Lit(expr_lit) = nv.value else {
                            panic!("expected string literal in #[access(get = \"...\")]");
                        };
                        let syn::Lit::Str(litstr) = expr_lit.lit else {
                            panic!("expected string literal in #[access(get = \"...\")]");
                        };
                        let method_name = syn::Ident::new(&litstr.value(), litstr.span());
                        methods.extend(quote! {
                            pub fn #method_name(&self) -> #ty
                            where
                                #ty: Copy,
                            {
                                self.#field_ident
                            }
                        });
                    }
                    syn::Meta::NameValue(_) => panic!("unknown accessor form"),
                    _ => panic!("invalid #[access(...)] syntax"),
                }
            }
        }
    }

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    quote! {
        #ast

        impl #impl_generics #struct_ident #ty_generics #where_clause {
            #methods
        }
    }
    .into()
}
