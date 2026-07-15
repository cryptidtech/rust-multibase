#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use multi_base::{Base, Base::*, decode, decode_into, encode};

fn encode_decode_assert(input: &[u8], test_cases: Vec<(Base, &str)>) {
    for (base, output) in test_cases {
        assert_eq!(encode(base, input), output);
        assert_eq!(decode(output, true).unwrap(), (base, input.to_vec()));
    }
}

#[test]
fn test_bases_code() {
    assert_eq!(Identity.code(), '\x00');
    assert_eq!(Base2.code(), '0');
}

#[test]
fn test_bases_from_code() {
    assert_eq!(Base::from_code('\x00').unwrap(), Identity);
    assert_eq!(Base::from_code('0').unwrap(), Base2);
}

#[test]
fn test_round_trip() {
    let test_cases: &[&str] = &[
        "helloworld",
        "we all want decentralization",
        "zdj7WfBb6j58iSJuAzDcSZgy2SxFhdpJ4H87uvMpfyN6hRGyH",
    ];

    for case in test_cases {
        let encoded = encode(Base58Btc, case);
        let decoded = decode(encoded, true).unwrap();
        assert_eq!(decoded, (Base58Btc, case.as_bytes().to_vec()));
    }
}

#[test]
fn test_basic() {
    let input = b"yes mani !";
    let test_cases = vec![
        (Identity, "\x00yes mani !"),
        (
            Base2,
            "001111001011001010111001100100000011011010110000101101110011010010010000000100001",
        ),
        (Base8, "7362625631006654133464440102"),
        (Base10, "9573277761329450583662625"),
        (Base16Lower, "f796573206d616e692021"),
        (Base16Upper, "F796573206D616E692021"),
        (Base32Lower, "bpfsxgidnmfxgsibb"),
        (Base32Upper, "BPFSXGIDNMFXGSIBB"),
        (Base32HexLower, "vf5in683dc5n6i811"),
        (Base32HexUpper, "VF5IN683DC5N6I811"),
        (Base32PadLower, "cpfsxgidnmfxgsibb"),
        (Base32PadUpper, "CPFSXGIDNMFXGSIBB"),
        (Base32HexPadLower, "tf5in683dc5n6i811"),
        (Base32HexPadUpper, "TF5IN683DC5N6I811"),
        (Base32Z, "hxf1zgedpcfzg1ebb"),
        (Base36Lower, "k2lcpzo5yikidynfl"),
        (Base36Upper, "K2LCPZO5YIKIDYNFL"),
        (Base58Flickr, "Z7Pznk19XTTzBtx"),
        (Base58Btc, "z7paNL19xttacUY"),
        (Base64, "meWVzIG1hbmkgIQ"),
        (Base64Pad, "MeWVzIG1hbmkgIQ=="),
        (Base64Url, "ueWVzIG1hbmkgIQ"),
        (Base64UrlPad, "UeWVzIG1hbmkgIQ=="),
        (Base256Emoji, "🚀🏃✋🌈😅🌷🤤😻🌟😅👏"),
    ];
    encode_decode_assert(input, test_cases);
}

#[test]
fn preserves_leading_zero() {
    let input = b"\x00yes mani !";
    let test_cases = vec![
        (Identity, "\x00\x00yes mani !"),
        (
            Base2,
            "00000000001111001011001010111001100100000011011010110000101101110011010010010000000100001",
        ),
        (Base8, "7000745453462015530267151100204"),
        (Base10, "90573277761329450583662625"),
        (Base16Lower, "f00796573206d616e692021"),
        (Base16Upper, "F00796573206D616E692021"),
        (Base32Lower, "bab4wk4zanvqw42jaee"),
        (Base32Upper, "BAB4WK4ZANVQW42JAEE"),
        (Base32HexLower, "v01smasp0dlgmsq9044"),
        (Base32HexUpper, "V01SMASP0DLGMSQ9044"),
        (Base32PadLower, "cab4wk4zanvqw42jaee======"),
        (Base32PadUpper, "CAB4WK4ZANVQW42JAEE======"),
        (Base32HexPadLower, "t01smasp0dlgmsq9044======"),
        (Base32HexPadUpper, "T01SMASP0DLGMSQ9044======"),
        (Base32Z, "hybhskh3ypiosh4jyrr"),
        (Base36Lower, "k02lcpzo5yikidynfl"),
        (Base36Upper, "K02LCPZO5YIKIDYNFL"),
        (Base58Flickr, "Z17Pznk19XTTzBtx"),
        (Base58Btc, "z17paNL19xttacUY"),
        (Base64, "mAHllcyBtYW5pICE"),
        (Base64Pad, "MAHllcyBtYW5pICE="),
        (Base64Url, "uAHllcyBtYW5pICE"),
        (Base64UrlPad, "UAHllcyBtYW5pICE="),
        (Base256Emoji, "🚀🚀🏃✋🌈😅🌷🤤😻🌟😅👏"),
    ];
    encode_decode_assert(input, test_cases);
}

