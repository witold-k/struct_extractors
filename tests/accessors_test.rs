// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use struct_extractors::extract_accessors;

#[extract_accessors]
#[derive(Clone, Copy, Default)]
pub struct MyStruct {
    #[access(get)]
    field1: usize,
    #[access(get_ref)]
    field2: usize
}

impl MyStruct {
    fn new() -> Self {
        Self { field1: 1, field2: 2 }
    }
}

#[test]
fn test_basic_arithmetic() {
    let a = MyStruct::new();
    assert!(a.get_field1() == 1);
    let a = MyStruct::new();
    assert!(*a.get_ref_field2() == 2);
}
