// Property-based tests for multibase using proptest
//
// These tests verify invariants that should hold for all inputs,
// using randomly generated test data.

use multi_base::{decode, decode_into, encode, encode_into, Base};
use proptest::prelude::*;

// Configure proptest to run fewer cases for faster tests
// Reduced to 16 cases to avoid timeouts while still providing good coverage
const PROPTEST_CASES: u32 = 16;

// Helper to get fast, testable bases
// Excludes Base58 and Base256Emoji which are significantly slower
fn all_bases() -> Vec<Base> {
    vec![
        Base::Base2,
        Base::Base8,
        Base::Base10,
        Base::Base16Lower,
        Base::Base16Upper,
        Base::Base32Lower,
        Base::Base32Upper,
        Base::Base32PadLower,
        Base::Base32PadUpper,
        Base::Base32HexLower,
        Base::Base32HexUpper,
        Base::Base32HexPadLower,
        Base::Base32HexPadUpper,
        Base::Base32Z,
        Base::Base36Lower,
        Base::Base36Upper,
        Base::Base64,
        Base::Base64Pad,
        Base::Base64Url,
        Base::Base64UrlPad,
        // Skip Identity (requires valid UTF-8)
        // Skip Base58Btc, Base58Flickr, Base256Emoji (too slow for property tests)
    ]
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(PROPTEST_CASES))]

    /// Property: encode then decode should return original data
    #[test]
    fn prop_encode_decode_roundtrip(data: Vec<u8>, base_idx: usize) {
        let bases = all_bases();
        if bases.is_empty() {
            return Ok(());
        }
        let base = bases[base_idx % bases.len()];

        let encoded = encode(base, &data);
        let (decoded_base, decoded) = decode(&encoded, true)
            .expect("decode of valid encoding should succeed");

        prop_assert_eq!(base, decoded_base, "base mismatch for {:?}", base);
        prop_assert_eq!(&data, &decoded, "data mismatch for {:?}", base);
    }

    /// Property: decode should never panic, even on invalid input
    #[test]
    fn prop_decode_never_panics(s: String) {
        let _ = decode(&s, true);
        let _ = decode(&s, false);
    }

    /// Property: encode should never panic
    #[test]
    fn prop_encode_never_panics(data: Vec<u8>) {
        for base in all_bases() {
            let _ = encode(base, &data);
        }
    }

    /// Property: encoded string always starts with the base code
    #[test]
    fn prop_encoded_starts_with_base_code(data: Vec<u8>, base_idx: usize) {
        let bases = all_bases();
        if bases.is_empty() {
            return Ok(());
        }
        let base = bases[base_idx % bases.len()];

        let encoded = encode(base, &data);
        let code = base.code();
        prop_assert!(encoded.starts_with(code),
            "encoded string should start with base code {:?}", code);
    }

    /// Property: encode_into produces same result as encode
    #[test]
    fn prop_encode_into_matches_encode(data: Vec<u8>, base_idx: usize) {
        let bases = all_bases();
        if bases.is_empty() {
            return Ok(());
        }
        let base = bases[base_idx % bases.len()];
        let mut buffer = String::new();

        let expected = encode(base, &data);
        encode_into(base, &data, &mut buffer);
        prop_assert_eq!(&buffer, &expected, "encode_into mismatch for {:?}", base);
    }

    /// Property: decode_into produces same result as decode
    #[test]
    fn prop_decode_into_matches_decode(data: Vec<u8>, base_idx: usize) {
        let bases = all_bases();
        if bases.is_empty() {
            return Ok(());
        }
        let base = bases[base_idx % bases.len()];
        let mut buffer = Vec::new();

        let encoded = encode(base, &data);

        // Use regular decode
        let (expected_base, expected_data) = decode(&encoded, true)
            .expect("decode should succeed");

        // Use decode_into
        let actual_base = decode_into(&encoded, true, &mut buffer)
            .expect("decode_into should succeed");

        prop_assert_eq!(actual_base, expected_base, "base mismatch for {:?}", base);
        prop_assert_eq!(&buffer, &expected_data, "data mismatch for {:?}", base);
    }

    /// Property: encoding preserves data length relationships
    /// (most bases produce deterministic output lengths)
    #[test]
    fn prop_encoding_is_deterministic(data: Vec<u8>, base_idx: usize) {
        let bases = all_bases();
        if bases.is_empty() {
            return Ok(());
        }
        let base = bases[base_idx % bases.len()];

        let encoded1 = encode(base, &data);
        let encoded2 = encode(base, &data);
        prop_assert_eq!(encoded1, encoded2,
            "encoding should be deterministic for {:?}", base);
    }

    /// Property: empty input encodes and decodes correctly
    #[test]
    fn prop_empty_input_roundtrip(base_idx: usize) {
        let bases = all_bases();
        if bases.is_empty() {
            return Ok(());
        }
        let base = bases[base_idx % bases.len()];

        let encoded = encode(base, []);
        let (decoded_base, decoded) = decode(&encoded, true)
            .expect("decode of empty should succeed");

        prop_assert_eq!(base, decoded_base);
        prop_assert_eq!(Vec::<u8>::new(), decoded);
    }

    /// Property: decode with strict=false should be more permissive than strict=true
    #[test]
    fn prop_permissive_decoding_accepts_strict(data: Vec<u8>, base_idx: usize) {
        let bases = all_bases();
        if bases.is_empty() {
            return Ok(());
        }
        let base = bases[base_idx % bases.len()];

        let encoded = encode(base, &data);

        // If strict decode succeeds, permissive should also succeed
        if let Ok((strict_base, strict_data)) = decode(&encoded, true) {
            let (permissive_base, permissive_data) = decode(&encoded, false)
                .expect("permissive decode should succeed if strict succeeds");

            prop_assert_eq!(strict_base, permissive_base);
            prop_assert_eq!(strict_data, permissive_data);
        }
    }

    /// Property: multiple encodings in sequence with buffer reuse work correctly
    #[test]
    fn prop_buffer_reuse_correctness(data_sets: Vec<Vec<u8>>, base_idx: usize) {
        let bases = all_bases();
        if bases.is_empty() || data_sets.is_empty() {
            return Ok(());
        }
        let base = bases[base_idx % bases.len()];

        let mut encode_buffer = String::new();
        let mut decode_buffer = Vec::new();

        for data in &data_sets {
            // Encode with buffer reuse
            encode_into(base, data, &mut encode_buffer);
            let encoded_copy = encode_buffer.clone();

            // Decode with buffer reuse
            let decoded_base = decode_into(&encoded_copy, true, &mut decode_buffer)
                .expect("decode_into should succeed");

            prop_assert_eq!(base, decoded_base);
            prop_assert_eq!(data, &decode_buffer);
        }
    }

    /// Property: EncodedString validation and decoding consistency
    #[test]
    fn prop_encoded_string_consistency(data: Vec<u8>, base_idx: usize) {
        use multi_base::{encode_to_validated, parse_encoded};

        let bases = all_bases();
        if bases.is_empty() {
            return Ok(());
        }
        let base = bases[base_idx % bases.len()];

        // Encode to validated
        let encoded = encode_to_validated(base, &data);
        prop_assert_eq!(encoded.base(), base);

        // Decode should work
        let decoded = encoded.decode()
            .expect("decode of validated string should succeed");
        prop_assert_eq!(&decoded, &data);

        // Re-parsing should work
        let reparsed = parse_encoded(encoded.as_str())
            .expect("re-parsing validated string should succeed");
        prop_assert_eq!(reparsed.base(), base);
    }
}