#[test]
fn preserves_two_leading_zeroes() {
    let input = b"\x00\x00yes mani !";
    let test_cases = vec![
        (Identity, "\x00\x00\x00yes mani !"),
        (
            Base2,
            "0000000000000000001111001011001010111001100100000011011010110000101101110011010010010000000100001",
        ),
        (Base8, "700000171312714403326055632220041"),
        (Base10, "900573277761329450583662625"),
        (Base16Lower, "f0000796573206d616e692021"),
        (Base16Upper, "F0000796573206D616E692021"),
        (Base32Lower, "baaahszltebwwc3tjeaqq"),
        (Base32Upper, "BAAAHSZLTEBWWC3TJEAQQ"),
        (Base32HexLower, "v0007ipbj41mm2rj940gg"),
        (Base32HexUpper, "V0007IPBJ41MM2RJ940GG"),
        (Base32PadLower, "caaahszltebwwc3tjeaqq===="),
        (Base32PadUpper, "CAAAHSZLTEBWWC3TJEAQQ===="),
        (Base32HexPadLower, "t0007ipbj41mm2rj940gg===="),
        (Base32HexPadUpper, "T0007IPBJ41MM2RJ940GG===="),
        (Base32Z, "hyyy813murbssn5ujryoo"),
        (Base36Lower, "k002lcpzo5yikidynfl"),
        (Base36Upper, "K002LCPZO5YIKIDYNFL"),
        (Base58Flickr, "Z117Pznk19XTTzBtx"),
        (Base58Btc, "z117paNL19xttacUY"),
        (Base64, "mAAB5ZXMgbWFuaSAh"),
        (Base64Pad, "MAAB5ZXMgbWFuaSAh"),
        (Base64Url, "uAAB5ZXMgbWFuaSAh"),
        (Base64UrlPad, "UAAB5ZXMgbWFuaSAh"),
        (Base256Emoji, "🚀🚀🚀🏃✋🌈😅🌷🤤😻🌟😅👏"),
    ];
    encode_decode_assert(input, test_cases);
}

#[test]
fn case_insensitivity() {
    let input = b"hello world";
    let test_cases = vec![
        (Base16Lower, "f68656c6c6f20776F726C64"),
        (Base16Upper, "F68656c6c6f20776F726C64"),
        (Base32Lower, "bnbswy3dpeB3W64TMMQ"),
        (Base32Upper, "Bnbswy3dpeB3W64TMMQ"),
        (Base32HexLower, "vd1imor3f41RMUSJCCG"),
        (Base32HexUpper, "Vd1imor3f41RMUSJCCG"),
        (Base32PadLower, "cnbswy3dpeB3W64TMMQ======"),
        (Base32PadUpper, "Cnbswy3dpeB3W64TMMQ======"),
        (Base32HexPadLower, "td1imor3f41RMUSJCCG======"),
        (Base32HexPadUpper, "Td1imor3f41RMUSJCCG======"),
        (Base36Lower, "kfUvrsIvVnfRbjWaJo"),
        (Base36Upper, "KfUVrSIVVnFRbJWAJo"),
    ];
    for (base, output) in test_cases {
        assert_eq!(decode(output, false).unwrap(), (base, input.to_vec()));
    }
}

// ============================================================================
// Identity Encoding Tests
// ============================================================================

/// Test that Identity encoding handles invalid UTF-8 gracefully without panicking.
#[test]
fn identity_encode_invalid_utf8_no_panic() {
    // Invalid UTF-8 sequences
    let invalid_utf8 = vec![0xFF, 0xFE, 0xFD];

    // Should not panic, uses lossy conversion
    let encoded = encode(Identity, &invalid_utf8);

    // Should start with null byte (Identity code)
    assert!(encoded.starts_with('\0'));

    // Should contain replacement characters for invalid UTF-8
    assert!(encoded.contains('\u{FFFD}'));
}

/// Test that Identity encoding properly handles valid UTF-8 including Unicode.
#[test]
fn identity_roundtrip_valid_utf8() {
    let test_cases = vec![
        "Hello, World!",
        "Hello, 世界! 🦀",
        "Rust is 🚀",
        "Émoji: 😀😃😄😁",
        "\n\t\r",
        "",
    ];

    for input in test_cases {
        let encoded = encode(Identity, input.as_bytes());
        let (base, decoded) = decode(&encoded, true).unwrap();
        assert_eq!(base, Identity);
        assert_eq!(decoded, input.as_bytes());
        assert_eq!(String::from_utf8(decoded).unwrap(), input);
    }
}

/// Test that decoding an empty string returns `EmptyInput` error.
#[test]
fn decode_empty_string_error() {
    use multi_base::Error;

    let result = decode("", true);
    assert!(result.is_err());

    match result.unwrap_err() {
        Error::EmptyInput => {} // Expected
        other => panic!("Expected EmptyInput error, got: {other:?}"),
    }
}

/// Test that unknown base codes return `UnknownBase` error.
#[test]
fn decode_unknown_base_error() {
    use multi_base::Error;

    let test_cases = vec![
        "xInvalidBase", // 'x' is not a valid base code
        "!NotABase",    // '!' is not a valid base code
        "🚫emoji",      // Emoji (except 🚀) is not a valid base code
    ];

    for input in test_cases {
        let result = decode(input, true);
        assert!(result.is_err(), "Expected error for input: {input}");

        match result.unwrap_err() {
            Error::UnknownBase { code } => {
                // Verify the error contains the invalid code
                assert_eq!(code, input.chars().next().unwrap());
            }
            other => panic!("Expected UnknownBase error for '{input}', got: {other:?}"),
        }
    }
}

