// SPDX-License-Identifier: MIT

//! Validated multibase-encoded strings.
//!
//! This module provides the [`EncodedString`] type, a newtype wrapper that guarantees
//! a string is a valid multibase-encoded value with a recognized base code prefix.
//!
//! # Type Safety
//!
//! Following the "parse, don't validate" principle, [`EncodedString`] ensures that
//! once created, the string is guaranteed to:
//! - Have a valid base code prefix
//! - Be parseable (though decoding may still fail if the data is malformed)
//!
//! # Examples
//!
//! ```
//! use multibase::{EncodedString, Base};
//!
//! // Parse and validate a multibase string
//! let encoded = EncodedString::new("zCn8eVZg").unwrap();
//! assert_eq!(encoded.base(), Base::Base58Btc);
//!
//! // Decode the validated string
//! let decoded = encoded.decode().unwrap();
//! assert_eq!(decoded, b"hello");
//!
//! // Access the underlying string
//! assert_eq!(encoded.as_str(), "zCn8eVZg");
//! ```

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

use crate::{Base, Error, Result};

/// A validated multibase-encoded string.
///
/// This type provides type-level guarantees that a string contains a valid
/// base code prefix and can be decoded (though decoding may still fail if
/// the data is malformed).
///
/// # Construction
///
/// Use [`EncodedString::new`] or parse from a string using [`core::str::FromStr`]:
///
/// ```
/// use multibase::EncodedString;
/// use std::str::FromStr;
///
/// // Using new()
/// let encoded = EncodedString::new("zCn8eVZg").unwrap();
///
/// // Using FromStr
/// let encoded: EncodedString = "md29ybGQ".parse().unwrap();
/// ```
///
/// # Errors
///
/// Construction fails if:
/// - The input string is empty
/// - The first character is not a valid base code
///
/// Note that construction only validates the base code prefix, not the
/// encoded data itself. Use [`decode()`](Self::decode) to fully validate
/// and decode the data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EncodedString {
    /// The validated base encoding.
    base: Base,
    /// The complete encoded string (including base code prefix).
    inner: String,
}

impl EncodedString {
    /// Create a new validated multibase-encoded string.
    ///
    /// This function validates that the input string has a valid base code
    /// prefix. It does not decode the full data, so malformed encoded data
    /// will only be detected when calling [`decode()`](Self::decode).
    ///
    /// # Examples
    ///
    /// ```
    /// use multibase::{EncodedString, Base};
    ///
    /// // Valid multibase string
    /// let encoded = EncodedString::new("zCn8eVZg").unwrap();
    /// assert_eq!(encoded.base(), Base::Base58Btc);
    ///
    /// // Empty string fails
    /// assert!(EncodedString::new("").is_err());
    ///
    /// // Invalid base code fails
    /// assert!(EncodedString::new("xInvalid").is_err());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The input string is empty ([`Error::EmptyInput`])
    /// - The first character is not a valid base code ([`Error::UnknownBase`])
    pub fn new<S: Into<String>>(s: S) -> Result<Self> {
        let inner = s.into();

        // Validate base code prefix
        let code = inner.chars().next().ok_or(Error::EmptyInput)?;
        let base = Base::from_code(code)?;

        Ok(Self { base, inner })
    }

    /// Get the base encoding of this string.
    ///
    /// # Examples
    ///
    /// ```
    /// use multibase::{EncodedString, Base};
    ///
    /// let encoded = EncodedString::new("zCn8eVZg").unwrap();
    /// assert_eq!(encoded.base(), Base::Base58Btc);
    ///
    /// let encoded = EncodedString::new("md29ybGQ").unwrap();
    /// assert_eq!(encoded.base(), Base::Base64);
    /// ```
    pub fn base(&self) -> Base {
        self.base
    }

    /// Get the encoded string as a string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use multibase::EncodedString;
    ///
    /// let encoded = EncodedString::new("zCn8eVZg").unwrap();
    /// assert_eq!(encoded.as_str(), "zCn8eVZg");
    /// ```
    pub fn as_str(&self) -> &str {
        &self.inner
    }

    /// Decode this validated multibase string.
    ///
    /// This performs the actual decoding of the data. Unlike construction,
    /// which only validates the base code prefix, this method validates and
    /// decodes the entire encoded data.
    ///
    /// # Examples
    ///
    /// ```
    /// use multibase::EncodedString;
    ///
    /// let encoded = EncodedString::new("zCn8eVZg").unwrap();
    /// let decoded = encoded.decode().unwrap();
    /// assert_eq!(decoded, b"hello");
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the encoded data is malformed for the detected base.
    pub fn decode(&self) -> Result<Vec<u8>> {
        let code_len = self.base.code().len_utf8();
        self.base.decode(&self.inner[code_len..], true)
    }

