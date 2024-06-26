use data_encoding::Encoding;
use data_encoding_macro::new_encoding;

// Base2 (alphabet: 01)
pub const BASE2_PERMISSIVE: Encoding = BASE2;
pub const BASE2: Encoding = new_encoding! {
    symbols: "01",
};

// Base8 (alphabet: 01234567)
pub const BASE8_PERMISSIVE: Encoding = BASE8;
pub const BASE8: Encoding = new_encoding! {
    symbols: "01234567",
};

/// Base10 (alphabet: 0123456789)
pub const BASE10_PERMISSIVE: &str = BASE10;
pub const BASE10: &str = "0123456789";

// Base16 lower hexadecimal (alphabet: 0123456789abcdef)
pub const BASE16_LOWER_PERMISSIVE: Encoding = data_encoding::HEXLOWER_PERMISSIVE;
pub const BASE16_LOWER: Encoding = data_encoding::HEXLOWER;

// Base16 upper hexadecimal (alphabet: 0123456789ABCDEF).
pub const BASE16_UPPER_PERMISSIVE: Encoding = data_encoding::HEXUPPER_PERMISSIVE;
pub const BASE16_UPPER: Encoding = data_encoding::HEXUPPER;

// Base32, rfc4648 no padding (alphabet: abcdefghijklmnopqrstuvwxyz234567).
pub const BASE32_NOPAD_LOWER_PERMISSIVE: Encoding = new_encoding! {
    symbols: "abcdefghijklmnopqrstuvwxyz234567",
    translate_from: "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    translate_to: "abcdefghijklmnopqrstuvwxyz",
};
pub const BASE32_NOPAD_LOWER: Encoding = new_encoding! {
    symbols: "abcdefghijklmnopqrstuvwxyz234567",
};

// Base32, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ234567).
pub const BASE32_NOPAD_UPPER_PERMISSIVE: Encoding = new_encoding! {
    symbols: "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567",
    translate_from: "abcdefghijklmnopqrstuvwxyz",
    translate_to: "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
};
pub const BASE32_NOPAD_UPPER: Encoding = new_encoding! {
    symbols: "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567",
};

// Base32, rfc4648 with padding (alphabet: abcdefghijklmnopqrstuvwxyz234567).
pub const BASE32_PAD_LOWER_PERMISSIVE: Encoding = new_encoding! {
    symbols: "abcdefghijklmnopqrstuvwxyz234567",
    translate_from: "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    translate_to: "abcdefghijklmnopqrstuvwxyz",
    padding: '=',
};
pub const BASE32_PAD_LOWER: Encoding = new_encoding! {
    symbols: "abcdefghijklmnopqrstuvwxyz234567",
    padding: '=',
};

// Base32, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ234567).
pub const BASE32_PAD_UPPER_PERMISSIVE: Encoding = new_encoding! {
    symbols: "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567",
    translate_from: "abcdefghijklmnopqrstuvwxyz",
    translate_to: "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    padding: '=',
};
pub const BASE32_PAD_UPPER: Encoding = new_encoding! {
    symbols: "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567",
    padding: '=',
};

// Base32hex, rfc4648 no padding (alphabet: 0123456789abcdefghijklmnopqrstuv).
pub const BASE32HEX_NOPAD_LOWER_PERMISSIVE: Encoding = new_encoding! {
    symbols: "0123456789abcdefghijklmnopqrstuv",
    translate_from: "ABCDEFGHIJKLMNOPQRSTUV",
    translate_to: "abcdefghijklmnopqrstuv",
};
pub const BASE32HEX_NOPAD_LOWER: Encoding = new_encoding! {
    symbols: "0123456789abcdefghijklmnopqrstuv",
};