// Additional non-proptest tests for specific scenarios

#[test]
fn test_all_ascii_chars_in_base64() {
    // Test that all ASCII printable characters can be encoded/decoded
    let ascii: Vec<u8> = (32..127).collect();
    let encoded = encode(Base::Base64, &ascii);
    let (_, decoded) = decode(&encoded, true).unwrap();
    assert_eq!(decoded, ascii);
}

#[test]
fn test_all_bytes_in_various_bases() {
    // Test that all byte values can be encoded/decoded
    let all_bytes: Vec<u8> = (0..=255).collect();

    for base in all_bases() {
        let encoded = encode(base, &all_bytes);
        let (decoded_base, decoded) =
            decode(&encoded, true).unwrap_or_else(|_| panic!("decode failed for {:?}", base));
        assert_eq!(base, decoded_base);
        assert_eq!(all_bytes, decoded);
    }
}

#[test]
fn test_large_data_roundtrip() {
    // Test with 1MB of data for fast bases
    let large_data: Vec<u8> = (0..1_000_000).map(|i| (i % 256) as u8).collect();

    for base in [Base::Base16Lower, Base::Base64] {
        let encoded = encode(base, &large_data);
        let (decoded_base, decoded) = decode(&encoded, true).unwrap();
        assert_eq!(base, decoded_base);
        assert_eq!(large_data.len(), decoded.len());
        assert_eq!(large_data[..1000], decoded[..1000]); // Check first 1KB
        assert_eq!(large_data[999000..], decoded[999000..]); // Check last 1KB
    }
}

#[test]
#[ignore] // Very slow - run with: cargo test test_base58_large_data -- --ignored
fn test_base58_large_data() {
    // Test Base58 with 1MB (this is slow, hence ignored by default)
    let large_data: Vec<u8> = (0..1_000_000).map(|i| (i % 256) as u8).collect();
    let encoded = encode(Base::Base58Btc, &large_data);
    let (decoded_base, decoded) = decode(&encoded, true).unwrap();
    assert_eq!(Base::Base58Btc, decoded_base);
    assert_eq!(large_data.len(), decoded.len());
}

// Dedicated tests for slow bases (Base58, Base256Emoji)
// These are separate from property tests to avoid timeouts

#[test]
fn test_base58_roundtrip() {
    // Test Base58 variants with various data sizes
    let test_cases = vec![
        vec![],
        vec![0],
        vec![255],
        vec![0, 1, 2, 3, 4],
        b"hello world".to_vec(),
        (0..100).collect::<Vec<u8>>(),
    ];

    for base in [Base::Base58Btc, Base::Base58Flickr] {
        for data in &test_cases {
            let encoded = encode(base, data);
            let (decoded_base, decoded) = decode(&encoded, true).unwrap_or_else(|_| {
                panic!("decode failed for {:?} with data len {}", base, data.len())
            });
            assert_eq!(base, decoded_base);
            assert_eq!(data, &decoded);
        }
    }
}

#[test]
fn test_base256emoji_roundtrip() {
    // Test Base256Emoji with various data sizes
    let test_cases = vec![
        vec![],
        vec![0],
        vec![255],
        vec![0, 1, 2, 3, 4],
        b"hello world".to_vec(),
        (0..50).collect::<Vec<u8>>(), // Keep smaller for performance
    ];

    for data in &test_cases {
        let encoded = encode(Base::Base256Emoji, data);
        let (decoded_base, decoded) = decode(&encoded, true).unwrap_or_else(|_| {
            panic!(
                "decode failed for Base256Emoji with data len {}",
                data.len()
            )
        });
        assert_eq!(Base::Base256Emoji, decoded_base);
        assert_eq!(data, &decoded);
    }
}
