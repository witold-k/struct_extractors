// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use proc_macro::TokenStream;

mod accessors;
mod comparators;
mod entries;
mod hashers;
mod number;

#[proc_macro_attribute]
pub fn access(args: TokenStream, input: TokenStream) -> TokenStream {
    accessors::access_impl(args, input)
}

#[proc_macro_attribute]
pub fn extract_accessors(args: TokenStream, input: TokenStream) -> TokenStream {
    accessors::extract_accessors_impl(args, input)
}

#[proc_macro_attribute]
pub fn extract_compare(args: TokenStream, input: TokenStream) -> TokenStream {
    comparators::extract_compare_impl(args, input)
}

#[proc_macro_attribute]
pub fn extract_comparators(args: TokenStream, input: TokenStream) -> TokenStream {
    comparators::extract_comparators_impl(args, input)
}

#[proc_macro_attribute]
pub fn base_entries(args: TokenStream, input: TokenStream) -> TokenStream {
    entries::base_entries_impl(args, input)
}

#[proc_macro_attribute]
pub fn same_entries(args: TokenStream, input: TokenStream) -> TokenStream {
    entries::same_entries_impl(args, input)
}

#[proc_macro_attribute]
pub fn extract_hash(args: TokenStream, input: TokenStream) -> TokenStream {
    hashers::extract_hash_impl(args, input)
}

#[proc_macro_attribute]
pub fn extract_hashers(args: TokenStream, input: TokenStream) -> TokenStream {
    hashers::extract_hashers_impl(args, input)
}

#[proc_macro_attribute]
pub fn extract_number(args: TokenStream, input: TokenStream) -> TokenStream {
    number::extract_number_impl(args, input)
}
