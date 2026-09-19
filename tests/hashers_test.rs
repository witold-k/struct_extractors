// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::collections::HashMap;
use struct_extractors::extract_hashers;

#[derive(Debug)]
#[extract_hashers]
pub struct MyStruct<T>
where
    T: Eq + core::hash::Hash,
{
    #[extract_hash]
    pub value: T,
    #[extract_hash]
    pub group: T,
    pub payload: usize,
}

#[test]
fn hash_wrapper_uses_only_selected_field() {
    let a = MyStruct { value: 10, group: 1, payload: 1 };
    let b = MyStruct { value: 10, group: 2, payload: 999 };
    let c = MyStruct { value: 20, group: 1, payload: 2 };

    assert_eq!(a.hash_by_value(), b.hash_by_value());
    assert_ne!(a.hash_by_value(), c.hash_by_value());

    assert_eq!(a.hash_by_group(), c.hash_by_group());
    assert_ne!(a.hash_by_group(), b.hash_by_group());
}

#[test]
fn hash_wrappers_work_as_hash_map_keys() {
    let a = MyStruct { value: 10, group: 1, payload: 1 };
    let b = MyStruct { value: 10, group: 2, payload: 999 };
    let c = MyStruct { value: 20, group: 1, payload: 2 };

    let mut map = HashMap::new();
    map.insert(a.hash_by_value(), "A");
    map.insert(b.hash_by_value(), "B");
    map.insert(c.hash_by_value(), "C");

    assert_eq!(map.get(&a.hash_by_value()), Some(&"B"));
    assert_eq!(map.get(&c.hash_by_value()), Some(&"C"));
}