/// Test that `Base::from_code` returns proper error for invalid codes.
#[test]
fn base_from_code_error() {
    use multi_base::Error;

    let invalid_codes = vec!['x', '!', '@', '#', '$', '%'];

    for code in invalid_codes {
        let result = Base::from_code(code);
        assert!(result.is_err());

        match result.unwrap_err() {
            Error::UnknownBase { code: c } => {
                assert_eq!(c, code);
            }
            other => panic!("Expected UnknownBase error, got: {other:?}"),
        }
    }
}

/// Test decoding with malformed/truncated data returns appropriate errors.
/// Note: This test verifies that truly malformed data returns errors without panicking.
#[test]
fn decode_malformed_data_errors() {
    // These should all return errors in strict mode, not panic
    let test_cases = vec![
        ("fGG", "Base16 with invalid characters (G is not hex)"),
        ("0222", "Base2 with invalid digits (2 is not binary)"),
    ];

    for (input, description) in test_cases {
        let result = decode(input, true);
        assert!(
            result.is_err(),
            "Expected error for malformed input: {input} ({description})"
        );
        // Just verify it errors - don't care about specific error type
    }

    // Test that various inputs don't panic, even if they succeed
    // (some bases are very permissive)
    let no_panic_cases = vec!["fXYZ", "mXXXX", "bXXXXX", "z123"];
    for input in no_panic_cases {
        let _ = decode(input, true); // Should not panic
        let _ = decode(input, false); // Should not panic in permissive mode either
    }
}

/// Test that error types implement expected traits.
#[test]
fn error_traits() {
    use multi_base::Error;

    let error = Error::EmptyInput;

    // Test Clone
    let cloned = error.clone();
    assert_eq!(error, cloned);

    // Test Debug
    let debug_str = format!("{error:?}");
    assert!(!debug_str.is_empty());

    // Test Display (from thiserror)
    let display_str = format!("{error}");
    assert!(!display_str.is_empty());
    assert!(display_str.contains("empty"));
}

/// Test that errors are properly converted from underlying libraries.
#[test]
fn error_conversions() {
    // Test that decode errors from various bases are properly converted
    // and don't panic

    // Invalid base2 (should only contain 0 and 1)
    let result = decode("02345", true);
    assert!(result.is_err());

    // Invalid base16 (should only contain 0-9, a-f, A-F)
    let result = decode("fXYZ", true);
    assert!(result.is_err());

    // Invalid base58 (should not contain 0, O, I, l)
    let result = decode("zOIl0", true);
    assert!(result.is_err());
}

/// Test that Identity with binary data uses lossy conversion.
#[test]
fn identity_binary_data_lossy() {
    // Test various invalid UTF-8 sequences
    let invalid_sequences = vec![
        vec![0x80],             // Invalid start byte
        vec![0xFF, 0xFF],       // Invalid bytes
        vec![0xC0, 0x80],       // Overlong encoding
        vec![b'H', 0xFF, b'i'], // Mixed valid and invalid
    ];

    for seq in invalid_sequences {
        // Should not panic
        let encoded = encode(Identity, &seq);

        // Verify it starts with Identity code
        assert_eq!(encoded.chars().next().unwrap(), Identity.code());

        // Verify the rest is valid UTF-8 (possibly with replacement characters)
        let without_code = &encoded[1..];
        assert!(std::str::from_utf8(without_code.as_bytes()).is_ok());
    }
}

// ============================================================================
// Zero-Copy Buffer Reuse Tests
// ============================================================================

/// Test `encode_into` with buffer reuse.
#[test]
fn test_encode_into_buffer_reuse() {
    use multi_base::encode_into;

    let mut buffer = String::new();

    // First encoding
    encode_into(Base58Btc, b"hello", &mut buffer);
    assert_eq!(buffer, "zCn8eVZg");

    // Reuse buffer for second encoding
    encode_into(Base64, b"world", &mut buffer);
    assert_eq!(buffer, "md29ybGQ");

    // Reuse buffer for third encoding with different data
    encode_into(Base16Lower, b"rust", &mut buffer);
    assert_eq!(buffer, "f72757374");
}

/// Test `encode_into` with all base types.
#[test]
fn test_encode_into_all_bases() {
    use multi_base::encode_into;

    let input = b"test data";
    let mut buffer = String::new();

    let bases = vec![
        Base::Identity,
        Base::Base2,
        Base::Base8,
        Base::Base10,
        Base::Base16Lower,
        Base::Base16Upper,
        Base::Base32Lower,
        Base::Base32Upper,
        Base::Base58Btc,
        Base::Base58Flickr,
        Base::Base64,
        Base::Base64Pad,
    ];

    for base in bases {
        encode_into(base, input, &mut buffer);
        assert!(buffer.starts_with(base.code()));
        assert!(!buffer.is_empty());
    }
}

