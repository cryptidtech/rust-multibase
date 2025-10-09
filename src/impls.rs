// SPDX-License-Identifier: MIT

use crate::encoding;
use crate::error::Result;
use base256emoji::{Base, Emoji};

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

/// Generates BaseCodec implementations for data-encoding-based encodings.
///
/// This macro creates type wrappers and `BaseCodec` trait implementations for
/// base encodings that use the `data-encoding` crate. It handles both strict
/// and permissive decoding modes.
///
/// # Macro Hygiene
///
/// Uses `$crate::` prefixes to ensure proper hygiene when invoked from
/// different modules or crates.
///
/// # Parameters
///
/// The macro accepts a comma-separated list of encoding definitions:
/// ```text
/// #[doc = "Documentation"] TypeName, ENCODING_CONSTANT, PERMISSIVE_ENCODING;
/// ```
///
/// Where:
/// - `#[doc = "..."]` - Documentation for the generated type
/// - `TypeName` - PascalCase name for the struct (e.g., `Base64`, `Base32Lower`)
/// - `ENCODING_CONSTANT` - Strict encoding spec from `data-encoding` crate
/// - `PERMISSIVE_ENCODING` - Permissive encoding spec (case-insensitive, etc.)
///
/// # Generated Code
///
/// For each encoding definition, generates:
/// 1. A zero-sized struct type
/// 2. `BaseCodec` implementation with `encode()` and `decode()` methods
/// 3. Strict/permissive decoding logic based on the `strict` parameter
///
/// # Example
///
/// ```ignore
/// derive_base_encoding! {
///     /// Base64 encoding
///     Base64, encoding::BASE64_NOPAD, encoding::BASE64_NOPAD_PERMISSIVE;
/// }
/// ```
///
/// # Strict vs Permissive Mode
///
/// - **Strict mode**: Uses the exact encoding specification (case-sensitive)
/// - **Permissive mode**: Accepts variations like mixed case where applicable
///
/// # Error Handling
///
/// Decoding errors from `data-encoding` are automatically converted to the
/// crate's `Error` type via the `?` operator.
macro_rules! derive_base_encoding {
    ( $(#[$doc:meta] $type:ident, $encoding:expr, $permissive:expr;)* ) => {
        $(
            #[$doc]
            #[derive(PartialEq, Eq, Clone, Copy, Debug)]
            pub(crate) struct $type;

            impl BaseCodec for $type {
                fn encode<I: AsRef<[u8]>>(input: I) -> String {
                    $encoding.encode(input.as_ref())
                }

                fn decode<I: AsRef<str>>(input: I, strict: bool) -> $crate::error::Result<Vec<u8>> {
                    if strict {
                        Ok($encoding.decode(input.as_ref().as_bytes())?)
                    } else {
                        Ok($permissive.decode(input.as_ref().as_bytes())?)
                    }
                }
            }
        )*
    };
}

/// Generates BaseCodec implementations for base-x-based encodings.
///
/// This macro creates type wrappers and `BaseCodec` trait implementations for
/// base encodings that use the `base-x` crate (variable-radix encodings like
/// Base10, Base58, etc.). It handles both strict and permissive decoding modes.
///
/// # Macro Hygiene
///
/// Uses `$crate::` prefixes to ensure proper hygiene when invoked from
/// different modules or crates.
///
/// # Parameters
///
/// The macro accepts a comma-separated list of encoding definitions:
/// ```text
/// #[doc = "Documentation"] TypeName, ALPHABET, PERMISSIVE_ALPHABET;
/// ```
///
/// Where:
/// - `#[doc = "..."]` - Documentation for the generated type
/// - `TypeName` - PascalCase name for the struct (e.g., `Base58Btc`, `Base10`)
/// - `ALPHABET` - Strict alphabet string for this encoding
/// - `PERMISSIVE_ALPHABET` - Permissive alphabet (may be same as strict)
///
/// # Generated Code
///
/// For each encoding definition, generates:
/// 1. A zero-sized struct type
/// 2. `BaseCodec` implementation with `encode()` and `decode()` methods
/// 3. Strict/permissive decoding logic based on the `strict` parameter
///
/// # Example
///
/// ```ignore
/// derive_base_x! {
///     /// Base58 Bitcoin encoding
///     Base58Btc, encoding::BASE58_BITCOIN, encoding::BASE58_BITCOIN_PERMISSIVE;
/// }
/// ```
///
/// # Difference from derive_base_encoding
///
/// This macro uses `base_x::encode/decode` instead of `data-encoding`, which
/// is appropriate for variable-radix encodings where the alphabet defines the base.
///
/// **Why Two Separate Macros?**
///
/// While `derive_base_encoding` and `derive_base_x` appear similar, they use
/// different APIs:
/// - `derive_base_encoding`: Calls methods on `data-encoding` objects
/// - `derive_base_x`: Calls functions from the `base-x` crate
///
/// Keeping them separate maintains clarity and avoids complex conditional logic.
///
/// # Strict vs Permissive Mode
///
/// - **Strict mode**: Uses the exact alphabet
/// - **Permissive mode**: May accept variations (though often identical to strict)
///
/// # Error Handling
///
/// Decoding errors from `base-x` are automatically converted to the
/// crate's `Error` type via the `?` operator.
macro_rules! derive_base_x {
    ( $(#[$doc:meta] $type:ident, $encoding:expr, $permissive:expr;)* ) => {
        $(
            #[$doc]
            #[derive(PartialEq, Eq, Clone, Copy, Debug)]
            pub(crate) struct $type;

            impl BaseCodec for $type {
                fn encode<I: AsRef<[u8]>>(input: I) -> String {
                    base_x::encode($encoding, input.as_ref())
                }

                fn decode<I: AsRef<str>>(input: I, strict: bool) -> $crate::error::Result<Vec<u8>> {
                    if strict {
                        Ok(base_x::decode($encoding, input.as_ref())?)
                    } else {
                        Ok(base_x::decode($permissive, input.as_ref())?)
                    }
                }
            }
        )*
    };
}

pub(crate) trait BaseCodec {
    /// Encode with the given byte slice.
    fn encode<I: AsRef<[u8]>>(input: I) -> String;

    /// Decode with the given string.
    fn decode<I: AsRef<str>>(input: I, strict: bool) -> Result<Vec<u8>>;
}

/// Identity, 8-bit binary (encoder and decoder keeps data unmodified).
///
/// # Encoding Behavior
///
/// When encoding with Identity, the input bytes are interpreted as UTF-8.
/// Invalid UTF-8 sequences are replaced with the Unicode replacement character (U+FFFD).
/// This uses [`String::from_utf8_lossy`] to ensure the operation never panics.
///
/// # Security Note
///
/// If you need to preserve exact binary data, consider using a different base encoding
/// like Base64 instead of Identity.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Identity;

impl BaseCodec for Identity {
    fn encode<I: AsRef<[u8]>>(input: I) -> String {
        // Use lossy conversion to prevent panics on invalid UTF-8
        // This is a security improvement - the old code would panic on arbitrary binary data
        String::from_utf8_lossy(input.as_ref()).into_owned()
    }

    fn decode<I: AsRef<str>>(input: I, _strict: bool) -> Result<Vec<u8>> {
        Ok(input.as_ref().as_bytes().to_vec())
    }
}

/// Base256Emoji (alphabet: 🚀🪐☄🛰🌌🌑🌒🌓🌔🌕🌖🌗🌘🌍🌏🌎🐉☀💻🖥💾💿😂❤😍🤣😊🙏💕😭😘👍😅👏😁🔥🥰💔💖💙😢🤔😆🙄💪😉☺👌🤗💜😔😎😇🌹🤦🎉💞✌✨🤷😱😌🌸🙌😋💗💚😏💛🙂💓🤩😄😀🖤😃💯🙈👇🎶😒🤭❣😜💋👀😪😑💥🙋😞😩😡🤪👊🥳😥🤤👉💃😳✋😚😝😴🌟😬🙃🍀🌷😻😓⭐✅🥺🌈😈🤘💦✔😣🏃💐☹🎊💘😠☝😕🌺🎂🌻😐🖕💝🙊😹🗣💫💀👑🎵🤞😛🔴😤🌼😫⚽🤙☕🏆🤫👈😮🙆🍻🍃🐶💁😲🌿🧡🎁⚡🌞🎈❌✊👋😰🤨😶🤝🚶💰🍓💢🤟🙁🚨💨🤬✈🎀🍺🤓😙💟🌱😖👶🥴▶➡❓💎💸⬇😨🌚🦋😷🕺⚠🙅😟😵👎🤲🤠🤧📌🔵💅🧐🐾🍒😗🤑🌊🤯🐷☎💧😯💆👆🎤🙇🍑❄🌴💣🐸💌📍🥀🤢👅💡💩👐📸👻🤐🤮🎼🥵🚩🍎🍊👼💍📣🥂)
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Base256Emoji;

impl BaseCodec for Base256Emoji {
    fn encode<I: AsRef<[u8]>>(input: I) -> String {
        Emoji::encode(input.as_ref())
    }

    fn decode<I: AsRef<str>>(input: I, _strict: bool) -> Result<Vec<u8>> {
        Emoji::decode(input.as_ref()).map_err(|e| e.into())
    }
}

derive_base_encoding! {
    /// Base2 (alphabet: 01).
    Base2, encoding::BASE2, encoding::BASE2_PERMISSIVE;
    /// Base8 (alphabet: 01234567).
    Base8, encoding::BASE8, encoding::BASE8_PERMISSIVE;
    /// Base16 lower hexadecimal (alphabet: 0123456789abcdef).
    Base16Lower, encoding::BASE16_LOWER, encoding::BASE16_LOWER_PERMISSIVE;
    /// Base16 upper hexadecimal (alphabet: 0123456789ABCDEF).
    Base16Upper, encoding::BASE16_UPPER, encoding::BASE16_UPPER_PERMISSIVE;
    /// Base32, rfc4648 no padding (alphabet: abcdefghijklmnopqrstuvwxyz234567).
    Base32Lower, encoding::BASE32_NOPAD_LOWER, encoding::BASE32_NOPAD_LOWER_PERMISSIVE;
    /// Base32, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ234567).
    Base32Upper, encoding::BASE32_NOPAD_UPPER, encoding::BASE32_NOPAD_UPPER_PERMISSIVE;
    /// Base32, rfc4648 with padding (alphabet: abcdefghijklmnopqrstuvwxyz234567).
    Base32PadLower, encoding::BASE32_PAD_LOWER, encoding::BASE32_PAD_LOWER_PERMISSIVE;
    /// Base32, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ234567).
    Base32PadUpper, encoding::BASE32_PAD_UPPER, encoding::BASE32_PAD_UPPER_PERMISSIVE;
    /// Base32hex, rfc4648 no padding (alphabet: 0123456789abcdefghijklmnopqrstuv).
    Base32HexLower, encoding::BASE32HEX_NOPAD_LOWER, encoding::BASE32HEX_NOPAD_LOWER_PERMISSIVE;
    /// Base32hex, rfc4648 no padding (alphabet: 0123456789ABCDEFGHIJKLMNOPQRSTUV).
    Base32HexUpper, encoding::BASE32HEX_NOPAD_UPPER, encoding::BASE32HEX_NOPAD_UPPER_PERMISSIVE;
    /// Base32hex, rfc4648 with padding (alphabet: 0123456789abcdefghijklmnopqrstuv).
    Base32HexPadLower, encoding::BASE32HEX_PAD_LOWER, encoding::BASE32HEX_PAD_LOWER_PERMISSIVE;
    /// Base32hex, rfc4648 with padding (alphabet: 0123456789ABCDEFGHIJKLMNOPQRSTUV).
    Base32HexPadUpper, encoding::BASE32HEX_PAD_UPPER, encoding::BASE32HEX_PAD_UPPER_PERMISSIVE;
    /// z-base-32 (used by Tahoe-LAFS) (alphabet: ybndrfg8ejkmcpqxot1uwisza345h769).
    Base32Z, encoding::BASE32Z, encoding::BASE32Z_PERMISSIVE;
    /// Base64, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/).
    Base64, encoding::BASE64_NOPAD, encoding::BASE64_NOPAD_PERMISSIVE;
    /// Base64, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/).
    Base64Pad, encoding::BASE64_PAD, encoding::BASE64_PAD_PERMISSIVE;
    /// Base64 url, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_).
    Base64Url, encoding::BASE64URL_NOPAD, encoding::BASE64URL_NOPAD_PERMISSIVE;
    /// Base64 url, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_).
    Base64UrlPad, encoding::BASE64URL_PAD, encoding::BASE64URL_PAD_PERMISSIVE;
}

derive_base_x! {
    /// Base10 (alphabet: 0123456789).
    Base10, encoding::BASE10, encoding::BASE10_PERMISSIVE;
    /// Base58 flicker (alphabet: 123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ).
    Base58Flickr, encoding::BASE58_FLICKR, encoding::BASE58_FLICKR_PERMISSIVE;
    /// Base58 bitcoin (alphabet: 123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz).
    Base58Btc, encoding::BASE58_BITCOIN, encoding::BASE58_BITCOIN_PERMISSIVE;
}

/// Base36, [0-9a-z] no padding (alphabet: abcdefghijklmnopqrstuvwxyz0123456789).
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Base36Lower;

impl BaseCodec for Base36Lower {
    fn encode<I: AsRef<[u8]>>(input: I) -> String {
        base_x::encode(encoding::BASE36_LOWER, input.as_ref())
    }

    fn decode<I: AsRef<str>>(input: I, strict: bool) -> Result<Vec<u8>> {
        if strict {
            Ok(base_x::decode(encoding::BASE36_LOWER, input.as_ref())?)
        } else {
            // The input is case insensitive, hence lowercase it
            let lowercased = input.as_ref().to_ascii_lowercase();
            Ok(base_x::decode(
                encoding::BASE36_LOWER_PERMISSIVE,
                &lowercased,
            )?)
        }
    }
}

/// Base36, [0-9A-Z] no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789).
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Base36Upper;

impl BaseCodec for Base36Upper {
    fn encode<I: AsRef<[u8]>>(input: I) -> String {
        base_x::encode(encoding::BASE36_UPPER, input.as_ref())
    }

    fn decode<I: AsRef<str>>(input: I, strict: bool) -> Result<Vec<u8>> {
        if strict {
            Ok(base_x::decode(encoding::BASE36_UPPER, input.as_ref())?)
        } else {
            // The input is case insensitive, hence uppercase it
            let uppercased = input.as_ref().to_ascii_uppercase();
            Ok(base_x::decode(
                encoding::BASE36_UPPER_PERMISSIVE,
                &uppercased,
            )?)
        }
    }
}
