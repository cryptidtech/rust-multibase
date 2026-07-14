// SPDX-License-Identifier: MIT

use crate::impls::*;

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

/// Generates the `Base` enum and its implementation with encode/decode methods.
///
/// This macro creates a comprehensive base encoding enum with:
/// - Enum variants for each base type with documentation
/// - `from_code()` method to convert character codes to base types
/// - `code()` method to get the character code for a base
/// - `encode()` method to encode bytes to a base string
/// - `decode()` method to decode base strings back to bytes
///
/// # Macro Hygiene
///
/// This macro uses `$crate::` prefixes to ensure proper hygiene when invoked
/// from other modules or crates.
///
/// # Parameters
///
/// The macro accepts a comma-separated list of base definitions in the form:
/// ```text
/// #[doc = "Documentation"] 'code' => VariantName,
/// ```
///
/// Where:
/// - `#[doc = "..."]` - Documentation attribute for the enum variant
/// - `'code'` - The single character or emoji that identifies this base encoding
/// - `VariantName` - The PascalCase name for the enum variant
///
/// # Generated Code
///
/// For each base definition, the macro generates:
/// - An enum variant in the `Base` enum
/// - A match arm in `from_code()` that maps the character code to the variant
/// - A match arm in `code()` that returns the character code
/// - A match arm in `encode()` that delegates to the base's encode function
/// - A match arm in `decode()` that delegates to the base's decode function
///
/// # Example
///
/// ```ignore
/// build_base_enum! {
///     /// Base64 encoding
///     'm' => Base64,
///     /// Base58 Bitcoin encoding
///     'z' => Base58Btc,
/// }
/// ```
///
/// This generates a `Base` enum with `Base64` and `Base58Btc` variants,
/// and implements all required methods.
///
/// # Error Handling
///
/// The `from_code()` method returns `Result<Base>` and will return
/// `Error::UnknownBase` if given an unrecognized character code.
macro_rules! build_base_enum {
    ( $(#[$attr:meta] $code:expr => $base:ident,)* ) => {
        /// List of types currently supported in the multibase spec.
        ///
        /// Not all base types are supported by this library.
        #[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
        pub enum Base {
            $( #[$attr] $base, )*
        }

        impl Base {
            /// Convert a character code to the matching base algorithm.
            ///
            /// Returns `Error::UnknownBase` if the code doesn't match any supported base.
            pub fn from_code(code: char) -> $crate::error::Result<Self> {
        	    match code {
                    $( $code => Ok(Self::$base), )*
            	    _ => Err($crate::error::Error::UnknownBase { code }),
        	    }
            }

            /// Get the character code corresponding to the base algorithm.
            ///
            /// Each base encoding has a unique single-character (or emoji) prefix
            /// that identifies it in multibase strings.
            pub fn code(&self) -> char {
                match self {
                    $( Self::$base => $code, )*
                }
            }

            /// Encode the given byte slice to a base string (without prefix).
            ///
            /// This method returns only the encoded data, without the base code prefix.
            /// Use the public `encode()` function to get the full multibase string.
            pub fn encode<I: AsRef<[u8]>>(&self, input: I) -> String {
                match self {
                    $( Self::$base => $crate::impls::$base::encode(input), )*
                }
            }

            /// Decode the base string (without prefix) back to bytes.
            ///
            /// The `strict` parameter controls whether to accept case-insensitive
            /// input for applicable bases.
            ///
            /// # Parameters
            ///
            /// * `input` - The encoded string (without the multibase prefix)
            /// * `strict` - If true, enforce strict decoding rules (case-sensitive, etc.)
            ///
            /// # Errors
            ///
            /// Returns an error if the input contains invalid characters for this base
            /// or if the input is malformed.
            pub fn decode<I: AsRef<str>>(&self, input: I, strict: bool) -> $crate::error::Result<Vec<u8>> {
                match self {
                    $( Self::$base => $crate::impls::$base::decode(input, strict), )*
                }
            }

            pub(crate) fn decode_into<I: AsRef<str>>(&self, input: I, strict: bool, buffer: &mut Vec<u8>) -> $crate::error::Result<()> {
                match self {
                    $( Self::$base => $crate::impls::$base::decode_into(input, strict, buffer), )*
                }
            }
        }
    }
}

build_base_enum! {
    /// 8-bit binary (encoder and decoder keeps data unmodified).
    '\x00' => Identity,
    /// Base2 (alphabet: 01).
    '0' => Base2,
    /// Base8 (alphabet: 01234567).
    '7' => Base8,
    /// Base10 (alphabet: 0123456789).
    '9' => Base10,
    /// Base16 lower hexadecimal (alphabet: 0123456789abcdef).
    'f' => Base16Lower,
    /// Base16 upper hexadecimal (alphabet: 0123456789ABCDEF).
    'F' => Base16Upper,
     /// Base32, rfc4648 no padding (alphabet: abcdefghijklmnopqrstuvwxyz234567).
    'b' => Base32Lower,
    /// Base32, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ234567).
    'B' => Base32Upper,
    /// Base32, rfc4648 with padding (alphabet: abcdefghijklmnopqrstuvwxyz234567).
    'c' => Base32PadLower,
    /// Base32, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ234567).
    'C' => Base32PadUpper,
    /// Base32hex, rfc4648 no padding (alphabet: 0123456789abcdefghijklmnopqrstuv).
    'v' => Base32HexLower,
    /// Base32hex, rfc4648 no padding (alphabet: 0123456789ABCDEFGHIJKLMNOPQRSTUV).
    'V' => Base32HexUpper,
    /// Base32hex, rfc4648 with padding (alphabet: 0123456789abcdefghijklmnopqrstuv).
    't' => Base32HexPadLower,
    /// Base32hex, rfc4648 with padding (alphabet: 0123456789ABCDEFGHIJKLMNOPQRSTUV).
    'T' => Base32HexPadUpper,
    /// z-base-32 (used by Tahoe-LAFS) (alphabet: ybndrfg8ejkmcpqxot1uwisza345h769).
    'h' => Base32Z,
    /// Base36, [0-9a-z] no padding (alphabet: 0123456789abcdefghijklmnopqrstuvwxyz).
    'k' => Base36Lower,
    /// Base36, [0-9A-Z] no padding (alphabet: 0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ).
    'K' => Base36Upper,
    /// Base58 flicker (alphabet: 123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ).
    'Z' => Base58Flickr,
    /// Base58 bitcoin (alphabet: 123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz).
    'z' => Base58Btc,
    /// Base64, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/).
    'm' => Base64,
    /// Base64, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/).
    'M' => Base64Pad,
    /// Base64 url, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_).
    'u' => Base64Url,
    /// Base64 url, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_).
    'U' => Base64UrlPad,
    /// Base256Emoji (alphabet: 🚀🪐☄🛰🌌🌑🌒🌓🌔🌕🌖🌗🌘🌍🌏🌎🐉☀💻🖥💾💿😂❤😍🤣😊🙏💕😭😘👍😅👏😁🔥🥰💔💖💙😢🤔😆🙄💪😉☺👌🤗💜😔😎😇🌹🤦🎉💞✌✨🤷😱😌🌸🙌😋💗💚😏💛🙂💓🤩😄😀🖤😃💯🙈👇🎶😒🤭❣😜💋👀😪😑💥🙋😞😩😡🤪👊🥳😥🤤👉💃😳✋😚😝😴🌟😬🙃🍀🌷😻😓⭐✅🥺🌈😈🤘💦✔😣🏃💐☹🎊💘😠☝😕🌺🎂🌻😐🖕💝🙊😹🗣💫💀👑🎵🤞😛🔴😤🌼😫⚽🤙☕🏆🤫👈😮🙆🍻🍃🐶💁😲🌿🧡🎁⚡🌞🎈❌✊👋😰🤨😶🤝🚶💰🍓💢🤟🙁🚨💨🤬✈🎀🍺🤓😙💟🌱😖👶🥴▶➡❓💎💸⬇😨🌚🦋😷🕺⚠🙅😟😵👎🤲🤠🤧📌🔵💅🧐🐾🍒😗🤑🌊🤯🐷☎💧😯💆👆🎤🙇🍑❄🌴💣🐸💌📍🥀🤢👅💡💩👐📸👻🤐🤮🎼🥵🚩🍎🍊👼💍📣🥂)
    '🚀' => Base256Emoji,
}
