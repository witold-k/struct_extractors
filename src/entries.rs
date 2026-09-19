// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use proc_macro::{Ident, TokenStream, TokenTree};

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
