// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, Type};

fn add_bound(generics: &syn::Generics, predicate: syn::WherePredicate) -> syn::Generics {
    let mut generics = generics.clone();
    generics.make_where_clause().predicates.push(predicate);
    generics
}

fn selected_field_type(input: &DeriveInput, field_ident: &Ident) -> Type {
    let Data::Struct(data) = &input.data else {
        panic!("#[extract_number(...)] requires a struct");
    };
    let Fields::Named(fields) = &data.fields else {
        panic!("#[extract_number(...)] requires named fields");
    };

    fields
        .named
        .iter()
        .find(|field| field.ident.as_ref() == Some(field_ident))
        .map(|field| field.ty.clone())
        .unwrap_or_else(|| panic!("field {field_ident} does not exist"))
}

pub(crate) fn extract_number_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let field_ident = parse_macro_input!(args as Ident);
    let input_ast = parse_macro_input!(input as DeriveInput);
    let field_ty = selected_field_type(&input_ast, &field_ident);
    let name = &input_ast.ident;
    let generics = &input_ast.generics;
    let (_, ty_generics, _) = generics.split_for_impl();

    let neg_generics = add_bound(
        generics,
        syn::parse_quote!(#field_ty: ::std::ops::Neg<Output = #field_ty>),
    );
    let (neg_impl_generics, _, neg_where) = neg_generics.split_for_impl();

    let add_generics = add_bound(
        generics,
        syn::parse_quote!(#field_ty: ::std::ops::Add<Output = #field_ty>),
    );
    let (add_impl_generics, _, add_where) = add_generics.split_for_impl();

    let sub_generics = add_bound(
        generics,
        syn::parse_quote!(#field_ty: ::std::ops::Sub<Output = #field_ty>),
    );
    let (sub_impl_generics, _, sub_where) = sub_generics.split_for_impl();

    let mul_generics = add_bound(
        generics,
        syn::parse_quote!(#field_ty: ::std::ops::Mul<Output = #field_ty>),
    );
    let (mul_impl_generics, _, mul_where) = mul_generics.split_for_impl();

    let div_generics = add_bound(
        generics,
        syn::parse_quote!(#field_ty: ::std::ops::Div<Output = #field_ty>),
    );
    let (div_impl_generics, _, div_where) = div_generics.split_for_impl();

    let add_assign_generics =
        add_bound(generics, syn::parse_quote!(#field_ty: ::std::ops::AddAssign));
    let (add_assign_impl_generics, _, add_assign_where) = add_assign_generics.split_for_impl();

    let sub_assign_generics =
        add_bound(generics, syn::parse_quote!(#field_ty: ::std::ops::SubAssign));
    let (sub_assign_impl_generics, _, sub_assign_where) = sub_assign_generics.split_for_impl();

    let mul_assign_generics =
        add_bound(generics, syn::parse_quote!(#field_ty: ::std::ops::MulAssign));
    let (mul_assign_impl_generics, _, mul_assign_where) = mul_assign_generics.split_for_impl();

    let div_assign_generics =
        add_bound(generics, syn::parse_quote!(#field_ty: ::std::ops::DivAssign));
    let (div_assign_impl_generics, _, div_assign_where) = div_assign_generics.split_for_impl();

    let eq_generics =
        add_bound(generics, syn::parse_quote!(#field_ty: ::std::cmp::PartialEq));
    let (eq_impl_generics, _, eq_where) = eq_generics.split_for_impl();

    let ord_generics =
        add_bound(generics, syn::parse_quote!(#field_ty: ::std::cmp::PartialOrd));
    let (ord_impl_generics, _, ord_where) = ord_generics.split_for_impl();

    let sum_generics = add_bound(
        &add_bound(generics, syn::parse_quote!(#field_ty: ::std::iter::Sum)),
        syn::parse_quote!(#name #ty_generics: ::std::default::Default),
    );
    let (sum_impl_generics, _, sum_where) = sum_generics.split_for_impl();

    let product_generics = add_bound(
        &add_bound(generics, syn::parse_quote!(#field_ty: ::std::iter::Product)),
        syn::parse_quote!(#name #ty_generics: ::std::default::Default),
    );
    let (product_impl_generics, _, product_where) = product_generics.split_for_impl();

    let zero_generics = add_bound(
        &add_bound(generics, syn::parse_quote!(#field_ty: ::num_traits::Zero)),
        syn::parse_quote!(#name #ty_generics: ::std::default::Default),
    );
    let (zero_impl_generics, _, zero_where) = zero_generics.split_for_impl();

    let one_generics = add_bound(
        &add_bound(generics, syn::parse_quote!(#field_ty: ::num_traits::One)),
        syn::parse_quote!(#name #ty_generics: ::std::default::Default),
    );
    let (one_impl_generics, _, one_where) = one_generics.split_for_impl();

    quote! {
        #input_ast

        impl #neg_impl_generics ::std::ops::Neg for #name #ty_generics #neg_where {
            type Output = Self;
            fn neg(self) -> Self {
                Self { #field_ident: -self.#field_ident, ..self }
            }
        }

        impl #add_impl_generics ::std::ops::Add for #name #ty_generics #add_where {
            type Output = Self;
            fn add(self, other: Self) -> Self {
                Self { #field_ident: self.#field_ident + other.#field_ident, ..self }
            }
        }

        impl #sub_impl_generics ::std::ops::Sub for #name #ty_generics #sub_where {
            type Output = Self;
            fn sub(self, other: Self) -> Self {
                Self { #field_ident: self.#field_ident - other.#field_ident, ..self }
            }
        }

        impl #mul_impl_generics ::std::ops::Mul for #name #ty_generics #mul_where {
            type Output = Self;
            fn mul(self, other: Self) -> Self {
                Self { #field_ident: self.#field_ident * other.#field_ident, ..self }
            }
        }

        impl #div_impl_generics ::std::ops::Div for #name #ty_generics #div_where {
            type Output = Self;
            fn div(self, other: Self) -> Self {
                Self { #field_ident: self.#field_ident / other.#field_ident, ..self }
            }
        }

        impl #mul_impl_generics ::std::ops::Mul<#field_ty> for #name #ty_generics #mul_where {
            type Output = Self;
            fn mul(self, other: #field_ty) -> Self {
                Self { #field_ident: self.#field_ident * other, ..self }
            }
        }

        impl #div_impl_generics ::std::ops::Div<#field_ty> for #name #ty_generics #div_where {
            type Output = Self;
            fn div(self, other: #field_ty) -> Self {
                Self { #field_ident: self.#field_ident / other, ..self }
            }
        }

        impl #add_assign_impl_generics ::std::ops::AddAssign for #name #ty_generics #add_assign_where {
            fn add_assign(&mut self, other: Self) {
                self.#field_ident += other.#field_ident;
            }
        }

        impl #sub_assign_impl_generics ::std::ops::SubAssign for #name #ty_generics #sub_assign_where {
            fn sub_assign(&mut self, other: Self) {
                self.#field_ident -= other.#field_ident;
            }
        }

        impl #mul_assign_impl_generics ::std::ops::MulAssign for #name #ty_generics #mul_assign_where {
            fn mul_assign(&mut self, other: Self) {
                self.#field_ident *= other.#field_ident;
            }
        }

        impl #mul_assign_impl_generics ::std::ops::MulAssign<#field_ty>
            for #name #ty_generics #mul_assign_where
        {
            fn mul_assign(&mut self, other: #field_ty) {
                self.#field_ident *= other;
            }
        }

        impl #div_assign_impl_generics ::std::ops::DivAssign for #name #ty_generics #div_assign_where {
            fn div_assign(&mut self, other: Self) {
                self.#field_ident /= other.#field_ident;
            }
        }

        impl #div_assign_impl_generics ::std::ops::DivAssign<#field_ty>
            for #name #ty_generics #div_assign_where
        {
            fn div_assign(&mut self, other: #field_ty) {
                self.#field_ident /= other;
            }
        }

        impl #eq_impl_generics ::std::cmp::PartialEq for #name #ty_generics #eq_where {
            fn eq(&self, other: &Self) -> bool {
                self.#field_ident == other.#field_ident
            }
        }

        impl #ord_impl_generics ::std::cmp::PartialOrd for #name #ty_generics #ord_where {
            fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
                self.#field_ident.partial_cmp(&other.#field_ident)
            }
        }

        impl #sum_impl_generics ::std::iter::Sum for #name #ty_generics #sum_where {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                let total = iter.map(|x| x.#field_ident).sum();
                Self { #field_ident: total, ..Default::default() }
            }
        }

        impl #product_impl_generics ::std::iter::Product for #name #ty_generics #product_where {
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                let total = iter.map(|x| x.#field_ident).product();
                Self { #field_ident: total, ..Default::default() }
            }
        }

        impl #zero_impl_generics ::num_traits::Zero for #name #ty_generics #zero_where {
            fn zero() -> Self {
                Self { #field_ident: <#field_ty as ::num_traits::Zero>::zero(), ..Default::default() }
            }

            fn is_zero(&self) -> bool {
                ::num_traits::Zero::is_zero(&self.#field_ident)
            }
        }

        impl #one_impl_generics ::num_traits::One for #name #ty_generics #one_where {
            fn one() -> Self {
                Self { #field_ident: <#field_ty as ::num_traits::One>::one(), ..Default::default() }
            }
        }
    }
    .into()
}
