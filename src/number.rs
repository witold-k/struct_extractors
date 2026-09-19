// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, DeriveInput, Expr, Token,
    punctuated::Punctuated,
};

struct ArgList {
    items: Punctuated<Expr, Token![,]>,
}

impl Parse for ArgList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            items: input.parse_terminated(Expr::parse, Token![,])?,
        })
    }
}

#[proc_macro_attribute]
pub fn extract_number(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ArgList);

    let field_ident = match args.items.len() {
        1 => match &args.items[0] {
            Expr::Path(expr_path) => {
                expr_path
                    .path
                    .get_ident()
                    .cloned()
                    .expect("expected identifier in #[extract_number(field)]")
            }
            _ => panic!("expected identifier in #[extract_number(field)]"),
        },
        _ => panic!("expected exactly one argument: #[extract_number(field)]"),
    };

    let input_ast = parse_macro_input!(input as DeriveInput);
    let name = &input_ast.ident;
    let generics = &input_ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        #input_ast

        /* ------------------ BASIC OPS ------------------ */

        impl #impl_generics ::std::ops::Neg for #name #ty_generics
        #where_clause
        {
            type Output = Self;
            fn neg(self) -> Self {
                Self { #field_ident: -self.#field_ident, ..self }
            }
        }

        impl #impl_generics ::std::ops::Add for #name #ty_generics
        where
            T: ::std::ops::Add<Output = T>,
        {
            type Output = Self;
            fn add(self, other: Self) -> Self {
                Self { #field_ident: self.#field_ident + other.#field_ident, ..self }
            }
        }

        impl #impl_generics ::std::ops::Sub for #name #ty_generics
        where
            T: ::std::ops::Sub<Output = T>,
        {
            type Output = Self;
            fn sub(self, other: Self) -> Self {
                Self { #field_ident: self.#field_ident - other.#field_ident, ..self }
            }
        }

        impl #impl_generics ::std::ops::Mul for #name #ty_generics
        where
            T: ::std::ops::Mul<Output = T>,
        {
            type Output = Self;
            fn mul(self, other: Self) -> Self {
                Self { #field_ident: self.#field_ident * other.#field_ident, ..self }
            }
        }

        impl #impl_generics ::std::ops::Div for #name #ty_generics
        where
            T: ::std::ops::Div<Output = T>,
        {
            type Output = Self;
            fn div(self, other: Self) -> Self {
                Self { #field_ident: self.#field_ident / other.#field_ident, ..self }
            }
        }

        /* scalar ops: Metric<T> * T, Metric<T> / T */

        impl #impl_generics ::std::ops::Mul<T> for #name #ty_generics
        where
            T: ::std::ops::Mul<Output = T>,
        {
            type Output = Self;
            fn mul(self, other: T) -> Self {
                Self { #field_ident: self.#field_ident * other, ..self }
            }
        }

        impl #impl_generics ::std::ops::Div<T> for #name #ty_generics
        where
            T: ::std::ops::Div<Output = T>,
        {
            type Output = Self;
            fn div(self, other: T) -> Self {
                Self { #field_ident: self.#field_ident / other, ..self }
            }
        }

        /* ------------------ ASSIGN OPS ------------------ */

        impl #impl_generics ::std::ops::AddAssign for #name #ty_generics
        where
            T: ::std::ops::AddAssign,
        {
            fn add_assign(&mut self, other: Self) {
                self.#field_ident += other.#field_ident;
            }
        }

        impl #impl_generics ::std::ops::SubAssign for #name #ty_generics
        where
            T: ::std::ops::SubAssign,
        {
            fn sub_assign(&mut self, other: Self) {
                self.#field_ident -= other.#field_ident;
            }
        }

        impl #impl_generics ::std::ops::MulAssign for #name #ty_generics
        where
            T: ::std::ops::MulAssign,
        {
            fn mul_assign(&mut self, other: Self) {
                self.#field_ident *= other.#field_ident;
            }
        }

        impl #impl_generics ::std::ops::MulAssign<T> for #name #ty_generics
        where
            T: ::std::ops::MulAssign,
        {
            fn mul_assign(&mut self, other: T) {
                self.#field_ident *= other;
            }
        }

        impl #impl_generics ::std::ops::DivAssign for #name #ty_generics
        where
            T: ::std::ops::DivAssign,
        {
            fn div_assign(&mut self, other: Self) {
                self.#field_ident /= other.#field_ident;
            }
        }

        impl #impl_generics ::std::ops::DivAssign<T> for #name #ty_generics
        where
            T: ::std::ops::DivAssign,
        {
            fn div_assign(&mut self, other: T) {
                self.#field_ident /= other;
            }
        }

        /* ------------------ COMPARISON ------------------ */

        impl #impl_generics ::std::cmp::PartialEq for #name #ty_generics
        where
            T: ::std::cmp::PartialEq,
        {
            fn eq(&self, other: &Self) -> bool {
                self.#field_ident == other.#field_ident
            }
        }

        impl #impl_generics ::std::cmp::PartialOrd for #name #ty_generics
        where
            T: ::std::cmp::PartialOrd,
        {
            fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
                self.#field_ident.partial_cmp(&other.#field_ident)
            }
        }

        /* ------------------ SUM & PRODUCT ------------------ */

        impl #impl_generics ::std::iter::Sum for #name #ty_generics
        where
            T: ::std::iter::Sum,
            #name #ty_generics: ::std::default::Default,
        {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                let total = iter.map(|x| x.#field_ident).sum();
                Self { #field_ident: total, ..Default::default() }
            }
        }

        impl #impl_generics ::std::iter::Product for #name #ty_generics
        where
            T: ::std::iter::Product,
            #name #ty_generics: ::std::default::Default,
        {
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                let total = iter.map(|x| x.#field_ident).product();
                Self { #field_ident: total, ..Default::default() }
            }
        }

        /* ------------------ ZERO & ONE ------------------ */

        impl #impl_generics ::num_traits::Zero for #name #ty_generics
        where
            T: ::num_traits::Zero,
            #name #ty_generics: ::std::default::Default,
        {
            fn zero() -> Self {
                Self { #field_ident: T::zero(), ..Default::default() }
            }
            fn is_zero(&self) -> bool {
                self.#field_ident.is_zero()
            }
        }

        impl #impl_generics ::num_traits::One for #name #ty_generics
        where
            T: ::num_traits::One,
            #name #ty_generics: ::std::default::Default,
        {
            fn one() -> Self {
                Self { #field_ident: T::one(), ..Default::default() }
            }
        }
    };

    TokenStream::from(expanded)
}
