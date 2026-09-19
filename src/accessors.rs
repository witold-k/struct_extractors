// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn access(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn extract_accessors(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    let struct_ident = &ast.ident;

    let mut methods = quote! {};

    if let syn::Data::Struct(ref mut data) = ast.data {
        for field in &mut data.fields {
            let mut accessor = None;

            // Extract #[access(...)]
            field.attrs.retain(|attr| {
                if attr.path().is_ident("access") {
                    accessor = Some(attr.parse_args::<syn::Meta>().unwrap());
                    false
                } else {
                    true
                }
            });

            if let Some(meta) = accessor {
                let field_ident = field.ident.as_ref().unwrap();
                let ty = &field.ty;

                match meta {
                    syn::Meta::Path(path) => {
                        let ident = path.get_ident().unwrap().to_string();

                    match ident.as_str() {
                        "get" => {
                            let method_name = syn::Ident::new(
                                &format!("get_{}", field_ident),
                                field_ident.span(),
                            );
                            methods.extend(quote! {
                                pub fn #method_name(&self) -> #ty {
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

                        other => panic!("Unknown accessor: {}", other),
                    }
                                        }

                    syn::Meta::NameValue(nv) => {
                        if nv.path.is_ident("get") {
                            if let syn::Expr::Lit(expr_lit) = nv.value
                                && let syn::Lit::Str(litstr) = expr_lit.lit {
                                    let method_name = syn::Ident::new(&litstr.value(), litstr.span());
                                    methods.extend(quote! {
                                        pub fn #method_name(&self) -> #ty {
                                            self.#field_ident
                                        }
                                    });
                            }
                        } else {
                            panic!("Unknown accessor form");
                        }
                    }

                    _ => panic!("Invalid #[access(...)] syntax"),
                }
            }
        }
    }

    // Handle generics properly
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let expanded = quote! {
        #ast

        impl #impl_generics #struct_ident #ty_generics #where_clause {
            #methods
        }
    };

    expanded.into()
}
