# struct_extractors

Small Rust procedural macros for generating focused behavior from selected struct fields and for compile-time enum-entry checks.

The crate is intentionally split by concept so that each macro family has one responsibility and one matching test file.

## Macro families

- `accessors`: generate value, shared-reference, or mutable-reference accessors from `#[access(...)]` field markers.
- `comparators`: generate named comparator functions for selected fields.
- `hashers`: generate lightweight hash/equality wrapper types for selected fields.
- `number`: derive arithmetic, assignment, comparison, aggregation, and `num_traits::Zero` / `One` behavior from one numeric field.
- `entries`: perform compile-time checks that enum variants exist in one or more declared base enums.

## Example

```rust
use struct_extractors::extract_accessors;

#[extract_accessors]
struct Config {
    #[access(get)]
    retries: usize,

    #[access(get_ref)]
    name: String,
}

let config = Config {
    retries: 3,
    name: String::from("worker"),
};

assert_eq!(config.get_retries(), 3);
assert_eq!(config.get_ref_name(), "worker");
```

## Design goals

- Keep each procedural-macro concept in its own source module.
- Generate small, explicit APIs instead of introducing a runtime abstraction layer.
- Preserve the annotated item and remove only helper attributes consumed by the corresponding struct-level macro.
- Keep generated behavior predictable from the field annotations.
- Prefer compile-time validation where the macro naturally supports it.
- Avoid dependencies beyond those needed for procedural-macro parsing and generated numeric traits.

## Source layout

```text
src/
├── accessors.rs
├── comparators.rs
├── entries.rs
├── hashers.rs
├── lib.rs
└── number.rs
```

`lib.rs` only declares the concept modules and re-exports their procedural macros.

## Testing

Consistent test layout: tests live under `tests/`, mirror the relative `src/` hierarchy where relevant, and use the source filename with a `_test.rs` suffix.

For this crate:

```text
src/accessors.rs    -> tests/accessors_test.rs
src/comparators.rs  -> tests/comparators_test.rs
src/entries.rs      -> tests/entries_test.rs
src/hashers.rs      -> tests/hashers_test.rs
src/number.rs       -> tests/number_test.rs
```

Run the project checks with:

```text
just build
```
