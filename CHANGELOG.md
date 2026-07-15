# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.2] - 2026-07-15

### Added

- **`#![deny(unsafe_code)]`** at the crate root. The library contains no
  `unsafe` code; this lint enforces that invariant at compile time.
- **`#[inline]`** on hot encode/decode paths: `encode`, `decode`,
  `encode_into`, `decode_into`, `encode_to_validated`, `parse_encoded`,
  `Base::from_code`, `Base::code`, `Base::encode`, `Base::decode`,
  `Base::decode_into`, `EncodedString::base`, `EncodedString::as_str`,
  `EncodedString::decode`, `EncodedString::decode_with_strictness`, and the
  macro-generated `BaseCodec` implementations.
- **`const fn`**: `Base::from_code`, `Base::code`, and `EncodedString::base`
  are now `const fn`.
- **`#[must_use]`** on `EncodedString::base`, `as_str`, and `into_inner`.
- **`# Errors`** doc sections on `decode()` and `Base::from_code`.
- **`# Panics`** doc section on `encode_to_validated()`.
- **MSRV declared**: `rust-version = "1.85"` in `Cargo.toml`. CI verifies the
  MSRV with a dedicated job.
- **`cargo audit`** job in CI.
- **Clippy lint configuration**: `[lints.clippy]` with `pedantic`, `nursery`,
  and `cargo` groups (all `warn`), plus `[lints.rust] unsafe_code = "deny"`.

### Changed

- **Edition 2024**: Updated from Rust 2021. Required escaping the now-reserved
  `gen` keyword (`rng.r#gen()`) in benchmarks.
- **CLI fixed**: `cli/Cargo.toml` and `cli/main.rs` updated from the legacy
  `multibase` dependency name to `multi-base`. The CLI now builds.
- **Fuzz targets fixed**: `fuzz/Cargo.toml` and fuzz targets updated from the
  legacy `multibase` name to `multi-base`, with edition 2024 and lint config.
- **`Error::eq`**: Merged identical match arms for unit variants into a single
  pattern.
- **Doc comments**: Backticked `rfc4648` in base-variant doc comments.

## [1.0.1] - 2026-07-13

### Changed

- Synced from bettersign workspace (bs-multibase 0.7.0).
- Renamed crate from `bs-multibase` to `multi-base`.
- Reimplemented `Base256Emoji` inline (dropped external `base256emoji` dep).
- Initial published release on crates.io as `multi-base`.

### Added

- **Zero-copy APIs**: `encode_into()` and `decode_into()` for buffer reuse.
- **`encode()` optimization**: Pre-allocates exact capacity instead of
  `insert(0, char)`.
- **`EncodedString` newtype**: Validated multibase-encoded string type
  implementing `FromStr`, `TryFrom<String>`, `TryFrom<&str>`, `AsRef<str>`,
  `Display`. Provides `decode()` and `decode_with_strictness()` methods.
- **Convenience functions**: `encode_to_validated()` and `parse_encoded()`.
- **Property-based tests**: 16 proptest cases for round-trip, determinism, and
  buffer-reuse invariants.
- **Error handling tests**: Coverage for all base types.
- **Concurrency tests**: Parallel operation verification.
- **Benchmarks**: Enhanced benchmarking for all base types and sizes.
- **Macro documentation**: Comprehensive docs for `build_base_enum`,
  `derive_base_encoding`, and `derive_base_x` macros.
- **Macro hygiene**: `$crate::` prefixes for proper hygiene.
- **Clap v4 migration**: CLI migrated from structopt 0.3 to clap v4.
- **CLI deduplication**: Unified Base↔string mappings with a macro.
- **CLI error messages**: Context added to all error paths; unknown bases list
  all available options.
- **Security tests**: 17 tests covering large inputs, malformed input,
  buffer-reuse safety, concurrent operations, and resource exhaustion.
- **Fuzzing infrastructure**: `cargo-fuzz` with `fuzz_decode`, `fuzz_encode`,
  and `fuzz_roundtrip` targets.
- **SECURITY.md**: Security review findings, best practices, input size limit
  recommendations, and vulnerability reporting process.
- **CONCURRENCY.md**: Thread safety guarantees, safe concurrent usage patterns,
  and performance considerations.
- **Thread safety tests**: 20 tests with compile-time Send/Sync assertions and
  a 2000-operation stress test.

### Changed (1.0.1)

- **BREAKING**: Error type uses `thiserror` instead of manual implementation.
  `#[non_exhaustive]` added for forward compatibility.
- **BREAKING**: Identity encoding uses lossy UTF-8 conversion instead of
  panicking. Invalid UTF-8 bytes are replaced with U+FFFD and will not
  round-trip perfectly.
- Updated from Rust 2018 to Rust 2021.
- Added SPDX-License-Identifier headers to all source files.
- Enhanced all public item documentation with performance characteristics,
  usage examples, and error conditions.

### Fixed (1.0.1)

- Panic in Identity encoding on invalid UTF-8 (now uses lossy conversion).
- O(n) string reallocation in `encode()`.
- Error context lost during conversions.
- Rustdoc warnings and broken links.

## [1.0.0] - Previous Release

Initial stable release with basic multibase functionality.

### Features
- Support for 24 base encodings.
- Strict and permissive decoding modes.
- `no_std` support with `alloc`.
- Basic error handling.
- CLI tool.

## Compatibility

### Minimum Supported Rust Version (MSRV)

Rust 1.85 (Edition 2024).

### Platform Support

- Linux
- macOS
- Windows
- WebAssembly (wasm32)
- `no_std` environments (with `alloc`)

### Breaking Changes in 1.0.1

1. Error type structure changed (uses `thiserror`).
2. Identity encoding no longer panics (uses lossy conversion).
3. `#[non_exhaustive]` added to `Error` enum.
4. Edition updated to 2021.

[1.0.2]: https://github.com/cryptidtech/multi-base/compare/v1.0.1...v1.0.2
[1.0.1]: https://github.com/cryptidtech/multi-base/releases/tag/v1.0.1
[1.0.0]: https://github.com/multiformats/rust-multibase/releases/tag/v1.0.0