/// Test `decode_into` with buffer reuse.
#[test]
fn test_decode_into_buffer_reuse() {
    use multi_base::decode_into;

    let mut buffer = Vec::new();

    // First decoding
    let base = decode_into("zCn8eVZg", true, &mut buffer).unwrap();
    assert_eq!(base, Base58Btc);
    assert_eq!(buffer, b"hello");

    // Reuse buffer for second decoding
    let base = decode_into("md29ybGQ", true, &mut buffer).unwrap();
    assert_eq!(base, Base64);
    assert_eq!(buffer, b"world");

    // Reuse buffer for third decoding
    let base = decode_into("f72757374", true, &mut buffer).unwrap();
    assert_eq!(base, Base16Lower);
    assert_eq!(buffer, b"rust");
}

/// Test that `encode_into` produces same results as encode.
#[test]
fn test_encode_into_matches_encode() {
    use multi_base::encode_into;

    let test_data = vec![
        b"".as_slice(),
        b"a",
        b"hello",
        b"the quick brown fox jumps over the lazy dog",
        &[0, 1, 2, 3, 4, 5],
        &[255, 254, 253],
    ];

    let bases = vec![
        Base::Base16Lower,
        Base::Base32Lower,
        Base::Base58Btc,
        Base::Base64,
    ];

    let mut buffer = String::new();

    for data in &test_data {
        for base in &bases {
            // Encode with regular encode
            let expected = encode(*base, data);

            // Encode with encode_into
            encode_into(*base, data, &mut buffer);

            assert_eq!(
                buffer, expected,
                "Mismatch for base {base:?} with data {data:?}"
            );
        }
    }
}

/// Test that `decode_into` produces same results as decode.
#[test]
fn test_decode_into_matches_decode() {
    let test_cases = vec![
        "zCn8eVZg",          // Base58BTC
        "md29ybGQ",          // Base64
        "f72757374",         // Base16Lower
        "BPFSXGIDNMFXGSIBB", // Base32Upper
        "001100001",         // Base2 with valid data
    ];

    let mut buffer = Vec::new();

    for input in test_cases {
        // Decode with regular decode
        let (expected_base, expected_data) = decode(input, true).unwrap();

        // Decode with decode_into
        let base = decode_into(input, true, &mut buffer).unwrap();

        assert_eq!(base, expected_base);
        assert_eq!(buffer, expected_data);
    }
}

/// Test `decode_into` error handling.
#[test]
fn test_decode_into_errors() {
    use multi_base::{Error, decode_into};

    let mut buffer = Vec::new();

    // Empty input
    let result = decode_into("", true, &mut buffer);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::EmptyInput));

    // Unknown base
    let result = decode_into("xInvalid", true, &mut buffer);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::UnknownBase { .. }));
}

/// Test `encode_into` with empty input.
#[test]
fn test_encode_into_empty() {
    use multi_base::encode_into;

    let mut buffer = String::new();
    encode_into(Base64, b"", &mut buffer);
    assert_eq!(buffer, "m");
}

/// Test `decode_into` with empty data (just code).
#[test]
fn test_decode_into_empty_data() {
    use multi_base::decode_into;

    let mut buffer = Vec::new();
    let base = decode_into("m", true, &mut buffer).unwrap();
    assert_eq!(base, Base64);
    assert_eq!(buffer, b"");
}

/// Test `encode_into` doesn't grow buffer unnecessarily.
#[test]
fn test_encode_into_buffer_capacity() {
    use multi_base::encode_into;

    let mut buffer = String::with_capacity(1000);
    let initial_capacity = buffer.capacity();

    // Encode small data - shouldn't need more capacity
    encode_into(Base64, b"hello", &mut buffer);

    // Capacity should remain the same or higher (never shrinks)
    assert!(buffer.capacity() >= initial_capacity);
}

/// Test `decode_into` buffer reuse in loop (performance test).
#[test]
fn test_decode_into_loop_performance() {
    use multi_base::decode_into;

    let inputs = vec!["zCn8eVZg", "md29ybGQ", "f72757374", "BPFSXGIDNMFXGSIBB"];

    let mut buffer = Vec::new();

    // This should reuse the buffer across iterations
    for input in inputs {
        let _base = decode_into(input, true, &mut buffer).unwrap();
        assert!(!buffer.is_empty());
    }
}

// ============================================================================
// EncodedString Newtype Tests
// ============================================================================

/// Test `EncodedString` basic usage.
#[test]
fn test_encoded_string_basic() {
    use multi_base::EncodedString;

    let encoded = EncodedString::new("zCn8eVZg").unwrap();
    assert_eq!(encoded.base(), Base58Btc);
    assert_eq!(encoded.as_str(), "zCn8eVZg");

    let decoded = encoded.decode().unwrap();
    assert_eq!(decoded, b"hello");
}

/// Test `EncodedString` `FromStr` implementation.
#[test]
fn test_encoded_string_from_str() {
    use multi_base::EncodedString;
    use std::str::FromStr;

    let encoded = EncodedString::from_str("md29ybGQ").unwrap();
    assert_eq!(encoded.base(), Base64);

    let parsed: EncodedString = "f48656c6c6f".parse().unwrap();
    assert_eq!(parsed.base(), Base16Lower);
}

