// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use struct_extractors::extract_comparators;

#[extract_comparators]
pub struct MyStruct {
    #[extract_compare(mycmp)]
    pub val: usize,
    pub other: usize,
}

#[test]
fn test_sort_by_val() {
    let mut v = [
        MyStruct { val: 5, other: 100 },
        MyStruct { val: 1, other: 200 },
        MyStruct { val: 3, other: 300 },
    ];

    v.sort_by(MyStruct::mycmp);

    let result: Vec<usize> = v.iter().map(|x| x.val).collect();
    assert_eq!(result, vec![1, 3, 5]);
}

