# struct_extractors

Small Rust procedural macros for generating focused behavior from selected struct fields and for compile-time enum-entry checks.

The crate is intentionally split by concept so that each macro family has one responsibility and one matching test file.

## Macro families

- `accessors`: generate value, shared-reference, or mutable-reference accessors from `#[access(...)]` field markers.
- `comparators`: generate named comparator functions for selected fields.
- `hashers`: generate lightweight hash/equality wrapper types for selected fields.
- `number`: derive arithmetic, assignment, comparison, aggregation, and `num_traits::Zero` / `One` behavior from one selected field. Binary and unary operations preserve the other fields from the left-hand value; `Sum`, `Product`, `Zero`, and `One` initialize other fields through `Default`.
- `enum_iter`: generate ordered variants and an owned iterator for unit-only enums.\n- `entries`: perform compile-time subset checks that every variant of an annotated enum exists in one or more declared base enums.

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

## Enum iteration

Apply `#[enum_iter]` to a fieldless enum to generate `Enum::VARIANTS` and
`Enum::iter()`. Both preserve declaration order. The iterator owns each value,
requires neither `Copy` nor `Clone`, and introduces no runtime dependencies.

```rust
use struct_extractors::enum_iter;

#[enum_iter]
#[derive(Debug, PartialEq, Eq)]
enum State {
    New,
    Running,
    Finished,
}

assert_eq!(State::VARIANTS.len(), 3);
assert_eq!(
    State::iter().collect::<Vec<_>>(),
    vec![State::New, State::Running, State::Finished],
);
```

Unit variants with explicit discriminants are supported, as are empty enums
and generic enums. Tuple and struct-like variants produce a compile-time
error: generating their values would require field data. Unlike the
`strum` crate, this macro is intentionally limited to unit variants.

## Design goals

- Keep each procedural-macro concept in its own source module.
- Generate small, explicit APIs instead of introducing a runtime abstraction layer.
- Treat value getters as `Copy` accessors; use reference or mutable-reference accessors for non-`Copy` fields.
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

`lib.rs` contains the required crate-root `#[proc_macro_attribute]` entrypoints as thin wrappers; the implementations live in the concept modules.

## Testing

Consistent test layout: tests live under `tests/`, mirror the relative `src/` hierarchy where relevant, and use the source filename with a `_test.rs` suffix.

For this crate:

```text
src/accessors.rs    -> tests/accessors_test.rs
src/comparators.rs  -> tests/comparators_test.rs
src/entries.rs      -> tests/entries_test.rs and tests/enum_iter_test.rs
src/hashers.rs      -> tests/hashers_test.rs
src/number.rs       -> tests/number_test.rs
```

Run the project checks with:

```text
just build
```