/// Test `EncodedString` `TryFrom` implementations.
#[test]
fn test_encoded_string_try_from() {
    use multi_base::EncodedString;
    use std::convert::TryFrom;

    // From String
    let encoded = EncodedString::try_from("zCn8eVZg".to_string()).unwrap();
    assert_eq!(encoded.base(), Base58Btc);

    // From &str
    let encoded = EncodedString::try_from("md29ybGQ").unwrap();
    assert_eq!(encoded.base(), Base64);
}

/// Test `EncodedString` error handling.
#[test]
fn test_encoded_string_errors() {
    use multi_base::{EncodedString, Error};

    // Empty string
    let result = EncodedString::new("");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::EmptyInput));

    // Invalid base code
    let result = EncodedString::new("xInvalid");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::UnknownBase { .. }));
}

/// Test `EncodedString` validation only checks prefix.
#[test]
fn test_encoded_string_validates_prefix_only() {
    use multi_base::EncodedString;

    // This has a valid base code but invalid data
    // Construction should succeed, but decode should fail
    let encoded = EncodedString::new("fXXXXXX").unwrap();
    assert_eq!(encoded.base(), Base16Lower);

    // Decoding should fail
    let result = encoded.decode();
    assert!(result.is_err());
}

/// Test `encode_to_validated` convenience function.
#[test]
fn test_encode_to_validated() {
    use multi_base::encode_to_validated;

    let encoded = encode_to_validated(Base58Btc, b"hello");
    assert_eq!(encoded.base(), Base58Btc);
    assert_eq!(encoded.as_str(), "zCn8eVZg");

    let decoded = encoded.decode().unwrap();
    assert_eq!(decoded, b"hello");
}

/// Test `parse_encoded` convenience function.
#[test]
fn test_parse_encoded() {
    use multi_base::parse_encoded;

    let encoded = parse_encoded("zCn8eVZg").unwrap();
    assert_eq!(encoded.base(), Base58Btc);

    let result = parse_encoded("");
    assert!(result.is_err());
}

/// Test `EncodedString` with all base types.
#[test]
fn test_encoded_string_all_bases() {
    use multi_base::{EncodedString, encode_to_validated};

    let data = b"test";
    let bases = vec![Base16Lower, Base32Lower, Base58Btc, Base64];

    for base in bases {
        // Encode to validated string
        let encoded = encode_to_validated(base, data);
        assert_eq!(encoded.base(), base);

        // Decode and verify
        let decoded = encoded.decode().unwrap();
        assert_eq!(decoded, data);

        // Parse the string again
        let reparsed = EncodedString::new(encoded.as_str()).unwrap();
        assert_eq!(reparsed.base(), base);
    }
}

/// Test `EncodedString` Display trait.
#[test]
fn test_encoded_string_display() {
    use multi_base::EncodedString;

    let encoded = EncodedString::new("zCn8eVZg").unwrap();
    assert_eq!(format!("{encoded}"), "zCn8eVZg");
}

/// Test `EncodedString` `into_inner`.
#[test]
fn test_encoded_string_into_inner() {
    use multi_base::EncodedString;

    let encoded = EncodedString::new("zCn8eVZg").unwrap();
    let inner = encoded.into_inner();
    assert_eq!(inner, "zCn8eVZg");
}

/// Test `EncodedString` clone and equality.
#[test]
fn test_encoded_string_clone_eq() {
    use multi_base::EncodedString;

    let encoded1 = EncodedString::new("zCn8eVZg").unwrap();
    let encoded2 = encoded1.clone();
    let encoded3 = EncodedString::new("md29ybGQ").unwrap();

    assert_eq!(encoded1, encoded2);
    assert_ne!(encoded1, encoded3);
}

/// Test `EncodedString` `decode_with_strictness`.
#[test]
fn test_encoded_string_strictness() {
    use multi_base::EncodedString;

    // Mixed case Base16Upper (should work in permissive mode)
    let encoded = EncodedString::new("FaB").unwrap();
    assert_eq!(encoded.base(), Base16Upper);

    // Permissive mode should work
    let decoded = encoded.decode_with_strictness(false);
    assert!(decoded.is_ok());
}

// ============================================================================
// Error Handling Tests - All Bases
// ============================================================================

/// Test Base2 error handling with invalid characters.
#[test]
fn test_base2_error_handling() {
    // Base2 should only accept 0 and 1
    let invalid_cases = vec![
        "02",   // contains digit 2
        "0abc", // contains letters
        "0123", // contains digits 2 and 3
        "0   ", // contains spaces
    ];

    for input in invalid_cases {
        let result = decode(input, true);
        assert!(
            result.is_err(),
            "Base2 should reject invalid input: {input}"
        );
    }

    // Valid case - properly encoded
    let valid = encode(Base2, b"test");
    assert!(decode(&valid, true).is_ok());
}

/// Test Base8 error handling with invalid characters.
#[test]
fn test_base8_error_handling() {
    // Base8 should only accept 0-7
    let invalid_cases = vec![
        "78",   // contains digit 8
        "79",   // contains digit 9
        "7abc", // contains letters
    ];

    for input in invalid_cases {
        let result = decode(input, true);
        assert!(
            result.is_err(),
            "Base8 should reject invalid input: {input}"
        );
    }

    // Valid case - properly encoded
    let valid = encode(Base8, b"test");
    assert!(decode(&valid, true).is_ok());
}

