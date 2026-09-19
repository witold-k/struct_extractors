// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use struct_extractors::{base_entries, same_entries};

#[base_entries(E1)]
pub enum E1 {
    V11,
    V22,
}

#[same_entries(E1)]
pub enum E2 {
    V11,
    V22,
}

#[same_entries(E1)]
pub enum E3 {
    V11,
    V22,
    // V33, // ❌ compile-time error: `__BASE_ENTRIES_E1::V33` does not exist
}

