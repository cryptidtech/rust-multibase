# Security Policy

## Security Audit Summary

This document summarizes the security audit conducted on the multibase crate and provides guidance for users regarding security considerations.

## Audit Date

Last comprehensive security audit: 2025-10-08

## Security Review Findings

### ✅ No Critical Vulnerabilities Found

The codebase has been thoroughly reviewed for common security issues and no critical vulnerabilities were identified.

### Areas Reviewed

1. **Integer Overflow** ✓ SAFE
   - All size calculations use Rust's default checked arithmetic in debug mode
   - Capacity calculations for String/Vec use addition that would fail allocation before overflow
   - Risk: Low - would require near-`usize::MAX` inputs, which would fail memory allocation first

2. **Buffer Overflow/Underflow** ✓ SAFE
   - All string slicing uses `char::len_utf8()` for correct UTF-8 boundary detection
   - No unsafe code or manual pointer arithmetic
   - Rust's bounds checking prevents buffer overflows
   - Risk: None - protected by Rust's safety guarantees

3. **Panic Conditions (DoS Vectors)** ✓ SAFE
   - Identity encoding uses `String::from_utf8_lossy` (no panic on invalid UTF-8)
   - All decoding operations return `Result` types
   - Empty inputs handled with `Error::EmptyInput`
   - Invalid base codes return `Error::UnknownBase`
   - One documented `.expect()` in `encode_to_validated()` that cannot fail
   - Risk: Minimal - library functions do not panic on arbitrary inputs

4. **Resource Exhaustion Attacks** ⚠️ CONSIDER
   - No hard limits on input size
   - Large inputs (e.g., gigabytes) will consume proportional memory
   - Memory allocation failures are handled by Rust's allocator
   - Risk: Medium - applications should implement their own size limits if needed
   - **Recommendation**: Applications processing untrusted input should enforce maximum size limits

5. **Input Validation** ✓ COMPREHENSIVE
   - Empty strings: Validated with `Error::EmptyInput`
   - Invalid base codes: Validated with `Error::UnknownBase`
   - Malformed encoded data: Validated by base-specific decoders
   - All validation through `Result` types
   - Risk: None - comprehensive input validation

## Security Best Practices for Users

### 1. Input Size Limits

For applications processing untrusted input, consider enforcing maximum size limits:

```rust
const MAX_INPUT_SIZE: usize = 10 * 1024 * 1024; // 10 MB

fn safe_decode(input: &str) -> Result<(Base, Vec<u8>), Error> {
    if input.len() > MAX_INPUT_SIZE {
        return Err(Error::InvalidBaseString); // or custom error
    }
    multibase::decode(input, true)
}
```

### 2. Error Handling

Always handle errors properly and avoid exposing detailed error messages to untrusted parties:

```rust
match multibase::decode(untrusted_input, true) {
    Ok((base, data)) => {
        // Process data
    }
    Err(_) => {
        // Log error internally, return generic error to user
        eprintln!("Invalid multibase input");
    }
}
```

### 3. Identity Encoding Considerations

The Identity encoding (`\0` prefix) uses lossy UTF-8 conversion:

- Invalid UTF-8 bytes are replaced with the Unicode replacement character (U+FFFD)
- This prevents panics but means invalid UTF-8 won't round-trip perfectly
- For binary data that must round-trip exactly, use a different base encoding (e.g., Base64)

```rust
// For exact binary data preservation, use Base64 or Base58
let encoded = multibase::encode(Base::Base64, binary_data);

// Identity is only appropriate for UTF-8 text
let text_encoded = multibase::encode(Base::Identity, "valid utf-8 text".as_bytes());
```

### 4. Strict vs Permissive Decoding

Use strict decoding (`true`) for untrusted input to ensure stricter validation:

```rust
// For untrusted input, always use strict mode
let (base, data) = multibase::decode(untrusted, true)?;

// Permissive mode allows case-insensitive decoding for some bases
let (base, data) = multibase::decode(trusted, false)?;
```

## Security Testing

The crate includes comprehensive security tests covering:

- **17 security-focused tests** in `tests/security.rs`
- Large input handling (up to 1 MB tested)
- Malformed and malicious input patterns
- Buffer reuse safety
- Concurrent operation safety
- Resource exhaustion resistance
- Integer overflow safety
- Invalid UTF-8 handling
- Empty and truncated input handling

Run security tests with:
```bash
cargo test --test security
```

## Fuzzing

The crate can be fuzzed using `cargo-fuzz`. Fuzzing targets are recommended for:

1. **Decoding arbitrary strings** - Ensures no panics on any input
2. **Encoding arbitrary bytes** - Ensures no panics on any binary data
3. **Round-trip operations** - Verifies encode/decode consistency

### Setting Up Fuzzing

```bash
# Install cargo-fuzz
cargo install cargo-fuzz

# Initialize fuzzing (if not already done)
cargo fuzz init

# Run fuzz tests
cargo fuzz run fuzz_decode    # Fuzz decoding operations
cargo fuzz run fuzz_encode    # Fuzz encoding operations
cargo fuzz run fuzz_roundtrip # Fuzz full round-trips
```

## Dependency Security

The crate depends on well-maintained libraries:

- `base-x` (0.2.7) - Variable-radix base encoding
- `base256emoji` (1.0.2) - Base256Emoji encoding
- `data-encoding` (2.3.1) - Standard base encodings
- `thiserror` (2.0) - Error handling

All dependencies are actively maintained and widely used in the Rust ecosystem.

## Reporting Security Issues

If you discover a security vulnerability in the multibase crate, please report it privately:

1. **Do not** open a public GitHub issue
2. Contact the maintainers via email (check Cargo.toml for contact information)
3. Provide detailed information about the vulnerability
4. Allow reasonable time for a fix before public disclosure

## Security Guarantees

### What This Crate Guarantees

- ✅ No panics on arbitrary untrusted input
- ✅ Memory safety (no unsafe code used)
- ✅ Comprehensive input validation
- ✅ Thread-safe operations (all types are Send + Sync where appropriate)
- ✅ Error information without exposing internal state

### What This Crate Does NOT Guarantee

- ❌ Protection against resource exhaustion (application responsibility)
- ❌ Constant-time operations (not designed for cryptographic use)
- ❌ Perfect round-tripping of invalid UTF-8 in Identity encoding

## Changelog of Security-Related Changes

### Version 2.0.0
- Fixed Identity encoding panic risk (now uses lossy UTF-8 conversion)
- Migrated to thiserror for better error handling
- Added 17 security-focused tests
- Conducted comprehensive security audit
- Added this SECURITY.md document

### Version 1.0.0
- Initial release with basic security considerations

## Security Acknowledgments

Security audits and improvements were guided by:
- [The Definitive Guide to Rust Error Handling](https://www.howtocodeit.com/articles/the-definitive-guide-to-rust-error-handling)
- Rust security best practices
- OWASP guidelines for input validation
- Industry-standard secure coding practices

## Last Updated

2025-10-08