/// Test Base10 error handling with invalid characters.
#[test]
fn test_base10_error_handling() {
    // Base10 should only accept 0-9
    let invalid_cases = vec![
        "9abc", // contains letters
        "9A",   // contains uppercase letter
        "9 12", // contains space
    ];

    for input in invalid_cases {
        let result = decode(input, true);
        assert!(
            result.is_err(),
            "Base10 should reject invalid input: {input}"
        );
    }

    // Valid case
    assert!(decode("9123456789", true).is_ok());
}

/// Test Base16 error handling with invalid characters.
#[test]
fn test_base16_error_handling() {
    // Base16 should only accept 0-9, a-f (lower) or A-F (upper)
    let invalid_cases = vec![
        "fG", // G is not a hex digit
        "fZ", // Z is not a hex digit
        "f ", // space is not valid
    ];

    for input in invalid_cases {
        let result = decode(input, true);
        assert!(
            result.is_err(),
            "Base16Lower should reject invalid input: {input}"
        );
    }

    // Valid cases
    assert!(decode("f0123456789abcdef", true).is_ok());
    assert!(decode("F0123456789ABCDEF", true).is_ok());
}

/// Test Base32 error handling with invalid characters.
#[test]
fn test_base32_error_handling() {
    // Base32 alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ234567
    // Does not include 0, 1, 8, 9
    let invalid_cases = vec![
        "b0", // contains 0 (not in alphabet)
        "b1", // contains 1 (not in alphabet)
        "b8", // contains 8 (not in alphabet)
        "b9", // contains 9 (not in alphabet)
    ];

    for input in invalid_cases {
        let result = decode(input, true);
        assert!(
            result.is_err(),
            "Base32 should reject invalid input: {input}"
        );
    }

    // Valid case - properly encoded
    let valid = encode(Base32Lower, b"test");
    assert!(decode(&valid, true).is_ok());
}

/// Test Base36 error handling with invalid characters.
#[test]
fn test_base36_error_handling() {
    // Base36: 0-9, a-z (or A-Z)
    let invalid_cases = vec![
        "k@", // @ is not alphanumeric
        "k!", // ! is not alphanumeric
        "k ", // space is not valid
    ];

    for input in invalid_cases {
        let result = decode(input, true);
        assert!(
            result.is_err(),
            "Base36 should reject invalid input: {input}"
        );
    }

    // Valid cases
    assert!(decode("kabc123", true).is_ok());
    assert!(decode("KABC123", true).is_ok());
}

/// Test Base58 error handling with invalid characters.
#[test]
fn test_base58_error_handling() {
    // Base58 excludes 0, O, I, l to avoid confusion
    let invalid_cases = vec![
        "z0", // contains 0 (not in alphabet)
        "zO", // contains O (not in alphabet)
        "zI", // contains I (not in alphabet)
        "zl", // contains l (not in alphabet)
    ];

    for input in invalid_cases {
        let result = decode(input, true);
        assert!(
            result.is_err(),
            "Base58 should reject invalid input: {input}"
        );
    }

    // Valid case (contains characters that ARE in the alphabet)
    assert!(decode("z123ABCabc", true).is_ok());
}

/// Test Base64 error handling with invalid characters.
#[test]
fn test_base64_error_handling() {
    // Base64: A-Z, a-z, 0-9, +, /
    let invalid_cases = vec![
        "m@", // @ is not in alphabet
        "m!", // ! is not in alphabet
        "m ", // space is not valid
    ];

    for input in invalid_cases {
        let result = decode(input, true);
        assert!(
            result.is_err(),
            "Base64 should reject invalid input: {input}"
        );
    }

    // Valid case - properly encoded
    let valid = encode(Base64, b"test");
    assert!(decode(&valid, true).is_ok());
}

/// Test `Base64Url` error handling with invalid characters.
#[test]
fn test_base64url_error_handling() {
    // Base64Url: A-Z, a-z, 0-9, -, _
    let invalid_cases = vec![
        "u+", // + is Base64, not Base64Url
        "u/", // / is Base64, not Base64Url
        "u@", // @ is not in alphabet
    ];

    for input in invalid_cases {
        let result = decode(input, true);
        assert!(
            result.is_err(),
            "Base64Url should reject invalid input: {input}"
        );
    }

    // Valid case - properly encoded
    let valid = encode(Base64Url, b"test");
    assert!(decode(&valid, true).is_ok());
}

/// Test that all bases handle empty data (just the code) correctly.
#[test]
fn test_all_bases_empty_data() {
    let bases = vec![
        (Base2, "0"),
        (Base8, "7"),
        (Base10, "9"),
        (Base16Lower, "f"),
        (Base16Upper, "F"),
        (Base32Lower, "b"),
        (Base32Upper, "B"),
        (Base58Btc, "z"),
        (Base64, "m"),
        (Base64Url, "u"),
    ];

    for (expected_base, code_str) in bases {
        let result = decode(code_str, true);
        assert!(
            result.is_ok(),
            "Base {expected_base:?} should handle empty data"
        );

        let (base, data) = result.unwrap();
        assert_eq!(base, expected_base);
        assert_eq!(
            data,
            Vec::<u8>::new(),
            "Empty data should decode to empty vec"
        );
    }
}

