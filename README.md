# chain-if

[![Crates.io](https://img.shields.io/crates/v/chain-if.svg)](https://crates.io/crates/chain-if)
[![Documentation](https://docs.rs/chain-if/badge.svg)](https://docs.rs/chain-if)
[![MSRV](https://img.shields.io/crates/msrv/chain-if)](https://crates.io/crates/chain-if)

Lightweight, `no_std` iterator extensions for conditional chaining with optional lazy evaluation.

Append iterators conditionally without breaking method chains. Supports both eager and lazy construction, with no external dependencies or allocation requirements.

## Quick Example

```rust
use chain_if::ChainIfExt;

let result: Vec<_> = (0..3)
    .chain_if(true, 3..5)
    .chain_if_lazy(false, || 5..7)
    .chain_if_else(true, 7..9, 9..11)
    .collect();

assert_eq!(result, [0, 1, 2, 3, 4, 7, 8]);
```

## Methods

| Method | Behavior |
|---|---|
| `chain_if` | Appends an iterator when the condition is `true`. |
| `chain_if_lazy` | Appends an iterator constructed lazily when the condition is `true`. |
| `chain_if_else` | Appends one of two iterators based on the condition. |
| `chain_if_else_lazy` | Lazily constructs and appends the selected iterator. |

The `_lazy` variants defer factory execution until the appended iterator is first accessed, rather than when the method is called. This also applies to reverse iteration, which accesses the appended iterator first.

See the [API documentation](https://docs.rs/chain-if) for details.

## Installation

```sh
cargo add chain-if
```

Or add it to your `Cargo.toml`:

```toml
[dependencies]
chain-if = "0.1"
```

Import the extension trait with `use chain_if::ChainIfExt;`.

## Compatibility

- `no_std` compatible, using only `core`
- No external dependencies or heap allocation required
- Supports `DoubleEndedIterator` and `FusedIterator` when the applicable trait bounds are satisfied
- Minimum supported Rust version (MSRV): **1.68.0**

## License

Licensed under either the [MIT License](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
