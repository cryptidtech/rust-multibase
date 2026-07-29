# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.4] - 2026-07-29

### Changed
- Rewrote `README.md`, `SECURITY.md`, and `CHANGELOG.md` in ASD-STE100
  strict mode. Removed marketing language, passive voice, and long
  sentences.
- Fixed `README.md` inaccuracies. The stale note that said the published
  version is `1.0.1` is removed. The install version now reads `1.1`.
  The test count now reads 142.
- Fixed `SECURITY.md` inaccuracies. The security changelog now lists
  1.0.2 and 1.0.3 changes. The stale "Last Updated" date is corrected.
  The reference to "the multibase crate" in the reporting section now
  says "the `multi-base` crate".

## [1.0.3] - 2026-07-16

### Security
- Removed the unmaintained `serde_cbor` dev-dependency. This addresses
  RUSTSEC-2021-0127.

### Changed
- Updated `SECURITY.md` to document the inline `Base256Emoji`
  implementation. The stale reference to the dropped `base256emoji`
  dependency is corrected.
- Updated `SECURITY.md` code examples to use `multi_base::` instead of
  the old `multibase::` module path.

## [1.0.2] - 2026-07-15

### Added

- `#![deny(unsafe_code)]` at the crate root. The library has no `unsafe`
  code. This lint enforces that at compile time.
- `#[inline]` on hot encode and decode paths. This covers `encode`,
  `decode`, `encode_into`, `decode_into`, `encode_to_validated`,
  `parse_encoded`, `Base::from_code`, `Base::code`, `Base::encode`,
  `Base::decode`, `Base::decode_into`, `EncodedString::base`,
  `EncodedString::as_str`, `EncodedString::decode`,
  `EncodedString::decode_with_strictness`, and the macro-generated
  `BaseCodec` implementations.
- `const fn` on `Base::from_code`, `Base::code`, and
  `EncodedString::base`.
- `#[must_use]` on `EncodedString::base`, `as_str`, and `into_inner`.
- `# Errors` doc sections on `decode()` and `Base::from_code`.
- `# Panics` doc section on `encode_to_validated()`.
- MSRV declared as `rust-version = "1.85"` in `Cargo.toml`. CI verifies
  the MSRV with a dedicated job.
- `cargo audit` job in CI.
- Clippy lint configuration. `[lints.clippy]` sets `pedantic`,
  `nursery`, and `cargo` groups to `warn`. `[lints.rust]` sets
  `unsafe_code` to `deny`.

### Changed

- Edition updated to 2024 from 2021. The `gen` keyword is now reserved.
  Benchmarks use `rng.r#gen()`.
- CLI fixed. `cli/Cargo.toml` and `cli/main.rs` updated from the
  legacy `multibase` dependency name to `multi-base`.
- Fuzz targets fixed. `fuzz/Cargo.toml` and fuzz targets updated from
  the legacy `multibase` name to `multi-base`. Edition 2024 and lint
  config added.
- `Error::eq` merged identical match arms into a single pattern.
- Backticked `rfc4648` in base-variant doc comments.

## [1.0.1] - 2026-07-13

### Changed

- Synced from the bettersign workspace (`bs-multibase` 0.7.0).
- Renamed the crate from `bs-multibase` to `multi-base`.
- Reimplemented `Base256Emoji` inline. Dropped the external
  `base256emoji` dependency.
- Initial published release on crates.io as `multi-base`.

### Added

- Zero-copy APIs: `encode_into()` and `decode_into()` for buffer reuse.
- `encode()` optimization. Pre-allocates exact capacity instead of
  `insert(0, char)`.
- `EncodedString` newtype. A validated multibase-encoded string type.
  Implements `FromStr`, `TryFrom<String>`, `TryFrom<&str>`, `AsRef<str>`,
  and `Display`. Gives `decode()` and `decode_with_strictness()`
  methods.
- Convenience functions: `encode_to_validated()` and `parse_encoded()`.
- 16 property-based tests for round-trip, determinism, and buffer-reuse
  invariants.
- Error handling tests for all base types.
- Concurrency tests for parallel operation verification.
- Enhanced benchmarks for all base types and sizes.
- Macro documentation for `build_base_enum`, `derive_base_encoding`,
  and `derive_base_x` macros.
- Macro hygiene with `$crate::` prefixes.
- CLI migrated from structopt 0.3 to clap v4.
- CLI deduplication. Unified Base and string mappings with a macro.
- CLI error messages. Context added to all error paths. Unknown bases
  list all available options.
- 17 security tests: large inputs, malformed input, buffer-reuse
  safety, concurrent operations, and resource exhaustion.
- Fuzzing infrastructure with `cargo-fuzz`. Targets: `fuzz_decode`,
  `fuzz_encode`, and `fuzz_roundtrip`.
- `SECURITY.md` with security review results, best practices, and
  vulnerability reporting process.
- `CONCURRENCY.md` with thread safety guarantees and concurrent usage
  patterns.
- 20 thread safety tests with compile-time `Send` and `Sync` assertions
  and a 2000-operation stress test.

### Changed (1.0.1)

- BREAKING: Error type uses `thiserror` instead of a manual
  implementation. `#[non_exhaustive]` added for forward compatibility.
- BREAKING: Identity encoding uses lossy UTF-8 conversion instead of
  panicking. Invalid UTF-8 bytes are replaced with U+FFFD. They do not
  round-trip.
- Updated from Rust 2018 to Rust 2021.
- Added SPDX-License-Identifier headers to all source files.
- Enhanced public item documentation with performance characteristics,
  usage examples, and error conditions.

### Fixed (1.0.1)

- Panic in Identity encoding on invalid UTF-8. Now uses lossy
  conversion.
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

1. Error type structure changed. Uses `thiserror`.
2. Identity encoding no longer panics. Uses lossy conversion.
3. `#[non_exhaustive]` added to `Error` enum.
4. Edition updated to 2021.

[1.0.4]: https://github.com/cryptidtech/multi-base/compare/v1.0.3...v1.0.4
[1.0.3]: https://github.com/cryptidtech/multi-base/compare/v1.0.2...v1.0.3
[1.0.2]: https://github.com/cryptidtech/multi-base/compare/v1.0.1...v1.0.2
[1.0.1]: https://github.com/cryptidtech/multi-base/releases/tag/v1.0.1
[1.0.0]: https://github.com/multiformats/rust-multibase/releases/tag/v1.0.0