    /// Decode this validated multibase string with strictness control.
    ///
    /// When `strict` is `false`, some bases allow case-insensitive decoding
    /// or other permissive behaviors.
    ///
    /// # Examples
    ///
    /// ```
    /// use multibase::EncodedString;
    ///
    /// // Case-insensitive decoding for some bases
    /// let encoded = EncodedString::new("FaB").unwrap(); // Base16Upper with mixed case
    /// let decoded = encoded.decode_with_strictness(false).unwrap();
    /// assert_eq!(decoded, b"\xab");
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the encoded data is malformed for the detected base.
    pub fn decode_with_strictness(&self, strict: bool) -> Result<Vec<u8>> {
        let code_len = self.base.code().len_utf8();
        self.base.decode(&self.inner[code_len..], strict)
    }

    /// Consume the `EncodedString` and return the inner string.
    ///
    /// # Examples
    ///
    /// ```
    /// use multibase::EncodedString;
    ///
    /// let encoded = EncodedString::new("zCn8eVZg").unwrap();
    /// let inner = encoded.into_inner();
    /// assert_eq!(inner, "zCn8eVZg");
    /// ```
    pub fn into_inner(self) -> String {
        self.inner
    }
}

// Implement AsRef<str> for ergonomic usage
impl AsRef<str> for EncodedString {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

// Implement Display for easy printing
impl core::fmt::Display for EncodedString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

// Implement FromStr for parsing
impl core::str::FromStr for EncodedString {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}

// Implement TryFrom<String> for convenient conversion
impl core::convert::TryFrom<String> for EncodedString {
    type Error = Error;

    fn try_from(s: String) -> Result<Self> {
        Self::new(s)
    }
}

// Implement TryFrom<&str> for convenient conversion
impl<'a> core::convert::TryFrom<&'a str> for EncodedString {
    type Error = Error;

    fn try_from(s: &'a str) -> Result<Self> {
        Self::new(s)
    }
}

// Implement From<EncodedString> for String for easy unwrapping
impl From<EncodedString> for String {
    fn from(encoded: EncodedString) -> Self {
        encoded.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid() {
        let encoded = EncodedString::new("zCn8eVZg").unwrap();
        assert_eq!(encoded.base(), Base::Base58Btc);
        assert_eq!(encoded.as_str(), "zCn8eVZg");
    }

    #[test]
    fn test_new_empty_fails() {
        let result = EncodedString::new("");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::EmptyInput));
    }

    #[test]
    fn test_new_invalid_base_fails() {
        let result = EncodedString::new("xInvalid");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::UnknownBase { .. }));
    }

    #[test]
    fn test_decode() {
        let encoded = EncodedString::new("zCn8eVZg").unwrap();
        let decoded = encoded.decode().unwrap();
        assert_eq!(decoded, b"hello");
    }

    #[test]
    fn test_from_str() {
        use core::str::FromStr;
        let encoded = EncodedString::from_str("md29ybGQ").unwrap();
        assert_eq!(encoded.base(), Base::Base64);
    }

    #[test]
    fn test_try_from_string() {
        use core::convert::TryFrom;
        let encoded = EncodedString::try_from("f48656c6c6f".to_string()).unwrap();
        assert_eq!(encoded.base(), Base::Base16Lower);
    }

    #[test]
    fn test_try_from_str() {
        use core::convert::TryFrom;
        let encoded = EncodedString::try_from("BPFSXGIDNMFXGSIBB").unwrap();
        assert_eq!(encoded.base(), Base::Base32Upper);
    }

    #[test]
    fn test_into_inner() {
        let encoded = EncodedString::new("zCn8eVZg").unwrap();
        let inner = encoded.into_inner();
        assert_eq!(inner, "zCn8eVZg");
    }

    #[test]
    fn test_display() {
        let encoded = EncodedString::new("zCn8eVZg").unwrap();
        assert_eq!(format!("{}", encoded), "zCn8eVZg");
    }

    #[test]
    fn test_clone() {
        let encoded = EncodedString::new("zCn8eVZg").unwrap();
        let cloned = encoded.clone();
        assert_eq!(encoded, cloned);
    }

    #[test]
    fn test_eq() {
        let encoded1 = EncodedString::new("zCn8eVZg").unwrap();
        let encoded2 = EncodedString::new("zCn8eVZg").unwrap();
        let encoded3 = EncodedString::new("md29ybGQ").unwrap();

        assert_eq!(encoded1, encoded2);
        assert_ne!(encoded1, encoded3);
    }

    #[test]
    fn test_as_ref() {
        let encoded = EncodedString::new("zCn8eVZg").unwrap();
        let s: &str = encoded.as_ref();
        assert_eq!(s, "zCn8eVZg");
    }
}
