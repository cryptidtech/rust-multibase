// SPDX-License-Identifier: MIT

//! # multibase
//!
//! Implementation of [multibase](https://github.com/multiformats/multibase) in Rust.

#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

mod base;
pub mod encoded;
mod encoding;
mod error;
mod impls;

pub use self::base::Base;
pub use self::encoded::EncodedString;
pub use self::error::{Error, Result};

/// Decode the base string.
///
/// # Examples
///
/// ```
/// use multibase::{Base, decode};
///
/// assert_eq!(
///     decode("zCn8eVZg", true).unwrap(),
///     (Base::Base58Btc, b"hello".to_vec())
/// );
/// ```
pub fn decode<T: AsRef<str>>(input: T, strict: bool) -> Result<(Base, Vec<u8>)> {
    let input = input.as_ref();
    let code = input.chars().next().ok_or(Error::EmptyInput)?;
    let base = Base::from_code(code)?;
    let decoded = base.decode(&input[code.len_utf8()..], strict)?;
    Ok((base, decoded))
}

/// Encode with the given byte slice to base string.
///
/// # Examples
///
/// ```
/// use multibase::{Base, encode};
///
/// assert_eq!(encode(Base::Base58Btc, b"hello"), "zCn8eVZg");
/// ```
///
/// # Performance
///
/// This function pre-allocates the exact capacity needed and constructs the
/// result string efficiently by prepending the base code without requiring
/// reallocation or memory moves.
pub fn encode<T: AsRef<[u8]>>(base: Base, input: T) -> String {
    let input = input.as_ref();
    let encoded = base.encode(input);
    let code = base.code();

    // Pre-allocate exact size needed: code length + encoded length
    // This is much faster than insert(0) which requires moving all bytes
    let mut result = String::with_capacity(code.len_utf8() + encoded.len());
    result.push(code);
    result.push_str(&encoded);
    result
}

/// Encode with the given byte slice to base string, writing into an existing buffer.
///
/// This is a zero-copy variant of [`encode`] that reuses an existing String buffer,
/// avoiding allocations when encoding multiple values in a loop.
///
/// The buffer will be cleared before encoding, then filled with the base code prefix
/// followed by the encoded data.
///
/// # Examples
///
/// ```
/// use multibase::{Base, encode_into};
///
/// let mut buffer = String::new();
///
/// // Encode multiple values reusing the same buffer
/// encode_into(Base::Base58Btc, b"hello", &mut buffer);
/// assert_eq!(buffer, "zCn8eVZg");
///
/// encode_into(Base::Base64, b"world", &mut buffer);
/// assert_eq!(buffer, "md29ybGQ");
/// ```
///
/// # Performance
///
/// When encoding many values, this function can be significantly faster than
/// [`encode`] as it reuses the allocated buffer instead of allocating a new
/// String for each encoding operation.
pub fn encode_into<T: AsRef<[u8]>>(base: Base, input: T, buffer: &mut String) {
    let input = input.as_ref();
    let encoded = base.encode(input);
    let code = base.code();

    // Clear and reserve exact capacity needed
    buffer.clear();
    buffer.reserve(code.len_utf8() + encoded.len());
    buffer.push(code);
    buffer.push_str(&encoded);
}

/// Decode the base string, writing the result into an existing buffer.
///
/// This is a zero-copy variant of [`decode`] that reuses an existing `Vec<u8>` buffer,
/// avoiding allocations when decoding multiple values in a loop.
///
/// The buffer will be cleared before decoding, then filled with the decoded bytes.
/// Returns the base encoding that was detected from the prefix.
///
/// # Examples
///
/// ```
/// use multibase::{Base, decode_into};
///
/// let mut buffer = Vec::new();
///
/// // Decode multiple values reusing the same buffer
/// let base = decode_into("zCn8eVZg", true, &mut buffer).unwrap();
/// assert_eq!(base, Base::Base58Btc);
/// assert_eq!(buffer, b"hello");
///
/// let base = decode_into("md29ybGQ", true, &mut buffer).unwrap();
/// assert_eq!(base, Base::Base64);
/// assert_eq!(buffer, b"world");
/// ```
///
/// # Performance
///
/// When decoding many values, this function can be significantly faster than
/// [`decode`] as it reuses the allocated buffer instead of allocating a new
/// `Vec<u8>` for each decoding operation.
///
/// # Errors
///
/// Returns an error if:
/// - The input string is empty
/// - The base code prefix is unknown
/// - The encoded data is invalid for the specified base
pub fn decode_into<T: AsRef<str>>(input: T, strict: bool, buffer: &mut Vec<u8>) -> Result<Base> {
    let input = input.as_ref();
    let code = input.chars().next().ok_or(Error::EmptyInput)?;
    let base = Base::from_code(code)?;
    let decoded = base.decode(&input[code.len_utf8()..], strict)?;

    // Clear and fill buffer with decoded data
    buffer.clear();
    buffer.extend_from_slice(&decoded);
    Ok(base)
}

/// Encode with the given byte slice and return a validated `EncodedString`.
///
/// This is a convenience function that combines [`encode`] with [`EncodedString::new`],
/// providing type-level guarantees that the returned string is a valid multibase encoding.
///
/// # Examples
///
/// ```
/// use multibase::{Base, encode_to_validated};
///
/// let encoded = encode_to_validated(Base::Base58Btc, b"hello");
/// assert_eq!(encoded.base(), Base::Base58Btc);
/// assert_eq!(encoded.as_str(), "zCn8eVZg");
///
/// // Decode directly from the validated string
/// let decoded = encoded.decode().unwrap();
/// assert_eq!(decoded, b"hello");
/// ```
///
/// # Performance
///
/// This function has the same performance characteristics as [`encode`].
/// The validation overhead is negligible (just checking the base code).
pub fn encode_to_validated<T: AsRef<[u8]>>(base: Base, input: T) -> EncodedString {
    let encoded_str = encode(base, input);
    // SAFETY: We just encoded this with a valid base, so it must be valid
    // This unwrap is safe and will never panic
    EncodedString::new(encoded_str).expect("freshly encoded string must be valid")
}

/// Parse a multibase string into a validated `EncodedString`.
///
/// This is equivalent to [`EncodedString::new`] but provides a standalone
/// function for consistency with other API functions.
///
/// # Examples
///
/// ```
/// use multibase::{Base, parse_encoded};
///
/// let encoded = parse_encoded("zCn8eVZg").unwrap();
/// assert_eq!(encoded.base(), Base::Base58Btc);
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - The input string is empty
/// - The base code prefix is unknown
pub fn parse_encoded<T: AsRef<str>>(input: T) -> Result<EncodedString> {
    EncodedString::new(input.as_ref())
}
