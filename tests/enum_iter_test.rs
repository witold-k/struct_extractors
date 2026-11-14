// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use struct_extractors::enum_iter;

#[enum_iter]
#[derive(Debug, PartialEq, Eq)]
enum Color {
    Red,
    Green,
    Blue,
}

#[test]
fn iterates_in_declaration_order() {
    assert_eq!(Color::iter().collect::<Vec<_>>(), vec![Color::Red, Color::Green, Color::Blue]);
    assert_eq!(Color::VARIANTS, [Color::Red, Color::Green, Color::Blue]);
}

#[enum_iter]
#[derive(Debug, PartialEq, Eq)]
enum Empty {}

#[test]
fn empty_enum_has_no_variants() {
    assert_eq!(Empty::VARIANTS.len(), 0);
    assert_eq!(Empty::iter().count(), 0);
}

#[enum_iter]
#[derive(Debug, PartialEq, Eq)]
enum WithDiscriminants {
    First = 10,
    Second = 20,
}

#[test]
fn supports_explicit_discriminants() {
    assert_eq!(
        WithDiscriminants::iter().collect::<Vec<_>>(),
        vec![WithDiscriminants::First, WithDiscriminants::Second]
    );
}

#[enum_iter]
#[derive(Debug, PartialEq, Eq)]
enum Generic<const N: usize> {
    First,
    Second,
}

#[test]
fn supports_const_generics() {
    assert_eq!(Generic::<4>::iter().count(), 2);
    assert_eq!(Generic::<4>::VARIANTS, [Generic::First, Generic::Second]);
}

#[enum_iter]
#[derive(Debug, PartialEq, Eq)]
enum NonCopy {
    First,
    Second,
}

#[test]
fn iterates_without_a_copy_derive() {
    assert_eq!(NonCopy::iter().collect::<Vec<_>>(), vec![NonCopy::First, NonCopy::Second]);
}

#[enum_iter]
#[derive(Debug, PartialEq, Eq)]
enum WithWhere<const N: usize>
where
    [u8; N]: Sized,
{
    A,
    B,
}

#[test]
fn preserves_where_clauses() {
    assert_eq!(WithWhere::<2>::iter().count(), 2);
}
