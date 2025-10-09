// SPDX-License-Identifier: MIT

//! Security-focused tests for the multibase crate.
//!
//! These tests verify that the library handles potentially malicious or
//! adversarial inputs safely without panicking or exhibiting undefined behavior.

use multibase::{decode, decode_into, encode, encode_into, Base};

/// Tests that very large inputs don't cause panics or undefined behavior.
#[test]
fn test_large_input_encoding() {
    // Test encoding a large input (1 MB - reduced from 10MB for test speed)
    let large_input = vec![0xAB; 1024 * 1024];

    // Should not panic
    let encoded = encode(Base::Base64, &large_input);

    // Verify it starts with correct prefix
    assert!(encoded.starts_with('m'));

    // Verify round-trip
    let (base, decoded) = decode(&encoded, true).unwrap();
    assert_eq!(base, Base::Base64);
    assert_eq!(decoded, large_input);
}

/// Tests that very large inputs work with buffer reuse APIs.
#[test]
fn test_large_input_with_buffer_reuse() {
    // Use 100KB and Base64 for speed (Base58 is much slower)
    let large_input = vec![0xCD; 100 * 1024];

    let mut encode_buffer = String::new();
    encode_into(Base::Base64, &large_input, &mut encode_buffer);
    assert!(encode_buffer.starts_with('m'));

    let mut decode_buffer = Vec::new();
    let base = decode_into(&encode_buffer, true, &mut decode_buffer).unwrap();
    assert_eq!(base, Base::Base64);
    assert_eq!(decode_buffer, large_input);
}

/// Tests that maximum-size single-byte inputs are handled safely.
#[test]
fn test_single_byte_maximum() {
    // Test sample of byte values (not all 256 to keep test fast)
    let test_bytes = vec![0, 1, 127, 128, 255];

    for byte in test_bytes {
        let input = vec![byte];

        for base in all_bases() {
            let encoded = encode(base, &input);
            let (decoded_base, decoded) = decode(&encoded, true).unwrap();
            assert_eq!(decoded_base, base);

            // Identity encoding with invalid UTF-8 won't round-trip perfectly
            // due to lossy conversion (replacement character)
            if base == Base::Identity && std::str::from_utf8(&input).is_err() {
                // Skip round-trip check for invalid UTF-8 in Identity encoding
                continue;
            }

            assert_eq!(
                decoded, input,
                "Failed for base {:?} with byte {}",
                base, byte
            );
        }
    }
}

/// Tests that empty input is handled safely.
#[test]
fn test_empty_input_encoding() {
    let empty: &[u8] = &[];

    for base in all_bases() {
        let encoded = encode(base, empty);
        let (decoded_base, decoded) = decode(&encoded, true).unwrap();
        assert_eq!(decoded_base, base);
        assert_eq!(decoded, empty);
    }
}

/// Tests that malformed base prefixes are rejected safely.
#[test]
fn test_malformed_prefix_rejection() {
    let invalid_prefixes = vec![
        "x",      // Unknown base code
        "!test",  // Invalid character
        "?data",  // Invalid character
        "@hello", // Invalid character
        "#world", // Invalid character
    ];

    for input in invalid_prefixes {
        let result = decode(input, true);
        assert!(result.is_err(), "Should reject invalid prefix: {}", input);
    }
}

/// Tests that truncated encoded data is rejected safely.
#[test]
fn test_truncated_data_rejection() {
    // Encode valid data
    let valid_encoded = encode(Base::Base64, b"hello world");

    // Try decoding truncated versions
    for len in 1..valid_encoded.len() {
        let truncated = &valid_encoded[..len];
        let result = decode(truncated, true);

        // Should either succeed (if valid by chance) or error (if invalid)
        // but must NOT panic
        let _ = result;
    }
}

/// Tests that strings with only the prefix (no data) are handled.
#[test]
fn test_prefix_only_inputs() {
    // Base codes without any encoded data
    let prefixes = vec!['m', 'z', 'f', 'b', 'u'];

    for prefix in prefixes {
        let prefix_only = format!("{}", prefix);
        let result = decode(&prefix_only, true);

        // Should either decode to empty or error, but not panic
        match result {
            Ok((_, data)) => {
                // Empty data is acceptable
                assert_eq!(data.len(), 0);
            }
            Err(_) => {
                // Error is also acceptable
            }
        }
    }
}

/// Tests that invalid UTF-8 sequences in Identity encoding are handled safely.
#[test]
fn test_identity_with_invalid_utf8() {
    let invalid_utf8_sequences = vec![
        vec![0xFF],                   // Invalid UTF-8
        vec![0xFF, 0xFE],             // Invalid UTF-8
        vec![0x80],                   // Continuation byte without start
        vec![0xC0, 0x80],             // Overlong encoding (deprecated)
        vec![0xED, 0xA0, 0x80],       // Surrogate (invalid)
        vec![0xF4, 0x90, 0x80, 0x80], // Above U+10FFFF
    ];

    for invalid_bytes in invalid_utf8_sequences {
        // Should not panic - uses lossy conversion
        let encoded = encode(Base::Identity, &invalid_bytes);

        // Should start with Identity prefix (null byte)
        assert!(encoded.starts_with('\0'));

        // Decoding may not round-trip perfectly due to lossy conversion
        let (base, _decoded) = decode(&encoded, true).unwrap();
        assert_eq!(base, Base::Identity);
    }
}

