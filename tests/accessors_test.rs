// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use struct_extractors::extract_accessors;

#[extract_accessors]
pub struct MyStruct<T>
where
    T: Default,
{
    #[access(get)]
    count: usize,
    #[access(get_ref)]
    name: String,
    #[access(get_mut)]
    value: T,
    #[access(get = "identifier")]
    id: u64,
}

#[test]
fn accessors_support_generics_and_each_access_mode() {
    let mut value = MyStruct {
        count: 3,
        name: String::from("worker"),
        value: 5_u32,
        id: 42,
    };

    assert_eq!(value.get_count(), 3);
    assert_eq!(value.get_ref_name(), "worker");
    assert_eq!(value.identifier(), 42);

    *value.get_mut_value() = 7;
    assert_eq!(*value.get_mut_value(), 7);
}
