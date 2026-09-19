// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use struct_extractors::extract_comparators;

#[extract_comparators]
pub struct MyStruct<T>
where
    T: Ord,
{
    #[extract_compare(compare_value)]
    pub value: T,
    #[extract_compare(compare_other)]
    pub other: T,
}

#[test]
fn sorts_generic_struct_by_each_marked_field() {
    let mut values = [
        MyStruct { value: 5, other: 1 },
        MyStruct { value: 1, other: 3 },
        MyStruct { value: 3, other: 2 },
    ];

    values.sort_by(MyStruct::compare_value);
    assert_eq!(values.iter().map(|x| x.value).collect::<Vec<_>>(), vec![1, 3, 5]);

    values.sort_by(MyStruct::compare_other);
    assert_eq!(values.iter().map(|x| x.other).collect::<Vec<_>>(), vec![1, 2, 3]);
}
