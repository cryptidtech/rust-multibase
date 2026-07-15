// SPDX-License-Identifier: MIT

#[cfg(not(feature = "std"))]
use alloc::format;

/// Type alias to use this library's [`Error`] type in a `Result`.
pub type Result<T> = core::result::Result<T, Error>;

/// Error types for multibase operations.
///
/// This enum uses `thiserror` for ergonomic error handling.
///
/// # Non-exhaustive
///
/// This enum is marked `#[non_exhaustive]` to allow adding new error variants
/// in the future without breaking changes.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Unknown base code encountered.
    ///
    /// The provided character does not correspond to any known base encoding
    /// in the multibase specification.
    #[error("unknown base code: {code}")]
    UnknownBase {
        /// The invalid base code character.
        code: char,
    },

    /// Invalid base string format.
    ///
    /// The string could not be decoded using the specified base encoding.
    /// This is a generic decoding error when the specific cause is unknown.
    #[error("invalid base string")]
    InvalidBaseString,

    /// Base-x decoding failed.
    ///
    /// Used for Base10, Base36, and Base58 variants that use the `base-x` library.
    #[error("base-x decoding failed")]
    BaseXDecode,

    /// `Base256Emoji` decoding failed.
    ///
    /// The input string contained invalid emoji sequences or could not be
    /// decoded as `Base256Emoji`.
    #[error("base256emoji decoding failed")]
    Base256EmojiDecode,

    /// Data-encoding decoding failed.
    ///
    /// Used for Base2, Base8, Base16, Base32, Base64 variants that use
    /// the `data-encoding` library.
    #[error("data-encoding decoding failed: {message}")]
    DataEncodingDecode {
        /// The error message from data-encoding.
        #[cfg(feature = "std")]
        message: std::string::String,
        /// The error message from data-encoding (no_std).
        #[cfg(not(feature = "std"))]
        message: alloc::string::String,
    },

    /// Empty input string.
    ///
    /// The decode function was called with an empty string, which cannot
    /// contain a valid base code prefix.
    #[error("empty input string")]
    EmptyInput,
}

// Implement PartialEq
impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::UnknownBase { code: c1 }, Self::UnknownBase { code: c2 }) => c1 == c2,
            (
                Self::DataEncodingDecode { message: m1 },
                Self::DataEncodingDecode { message: m2 },
            ) => m1 == m2,
            (Self::InvalidBaseString, Self::InvalidBaseString)
            | (Self::EmptyInput, Self::EmptyInput)
            | (Self::BaseXDecode, Self::BaseXDecode)
            | (Self::Base256EmojiDecode, Self::Base256EmojiDecode) => true,
            _ => false,
        }
    }
}

// Implement Eq
impl Eq for Error {}

// Implement Clone
impl Clone for Error {
    fn clone(&self) -> Self {
        match self {
            Self::UnknownBase { code } => Self::UnknownBase { code: *code },
            Self::InvalidBaseString => Self::InvalidBaseString,
            Self::BaseXDecode => Self::BaseXDecode,
            Self::Base256EmojiDecode => Self::Base256EmojiDecode,
            Self::DataEncodingDecode { message } => Self::DataEncodingDecode {
                message: message.clone(),
            },
            Self::EmptyInput => Self::EmptyInput,
        }
    }
}

// Manual From implementations for error conversions
impl From<base_x::DecodeError> for Error {
    fn from(_: base_x::DecodeError) -> Self {
        Self::BaseXDecode
    }
}

impl From<data_encoding::DecodeError> for Error {
    fn from(err: data_encoding::DecodeError) -> Self {
        #[cfg(feature = "std")]
        let message = std::format!("{err}");
        #[cfg(not(feature = "std"))]
        let message = format!("{}", err);

        Self::DataEncodingDecode { message }
    }
}