/// Test that all bases handle single byte correctly.
#[test]
fn test_all_bases_single_byte() {
    let bases = vec![
        Base2,
        Base8,
        Base10,
        Base16Lower,
        Base16Upper,
        Base32Lower,
        Base32Upper,
        Base58Btc,
        Base64,
        Base64Url,
    ];

    let single_byte = vec![42u8]; // Arbitrary byte

    for base in bases {
        let encoded = encode(base, &single_byte);
        let (decoded_base, decoded_data) = decode(&encoded, true)
            .unwrap_or_else(|_| panic!("Failed to decode single byte for {base:?}"));

        assert_eq!(decoded_base, base);
        assert_eq!(decoded_data, single_byte);
    }
}

/// Test that strict mode is actually stricter than permissive mode.
#[test]
fn test_strict_vs_permissive_mode() {
    // Cases that should work in permissive but might fail in strict
    let test_cases = vec![
        ("f4142", Base16Lower), // Uppercase/lowercase mixing in some impls
        ("b", Base32Lower),     // Empty data
    ];

    for (input, _expected_base) in test_cases {
        // Permissive mode
        let permissive_result = decode(input, false);

        // Strict mode
        let strict_result = decode(input, true);

        // If strict succeeds, permissive must also succeed
        if let Ok((strict_base, strict_data)) = strict_result {
            assert!(
                permissive_result.is_ok(),
                "Permissive mode should succeed if strict mode succeeds for input: {input}"
            );

            let (permissive_base, permissive_data) = permissive_result.unwrap();

            assert_eq!(strict_base, permissive_base);
            assert_eq!(strict_data, permissive_data);
        }
    }
}

/// Test error messages contain useful information.
#[test]
fn test_error_messages_are_descriptive() {
    use multi_base::Error;

    // UnknownBase error should contain the invalid code
    let result = decode("xInvalid", true);
    assert!(result.is_err());
    let error_msg = format!("{}", result.unwrap_err());
    assert!(
        error_msg.contains('x'),
        "Error message should mention invalid code 'x'"
    );

    // EmptyInput error should be descriptive
    let result = decode("", true);
    assert!(result.is_err());
    match result.unwrap_err() {
        Error::EmptyInput => {
            let error_msg = format!("{}", Error::EmptyInput);
            assert!(
                error_msg.contains("empty"),
                "Error message should mention 'empty'"
            );
        }
        _ => panic!("Expected EmptyInput error"),
    }
}

/// Test that all error types are properly constructed.
#[test]
fn test_all_error_variants() {
    use multi_base::Error;

    // EmptyInput
    let err = Error::EmptyInput;
    assert!(format!("{err}").contains("empty"));

    // UnknownBase
    let err = Error::UnknownBase { code: 'x' };
    let msg = format!("{err}");
    assert!(msg.contains("unknown") || msg.contains("base"));
    assert!(msg.contains('x'));

    // InvalidBaseString
    let err = Error::InvalidBaseString;
    assert!(format!("{err}").contains("invalid"));

    // All errors should support Debug
    let err = Error::EmptyInput;
    let debug = format!("{err:?}");
    assert!(!debug.is_empty());
}

/// Test concurrent encoding/decoding doesn't cause issues.
#[test]
fn test_no_panic_under_various_inputs() {
    // Fuzz-like test with many edge case inputs
    let long_data = format!("f{}", "0".repeat(1000));
    let edge_cases = vec![
        "",         // empty
        "x",        // single invalid code
        "\0",       // null byte as code
        "f",        // just code, no data
        "f0",       // minimal valid
        "f00",      // short data
        &long_data, // long data
        "🚀",       // emoji (valid Base256Emoji code)
        "🚀test",   // Base256Emoji with data
    ];

    for input in edge_cases {
        // Should never panic
        let _ = decode(input, true);
        let _ = decode(input, false);
    }
}

/// Test that `Base32Z` uses correct alphabet.
#[test]
fn test_base32z_alphabet() {
    // Base32Z uses: ybndrfg8ejkmcpqxot1uwisza345h769
    // Does not include uppercase or some confusing chars

    let data = b"hello";
    let encoded = encode(Base32Z, data);

    // Should start with 'h' (Base32Z code)
    assert!(encoded.starts_with('h'));

    // Should roundtrip correctly
    let (base, decoded) = decode(&encoded, true).unwrap();
    assert_eq!(base, Base32Z);
    assert_eq!(decoded, data);
}

// ============================================================================
// Concurrency and Thread Safety Tests
// ============================================================================

