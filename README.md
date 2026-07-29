# multi-base

[![](https://img.shields.io/badge/made%20by-Cryptid%20Technologies-gold.svg?style=flat-square)](https://cryptid.tech/)
[![](https://img.shields.io/badge/project-multiformats-blue.svg?style=flat-square)](https://github.com/multiformats/multiformats)

[![Build Status](https://github.com/cryptidtech/multi-base/workflows/build/badge.svg)](https://github.com/cryptidtech/multi-base/actions)
[![License](https://img.shields.io/crates/l/multi-base?style=flat-square)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/multi-base?style=flat-square)](https://crates.io/crates/multi-base)
[![Documentation](https://docs.rs/multi-base/badge.svg?style=flat-square)](https://docs.rs/multi-base)
[![Coverage Status](https://img.shields.io/codecov/c/github/cryptidtech/multi-base?style=flat-square)](https://codecov.io/gh/cryptidtech/multi-base)

A [multibase](https://github.com/multiformats/multibase) implementation in
Rust. The crate gives encoding and decoding with error handling, type safety,
and `no_std` support.

The crate is published as `multi-base` on crates.io. Import it as
`multi_base` in Rust.

## Table of Contents

- [Features](#features)
- [Install](#install)
- [Usage](#usage)
  - [Basic Usage](#basic-usage)
  - [Buffer Reuse for Performance](#buffer-reuse-for-performance)
  - [Type Safety with EncodedString](#type-safety-with-encodedstring)
  - [Error Handling](#error-handling)
- [Supported Bases](#supported-bases)
- [Performance](#performance)
- [Security](#security)
- [Concurrency](#concurrency)
- [CLI Tool](#cli-tool)
- [Testing](#testing)
- [Maintainers](#maintainers)
- [Contribute](#contribute)
- [License](#license)

## Features

- 24 base encodings.
- Zero-copy buffer reuse APIs: `encode_into` and `decode_into`.
- `#[inline]` on hot encode and decode paths.
- Pre-allocated exact-capacity encoding.
- Validated `EncodedString` newtype. Parse, do not validate.
- `#![deny(unsafe_code)]` set at the crate root.
- No panics on untrusted input.
- Fuzzing infrastructure with `cargo-fuzz`.
- All types are `Send + Sync`. No interior mutability.
- `no_std` support with `alloc`.
- WebAssembly compatible.
- 142 tests: unit, integration, property-based, security, concurrency, and
  documentation tests.

## Install

Add this to your `Cargo.toml`:

```toml
[dependencies]
multi-base = "1.1"
```

For `no_std` environments:

```toml
[dependencies]
multi-base = { version = "1.1", default-features = false }
```

MSRV: Rust 1.85 (Edition 2024).

## Usage

### Basic Usage

```rust
use multi_base::{Base, encode, decode};

// Encode data
let encoded = encode(Base::Base64, b"hello world");
println!("{}", encoded); // "maGVsbG8gd29ybGQ"

// Decode data
let (base, data) = decode(&encoded, true)?;
assert_eq!(base, Base::Base64);
assert_eq!(data, b"hello world");
```

### Buffer Reuse for Performance

When you encode or decode multiple values, reuse buffers to avoid
allocations:

```rust
use multi_base::{Base, encode_into, decode_into};

let mut encode_buffer = String::new();
let mut decode_buffer = Vec::new();

for data in dataset {
    // Encode into the existing buffer. No allocation.
    encode_into(Base::Base64, data, &mut encode_buffer);

    // Decode into the existing buffer. No allocation.
    let base = decode_into(&encode_buffer, true, &mut decode_buffer)?;

    // Process the decoded data.
}
```

### Type Safety with EncodedString

Use `EncodedString` for validated multibase strings:

```rust
use multi_base::{EncodedString, Base};

// Parse and validate at construction
let encoded = EncodedString::new("zCn8eVZg")?;

// The base is known at compile time
assert_eq!(encoded.base(), Base::Base58Btc);

// Decode directly
let data = encoded.decode()?;
assert_eq!(data, b"hello");

// Or use FromStr
let encoded: EncodedString = "md29ybGQ".parse()?;
```

### Error Handling

The crate gives error types with context:

```rust
use multi_base::{decode, Error};

match decode(input, true) {
    Ok((base, data)) => {
        println!("Decoded with {:?}: {:?}", base, data);
    }
    Err(Error::UnknownBase { code }) => {
        eprintln!("Unknown base code: {}", code);
    }
    Err(Error::EmptyInput) => {
        eprintln!("Input string is empty");
    }
    Err(Error::DataEncodingDecode { message }) => {
        eprintln!("Decoding failed: {}", message);
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

## Supported Bases

The crate supports 24 base encodings:

| Base | Code | Alphabet |
|------|------|----------|
| Identity | `\0` | 8-bit binary (no encoding) |
| Base2 | `0` | `01` |
| Base8 | `7` | `01234567` |
| Base10 | `9` | `0123456789` |
| Base16 (Lower) | `f` | `0123456789abcdef` |
| Base16 (Upper) | `F` | `0123456789ABCDEF` |
| Base32 (Lower) | `b` | RFC 4648 (no padding) |
| Base32 (Upper) | `B` | RFC 4648 (no padding) |
| Base32Pad (Lower) | `c` | RFC 4648 (with padding) |
| Base32Pad (Upper) | `C` | RFC 4648 (with padding) |
| Base32Hex (Lower) | `v` | RFC 4648 hex (no padding) |
| Base32Hex (Upper) | `V` | RFC 4648 hex (no padding) |
| Base32HexPad (Lower) | `t` | RFC 4648 hex (with padding) |
| Base32HexPad (Upper) | `T` | RFC 4648 hex (with padding) |
| Base32Z | `h` | z-base-32 (Tahoe-LAFS) |
| Base36 (Lower) | `k` | `0123456789abcdefghijklmnopqrstuvwxyz` |
| Base36 (Upper) | `K` | `0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ` |
| Base58 Flickr | `Z` | Flickr alphabet |
| Base58 Bitcoin | `z` | Bitcoin alphabet |
| Base64 | `m` | RFC 4648 (no padding) |
| Base64Pad | `M` | RFC 4648 (with padding) |
| Base64Url | `u` | RFC 4648 URL-safe (no padding) |
| Base64UrlPad | `U` | RFC 4648 URL-safe (with padding) |
| Base256Emoji | `🚀` | Emoji alphabet |

## Performance

Base32 and Base64 are faster than other bases. This is because of byte
alignment.

Optimization tips:

1. Use `encode_into()` and `decode_into()` for buffer reuse in loops.
2. Prefer Base32 or Base64 for performance-critical code.
3. Use Base58 or Base16 when human readability is important.

Run `cargo bench` to see performance on your system.

## Security

- No panics on untrusted input.
- `#![deny(unsafe_code)]` set at the crate root.
- Input validation at all boundaries.
- 17 security tests.
- Fuzzing infrastructure with 3 targets: `fuzz_decode`, `fuzz_encode`,
  and `fuzz_roundtrip`.

Best practices:

- For untrusted input, use strict mode: `decode(input, true)`.
- Set application-level size limits. See [SECURITY.md](SECURITY.md).
- For binary data preservation, do not use Identity encoding. Use Base64.

See [SECURITY.md](SECURITY.md) for the full security policy.

## Concurrency

All public types are thread-safe:

- All types implement `Send + Sync`.
- No interior mutability.
- No data races.
- 20 thread safety tests verify these properties.

Concurrent usage:

```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(b"data".to_vec());
let handles: Vec<_> = (0..10)
    .map(|_| {
        let d = Arc::clone(&data);
        thread::spawn(move || {
            multi_base::encode(Base::Base64, &*d)
        })
    })
    .collect();

for handle in handles {
    let encoded = handle.join().unwrap();
    // All threads give the same result.
}
```

See [CONCURRENCY.md](CONCURRENCY.md) for detailed concurrency information.

## CLI Tool

The crate includes a command-line tool in the `cli/` directory.

Build the CLI:

```bash
cd cli
cargo build --release
```

Example usage:

```bash
# Encode data
echo "hello world" | multibase encode --base base64

# Decode data
echo "maGVsbG8gd29ybGQ" | multibase decode

# Specify input directly
multibase encode --base base58btc --input "hello world"
```

## Testing

The crate has 142 tests:

- 12 unit tests.
- 63 integration tests.
- 16 property-based tests with proptest.
- 17 security tests.
- 20 thread safety tests.
- 14 documentation tests.

Run all tests:

```bash
cargo test --all
```

Run specific test suites:

```bash
cargo test --test lib          # Integration tests
cargo test --test properties   # Property-based tests
cargo test --test security     # Security tests
cargo test --test thread_safety # Concurrency tests
```

Run benchmarks:

```bash
cargo bench
```

Run fuzzing (requires `cargo-fuzz`):

```bash
cargo install cargo-fuzz
cargo fuzz run fuzz_decode
cargo fuzz run fuzz_encode
cargo fuzz run fuzz_roundtrip
```

## Documentation

Generate and view the documentation:

```bash
cargo doc --open
```

Additional documentation:

- [SECURITY.md](SECURITY.md) — Security review and best practices.
- [CONCURRENCY.md](CONCURRENCY.md) — Thread safety analysis.
- [CHANGELOG.md](CHANGELOG.md) — Version history and migration notes.

## Maintainers

This repo: [@dhuseby](https://github.com/dhuseby).

Original author: [@dignifiedquire](https://github.com/dignifiedquire).

## Contribute

Contributions are welcome. Please check out
[the issues](https://github.com/cryptidtech/multi-base/issues).

### Development Guidelines

- Run `cargo fmt` before you commit.
- Run `cargo clippy -- -D warnings` to check for issues.
- Add tests for new features.
- Update documentation for API changes.
- Run the full test suite: `cargo test --all`.

## License

[MIT](LICENSE) © Friedel Ziegelmayer