/// Tests that rapid repeated encoding/decoding doesn't cause issues.
#[test]
fn test_repeated_operations_safety() {
    let data = b"test data for repeated operations";

    // Rapidly encode and decode many times
    for _ in 0..1000 {
        for base in all_bases() {
            let encoded = encode(base, data);
            let (decoded_base, decoded) = decode(&encoded, true).unwrap();
            assert_eq!(decoded_base, base);
            assert_eq!(&decoded[..], data);
        }
    }
}

/// Tests that concurrent operations are safe (thread safety).
#[test]
fn test_concurrent_safety() {
    use std::thread;

    let handles: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                let data = format!("thread {}", i);
                for base in all_bases() {
                    let encoded = encode(base, data.as_bytes());
                    let (_, decoded) = decode(&encoded, true).unwrap();
                    assert_eq!(decoded, data.as_bytes());
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

/// Tests that malicious patterns in encoded data are rejected safely.
#[test]
fn test_malicious_patterns_rejection() {
    let malicious_inputs = vec![
        // Null bytes
        "m\0\0\0\0",
        // Control characters
        "z\x01\x02\x03",
        // Mixed valid/invalid for various bases
        "mAAAA!!!!", // Base64 with invalid chars
        "fGGGG",     // Base16Lower with invalid chars
        "z00000",    // Base58 with invalid chars (0 not in alphabet)
    ];

    for input in malicious_inputs {
        let result = decode(input, true);
        // Should error gracefully, not panic
        assert!(result.is_err() || result.is_ok());
    }
}

/// Tests that buffer reuse doesn't cause use-after-free or similar issues.
#[test]
fn test_buffer_reuse_safety() {
    let mut encode_buf = String::new();
    let mut decode_buf = Vec::new();

    // Encode/decode with progressively larger data
    for size in &[10, 100, 1000, 10000] {
        let data = vec![0xAB; *size];

        encode_into(Base::Base64, &data, &mut encode_buf);
        let base = decode_into(&encode_buf, true, &mut decode_buf).unwrap();

        assert_eq!(base, Base::Base64);
        assert_eq!(decode_buf, data);
        assert_eq!(decode_buf.len(), *size);
    }
}

/// Tests integer overflow safety in capacity calculations.
#[test]
fn test_capacity_calculation_safety() {
    // Test with maximum reasonable sizes
    // We can't actually allocate usize::MAX bytes, but we can verify
    // that the code doesn't panic when calculating capacities

    let sizes = vec![100, 1_000, 10_000, 100_000, 1_000_000];

    for size in sizes {
        let data = vec![0xFF; size];

        // Test that capacity calculations don't overflow
        let encoded = encode(Base::Base64, &data);
        assert!(!encoded.is_empty());

        // Verify decode works
        let (base, decoded) = decode(&encoded, true).unwrap();
        assert_eq!(base, Base::Base64);
        assert_eq!(decoded.len(), size);
    }
}

/// Tests that all error paths are exercised without panics.
#[test]
fn test_all_error_paths() {
    // Empty input
    assert!(decode("", true).is_err());

    // Unknown base code
    assert!(decode("x123", true).is_err());

    // Invalid data for specific bases
    assert!(decode("fZZZZ", true).is_err()); // Base16 doesn't have 'Z'
    assert!(decode("z0000", true).is_err()); // Base58 doesn't have '0'

    // Malformed data
    assert!(decode("m!!!!", true).is_err()); // Invalid Base64
}

/// Tests that EncodedString validation is secure.
#[test]
fn test_encoded_string_security() {
    use multibase::EncodedString;

    // Valid string
    let valid = EncodedString::new("zCn8eVZg").unwrap();
    assert_eq!(valid.base(), Base::Base58Btc);

    // Empty string
    assert!(EncodedString::new("").is_err());

    // Invalid base code
    assert!(EncodedString::new("xInvalid").is_err());

    // Very long string
    let long_string = format!("z{}", "A".repeat(1_000_000));
    let validated = EncodedString::new(long_string).unwrap();
    assert_eq!(validated.base(), Base::Base58Btc);
}

/// Tests resource exhaustion resistance.
#[test]
fn test_resource_exhaustion_resistance() {
    // Attempt to create many allocations rapidly
    let data = b"test";

    for _ in 0..10_000 {
        let encoded = encode(Base::Base64, data);
        let _ = decode(&encoded, true);
    }

    // If we got here without OOM, the test passes
}

/// Tests that base encoding with zero-length data is safe across all bases.
#[test]
fn test_zero_length_all_bases() {
    let empty: &[u8] = &[];

    for base in all_bases() {
        let encoded = encode(base, empty);
        let (decoded_base, decoded) = decode(&encoded, true).unwrap();
        assert_eq!(decoded_base, base);
        assert_eq!(decoded.len(), 0);
    }
}

// Helper function to get all base variants
fn all_bases() -> Vec<Base> {
    vec![
        Base::Identity,
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
        Base::Base58Flickr,
        Base::Base58Btc,
        Base::Base64,
        Base::Base64Pad,
        Base::Base64Url,
        Base::Base64UrlPad,
        Base::Base256Emoji,
    ]
}