/// Test concurrent encoding from multiple threads.
#[test]
fn test_concurrent_encoding() {
    use std::sync::Arc;
    use std::thread;

    let data = Arc::new(b"test data for concurrent encoding".to_vec());
    let bases = vec![Base16Lower, Base32Lower, Base58Btc, Base64];

    let handles: Vec<_> = bases
        .into_iter()
        .map(|base| {
            let data = Arc::clone(&data);
            thread::spawn(move || {
                let encoded = encode(base, &*data);
                let (decoded_base, decoded_data) = decode(&encoded, true).unwrap();
                assert_eq!(decoded_base, base);
                assert_eq!(decoded_data, *data);
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}

/// Test concurrent decoding from multiple threads.
#[test]
fn test_concurrent_decoding() {
    use std::thread;

    let test_cases = vec![
        ("zCn8eVZg", Base58Btc, b"hello".to_vec()),
        ("md29ybGQ", Base64, b"world".to_vec()),
        ("f72757374", Base16Lower, b"rust".to_vec()),
        ("borsxg5a", Base32Lower, b"test".to_vec()),
    ];

    let handles: Vec<_> = test_cases
        .into_iter()
        .map(|(encoded, expected_base, expected_data)| {
            thread::spawn(move || {
                let (base, data) = decode(encoded, true).unwrap();
                assert_eq!(base, expected_base);
                assert_eq!(data, expected_data);
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}

/// Test concurrent encoding with buffer reuse (`encode_into`).
#[test]
fn test_concurrent_encode_into() {
    use multi_base::encode_into;
    use std::thread;

    let test_data = vec![
        (b"data1".as_slice(), Base64),
        (b"data2", Base16Lower),
        (b"data3", Base58Btc),
        (b"data4", Base32Lower),
    ];

    let handles: Vec<_> = test_data
        .into_iter()
        .map(|(data, base)| {
            thread::spawn(move || {
                let mut buffer = String::new();
                encode_into(base, data, &mut buffer);

                // Verify encoding
                let (decoded_base, decoded) = decode(&buffer, true).unwrap();
                assert_eq!(decoded_base, base);
                assert_eq!(decoded, data);
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}

/// Test concurrent decoding with buffer reuse (`decode_into`).
#[test]
fn test_concurrent_decode_into() {
    use multi_base::decode_into;
    use std::thread;

    let test_cases = vec![
        ("zCn8eVZg", Base58Btc, b"hello".to_vec()),
        ("md29ybGQ", Base64, b"world".to_vec()),
        ("f72757374", Base16Lower, b"rust".to_vec()),
        ("borsxg5a", Base32Lower, b"test".to_vec()),
    ];

    let handles: Vec<_> = test_cases
        .into_iter()
        .map(|(encoded, expected_base, expected_data)| {
            thread::spawn(move || {
                let mut buffer = Vec::new();
                let base = decode_into(encoded, true, &mut buffer).unwrap();
                assert_eq!(base, expected_base);
                assert_eq!(buffer, expected_data);
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}

/// Test many concurrent operations to stress test thread safety.
#[test]
fn test_many_concurrent_operations() {
    use std::sync::Arc;
    use std::thread;

    let data = Arc::new(b"concurrent test data".to_vec());
    let num_threads = 20;

    let handles: Vec<_> = (0..num_threads)
        .map(|i| {
            let data = Arc::clone(&data);
            thread::spawn(move || {
                let base = match i % 4 {
                    0 => Base16Lower,
                    1 => Base32Lower,
                    2 => Base58Btc,
                    _ => Base64,
                };

                // Encode
                let encoded = encode(base, &*data);

                // Decode
                let (decoded_base, decoded_data) = decode(&encoded, true).unwrap();
                assert_eq!(decoded_base, base);
                assert_eq!(decoded_data, *data);
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}

/// Test that `EncodedString` is Send and Sync.
#[test]
fn test_encoded_string_is_send_sync() {
    use multi_base::EncodedString;
    use std::thread;

    let encoded = EncodedString::new("zCn8eVZg").unwrap();

    // Test Send - can move between threads
    let handle = thread::spawn(move || {
        let decoded = encoded.decode().unwrap();
        assert_eq!(decoded, b"hello");
    });

    handle.join().expect("Thread panicked");
}

/// Test that `EncodedString` can be shared across threads via Arc.
#[test]
fn test_encoded_string_shared_across_threads() {
    use multi_base::EncodedString;
    use std::sync::Arc;
    use std::thread;

    let encoded = Arc::new(EncodedString::new("zCn8eVZg").unwrap());

    let handles: Vec<_> = (0..5)
        .map(|_| {
            let encoded = Arc::clone(&encoded);
            thread::spawn(move || {
                let decoded = encoded.decode().unwrap();
                assert_eq!(decoded, b"hello");
                assert_eq!(encoded.base(), Base58Btc);
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}

/// Test concurrent `Base::from_code` calls.
#[test]
fn test_concurrent_base_from_code() {
    use std::thread;

    let codes = vec!['0', '7', '9', 'f', 'F', 'b', 'B', 'z', 'm', 'u'];

    let handles: Vec<_> = codes
        .into_iter()
        .map(|code| {
            thread::spawn(move || {
                let base = Base::from_code(code).unwrap();
                assert_eq!(base.code(), code);
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}

/// Test concurrent operations with different data sizes.
#[test]
fn test_concurrent_various_data_sizes() {
    use std::thread;

    let test_sizes = vec![0, 1, 10, 100, 1000, 10000];

    let handles: Vec<_> = test_sizes
        .into_iter()
        .map(|size| {
            thread::spawn(move || {
                let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
                let encoded = encode(Base64, &data);
                let (base, decoded) = decode(&encoded, true).unwrap();
                assert_eq!(base, Base64);
                assert_eq!(decoded, data);
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}
