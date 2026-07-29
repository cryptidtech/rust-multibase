# Security Policy

## Security Review

This document describes the security posture of the `multi-base` crate. It
gives guidance for users on security considerations.

## Security Review Results

### No Critical Vulnerabilities Found

The codebase was reviewed for common security issues. No critical
vulnerabilities were found.

### Areas Reviewed

1. Integer Overflow. Safe.
   - All size calculations use checked arithmetic in debug mode.
   - Capacity calculations for `String` and `Vec` use addition. The
     allocation fails before overflow.
   - Risk: Low. An attack would need near-`usize::MAX` inputs. Memory
     allocation fails first.

2. Buffer Overflow and Underflow. Safe.
   - All string slicing uses `char::len_utf8()` for UTF-8 boundary
     detection.
   - No unsafe code or manual pointer arithmetic.
   - Rust bounds checking prevents buffer overflows.
   - Risk: None. Protected by Rust safety guarantees.

3. Panic Conditions. Safe.
   - Identity encoding uses `String::from_utf8_lossy`. No panic on
     invalid UTF-8.
   - All decode operations return `Result`.
   - Empty inputs return `Error::EmptyInput`.
   - Invalid base codes return `Error::UnknownBase`.
   - One `.expect()` in `encode_to_validated()`. It cannot fail in
     practice.
   - Risk: Minimal. Library functions do not panic on arbitrary input.

4. Resource Exhaustion Attacks. Consider.
   - No hard limits on input size.
   - Large inputs consume proportional memory.
   - Memory allocation failures are handled by the Rust allocator.
   - Risk: Medium. Applications should set their own size limits.
   - Recommendation: Applications that process untrusted input should
     enforce a maximum size limit.

5. Input Validation. Complete.
   - Empty strings return `Error::EmptyInput`.
   - Invalid base codes return `Error::UnknownBase`.
   - Malformed encoded data is rejected by base-specific decoders.
   - All validation goes through `Result` types.
   - Risk: None. Input validation at all boundaries.

## Security Best Practices for Users

### 1. Input Size Limits

For applications that process untrusted input, set a maximum size limit:

```rust
const MAX_INPUT_SIZE: usize = 10 * 1024 * 1024; // 10 MB

fn safe_decode(input: &str) -> Result<(Base, Vec<u8>), Error> {
    if input.len() > MAX_INPUT_SIZE {
        return Err(Error::InvalidBaseString);
    }
    multi_base::decode(input, true)
}
```

### 2. Error Handling

Handle errors. Do not expose detailed error messages to untrusted parties:

```rust
match multi_base::decode(untrusted_input, true) {
    Ok((base, data)) => {
        // Process data
    }
    Err(_) => {
        // Log the error internally. Return a generic error to the user.
        eprintln!("Invalid multibase input");
    }
}
```

### 3. Identity Encoding

The Identity encoding (`\0` prefix) uses lossy UTF-8 conversion:

- Invalid UTF-8 bytes are replaced with the Unicode replacement character
  (U+FFFD).
- This prevents panics. Invalid UTF-8 does not round-trip.
- For binary data that must round-trip, use a different base encoding.

```rust
// For binary data preservation, use Base64 or Base58
let encoded = multi_base::encode(Base::Base64, binary_data);

// Identity is for UTF-8 text only
let text_encoded = multi_base::encode(Base::Identity, "valid utf-8 text".as_bytes());
```

### 4. Strict vs Permissive Decoding

Use strict decoding for untrusted input:

```rust
// For untrusted input, use strict mode
let (base, data) = multi_base::decode(untrusted, true)?;

// Permissive mode allows case-insensitive decoding for some bases
let (base, data) = multi_base::decode(trusted, false)?;
```

## Security Testing

The crate has 17 security tests in `tests/security.rs`. They cover:

- Large input handling (up to 1 MB).
- Malformed and malicious input patterns.
- Buffer reuse safety.
- Concurrent operation safety.
- Resource exhaustion resistance.
- Integer overflow safety.
- Invalid UTF-8 handling.
- Empty and truncated input handling.

Run security tests:

```bash
cargo test --test security
```

## Fuzzing

The crate can be fuzzed with `cargo-fuzz`. Fuzz targets:

1. `fuzz_decode` — Decode arbitrary strings. Checks for panics on any
   input.
2. `fuzz_encode` — Encode arbitrary bytes. Checks for panics on any
   binary data.
3. `fuzz_roundtrip` — Encode and decode round-trip. Checks encode and
   decode consistency.

### Set Up Fuzzing

```bash
# Install cargo-fuzz
cargo install cargo-fuzz

# Run fuzz tests
cargo fuzz run fuzz_decode
cargo fuzz run fuzz_encode
cargo fuzz run fuzz_roundtrip
```

## Dependency Security

The crate depends on:

- `base-x` (0.2.7) — Variable-radix base encoding.
- `data-encoding` (2.3.1) — Standard base encodings.
- `thiserror` (2.0) — Error handling.

The `Base256Emoji` codec is implemented inline. The external
`base256emoji` dependency was dropped. See `CHANGELOG.md`.

All dependencies are maintained and used in the Rust ecosystem.

## Reporting Security Issues

If you find a security vulnerability in the `multi-base` crate, report it
as follows:

1. Do not open a public GitHub issue.
2. Contact the maintainers by email. See `Cargo.toml` for the address.
3. Give detailed information about the vulnerability.
4. Allow reasonable time for a fix before public disclosure.

## Security Guarantees

### What the Crate Guarantees

- No panics on arbitrary untrusted input.
- Memory safety. No unsafe code is used.
- Input validation at all boundaries.
- Thread-safe operations. All types are `Send + Sync`.
- Error information without exposure of internal state.

### What the Crate Does Not Guarantee

- Protection against resource exhaustion. This is an application
  responsibility.
- Constant-time operations. The crate is not for cryptographic use.
- Perfect round-trip of invalid UTF-8 in Identity encoding.

## Changelog of Security-Related Changes

### Version 1.0.3
- Updated `SECURITY.md` code examples to use `multi_base::` instead of
  the old `multibase::` module path.
- Corrected the stale `base256emoji` dependency reference.

### Version 1.0.2
- Added `#![deny(unsafe_code)]` at the crate root.
- Added clippy lint configuration with `unsafe_code = "deny"`.
- Added `cargo audit` job in CI.

### Version 1.0.1
- Fixed Identity encoding panic risk. Now uses lossy UTF-8 conversion.
- Migrated to `thiserror` for error handling.
- Added 17 security tests.
- Added this `SECURITY.md` document.

### Version 1.0.0
- Initial release with basic security considerations.