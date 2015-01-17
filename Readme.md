# Static Slice

Rust macro for succinctly creating static slices.

## Usage

```rust
static_slice![<type>: a1, a2, ..., an]
```

## Examples

```rust
let bytes = static_slice![u8: 1, 3, 5, 7, 8];
```

```rust
enum TruthValue {
    Yes,
    Maybe,
    No
}

fn to_bools(x: TruthValue) -> &'static [bool] {
    match x {
        Yes => static_slice![true],
        Maybe => static_slice![true, false],
        No => static_slice![false]
    }
}
```

## License

Public domain. Go for it.
