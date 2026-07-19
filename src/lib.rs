// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use proc_macro::{
    TokenStream,
    TokenTree,
    Ident
};
use quote::{quote, format_ident};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated, Token,
    parse_macro_input, DeriveInput, Data, Expr,
};

// --- syn 2.0 argument parser -----------------------------------------------

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


// --- extract number ---------------------------------------------------------

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

// --- extract compare --------------------------------------------------------

/* -------------------------------------------------------------------------
   FIELD ATTRIBUTE MACRO
   ------------------------------------------------------------------------- */

/// This macro is placed on *fields*. It must return the field unchanged.
/// We only use it as a marker; the struct-level macro will read it.
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

// --- extract access ---------------------------------------------------------

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

// --- extract hash -----------------------------------------------------------

/* -------------------------------------------------------------------------
   FIELD ATTRIBUTE MACRO
   ------------------------------------------------------------------------- */

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

#[proc_macro_attribute]
pub fn base_entries(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut tokens = item.clone().into_iter();
    let mut enum_name = None;

    // Find identifier after `enum`
    while let Some(tt) = tokens.next() {
        if let TokenTree::Ident(id) = tt
            && id.to_string() == "enum" {
                if let Some(TokenTree::Ident(name)) = tokens.next() {
                    enum_name = Some(name);
                }
                break;
            }
    }

    let enum_name = enum_name.expect("Could not find enum name");
    let mod_ident = Ident::new(
        &format!("__BASE_ENTRIES_{}", enum_name),
        enum_name.span(),
    );

    // Collect variant names
    let mut variants = Vec::new();
    for tt in item.clone() {
        if let TokenTree::Group(g) = tt
            && g.delimiter() == proc_macro::Delimiter::Brace {
                for inner in g.stream() {
                    if let TokenTree::Ident(id) = inner {
                        variants.push(id.to_string());
                    }
                }
            }
    }

    // Generate module with structs
    let mut out = item.to_string();
    out.push_str("\npub mod ");
    out.push_str(&mod_ident.to_string());
    out.push_str(" {");

    for v in variants {
        out.push_str("\n    pub struct ");
        out.push_str(&v);
        out.push(';');
    }

    out.push_str("\n}");

    out.parse().unwrap()
}

#[proc_macro_attribute]
pub fn same_entries(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse attribute: E1, E2, E3
    let attr_string = attr.to_string();
    let referenced: Vec<String> =
        attr_string.split(',').map(|s| s.trim().to_string()).collect();

    // Find THIS enum's name
    let mut tokens = item.clone().into_iter();
    let mut this_enum_name = None;

    while let Some(tt) = tokens.next() {
        if let TokenTree::Ident(id) = tt
            && id.to_string() == "enum" {
                if let Some(TokenTree::Ident(name)) = tokens.next() {
                    this_enum_name = Some(name.to_string());
                }
                break;
            }
    }

    let this_enum_name = this_enum_name.expect("Could not find enum name");

    // Collect this enum's variants
    let mut variants = Vec::new();
    for tt in item.clone() {
        if let TokenTree::Group(g) = tt
            && g.delimiter() == proc_macro::Delimiter::Brace {
                for inner in g.stream() {
                    if let TokenTree::Ident(id) = inner {
                        variants.push(id.to_string());
                    }
                }
            }
    }

    // Build unique module name
    let check_mod_name = format!("__same_entries_check_{}", this_enum_name);

    let mut check = String::new();
    check.push_str("\nmod ");
    check.push_str(&check_mod_name);
    check.push_str(" {");

    // For each referenced enum, check all variants
    for base in referenced {
        let base_mod = format!("__BASE_ENTRIES_{}", base);

        check.push_str("\n    use super::");
        check.push_str(&base_mod);
        check.push(';');

        for v in &variants {
            check.push_str("\n    fn _check_");
            check.push_str(v);
            check.push_str("_from_");
            check.push_str(&base);
            check.push_str("() { let _ : ");
            check.push_str(&base_mod);
            check.push_str("::");
            check.push_str(v);
            check.push_str("; }");
        }
    }

    check.push_str("\n}");

    let mut out = item.to_string();
    out.push_str(&check);

    out.parse().unwrap()
}

