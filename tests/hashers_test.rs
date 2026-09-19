// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use struct_extractors::extract_hashers;
use std::hash::{Hash, Hasher, DefaultHasher};
use std::collections::HashMap;

#[derive(Debug)]
#[extract_hashers]
pub struct MyStruct {
    #[extract_hash]
    pub val: usize,
    pub other: usize,
}

#[test]
fn test_hash_val() {
    let a = MyStruct { val: 10, other: 1 };
    let b = MyStruct { val: 10, other: 999 };

    let mut h_a = DefaultHasher::new();
    let hv_a = MyStructHashByval(&a);

    let mut h_b = DefaultHasher::new();
    let hv_b = MyStructHashByval(&b);

    hv_a.hash(&mut h_a);
    hv_b.hash(&mut h_b);

    let hv_a = h_a.finish();
    let hv_b = h_b.finish();
    assert_eq!(hv_a, hv_b);

    let mut h_c = DefaultHasher::new();
    let c = MyStruct { val: 20, other: 1 };
    let hv_c = MyStructHashByval(&c);
    hv_c.hash(&mut h_c);
    let hv_c = h_c.finish();

    assert_ne!(hv_a, hv_c);

    //h.finish()
}

#[test]
fn test_hash_map() {
    let a = MyStruct { val: 10, other: 1 };
    let b = MyStruct { val: 10, other: 999 };
    let c = MyStruct { val: 20, other: 1 };

    // --- neuer Teil: HashMap-Test ---
    let mut map: HashMap<MyStructHashByval<'_>, &str> = HashMap::new();

    map.insert(MyStructHashByval(&a), "A");
    map.insert(MyStructHashByval(&b), "B"); // overwrites "A"
    map.insert(MyStructHashByval(&c), "C");

    // a und b haben denselben Hash → gleicher Key → letzter gewinnt
    assert_eq!(map.get(&MyStructHashByval(&a)), Some(&"B"));
    assert_eq!(map.get(&MyStructHashByval(&b)), Some(&"B"));

    // c hat anderen Hash → eigener Eintrag
    assert_eq!(map.get(&MyStructHashByval(&c)), Some(&"C"));
}