// Base32hex, rfc4648 no padding (alphabet: 0123456789ABCDEFGHIJKLMNOPQRSTUV).
pub const BASE32HEX_NOPAD_UPPER_PERMISSIVE: Encoding = new_encoding! {
    symbols: "0123456789ABCDEFGHIJKLMNOPQRSTUV",
    translate_from: "abcdefghijklmnopqrstuv",
    translate_to: "ABCDEFGHIJKLMNOPQRSTUV",
};
pub const BASE32HEX_NOPAD_UPPER: Encoding = new_encoding! {
    symbols: "0123456789ABCDEFGHIJKLMNOPQRSTUV",
};

// Base32hex, rfc4648 with padding (alphabet: 0123456789abcdefghijklmnopqrstuv).
pub const BASE32HEX_PAD_LOWER_PERMISSIVE: Encoding = new_encoding! {
    symbols: "0123456789abcdefghijklmnopqrstuv",
    translate_from: "ABCDEFGHIJKLMNOPQRSTUV",
    translate_to: "abcdefghijklmnopqrstuv",
    padding: '=',
};
pub const BASE32HEX_PAD_LOWER: Encoding = new_encoding! {
    symbols: "0123456789abcdefghijklmnopqrstuv",
    padding: '=',
};

/// Base32hex, rfc4648 with padding (alphabet: 0123456789ABCDEFGHIJKLMNOPQRSTUV).
pub const BASE32HEX_PAD_UPPER_PERMISSIVE: Encoding = new_encoding! {
    symbols: "0123456789ABCDEFGHIJKLMNOPQRSTUV",
    translate_from: "abcdefghijklmnopqrstuv",
    translate_to: "ABCDEFGHIJKLMNOPQRSTUV",
    padding: '=',
};
pub const BASE32HEX_PAD_UPPER: Encoding = new_encoding! {
    symbols: "0123456789ABCDEFGHIJKLMNOPQRSTUV",
    padding: '=',
};

// z-base-32 (used by Tahoe-LAFS) (alphabet: ybndrfg8ejkmcpqxot1uwisza345h769).
pub const BASE32Z_PERMISSIVE: Encoding = BASE32Z;
pub const BASE32Z: Encoding = new_encoding! {
    symbols: "ybndrfg8ejkmcpqxot1uwisza345h769",
};

/// Base36, [0-9a-z] no padding (alphabet: 0123456789abcdefghijklmnopqrstuvwxyz).
pub const BASE36_LOWER_PERMISSIVE: &str = BASE36_LOWER;
pub const BASE36_LOWER: &str = "0123456789abcdefghijklmnopqrstuvwxyz";

/// Base36, [0-9A-Z] no padding (alphabet: 0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ).
pub const BASE36_UPPER_PERMISSIVE: &str = BASE36_UPPER;
pub const BASE36_UPPER: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

// Base58 Flickr's alphabet for creating short urls from photo ids.
pub const BASE58_FLICKR_PERMISSIVE: &str = BASE58_FLICKR;
pub const BASE58_FLICKR: &str = "123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";

// Base58 Bitcoin's alphabet as defined in their Base58Check encoding.
pub const BASE58_BITCOIN_PERMISSIVE: &str = BASE58_BITCOIN;
pub const BASE58_BITCOIN: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

// Base64, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/).
pub const BASE64_NOPAD_PERMISSIVE: Encoding = BASE64_NOPAD;
pub const BASE64_NOPAD: Encoding = data_encoding::BASE64_NOPAD;

// Base64, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/).
pub const BASE64_PAD_PERMISSIVE: Encoding = BASE64_PAD;
pub const BASE64_PAD: Encoding = data_encoding::BASE64;

// Base64 url, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_).
pub const BASE64URL_NOPAD_PERMISSIVE: Encoding = BASE64URL_NOPAD;
pub const BASE64URL_NOPAD: Encoding = data_encoding::BASE64URL_NOPAD;

// Base64 url, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_).
pub const BASE64URL_PAD_PERMISSIVE: Encoding = BASE64URL_PAD;
pub const BASE64URL_PAD: Encoding = data_encoding::BASE64URL;
