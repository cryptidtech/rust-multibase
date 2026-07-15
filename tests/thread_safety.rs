// SPDX-License-Identifier: MIT

//! Thread safety tests for the multibase crate.
//!
//! This module contains compile-time and runtime tests to verify that all
//! public types are properly Send and Sync, and that concurrent operations
//! are safe and correct.

use multi_base::{Base, EncodedString, Error};

/// Compile-time assertion that a type implements Send.
const fn assert_send<T: Send>() {}

/// Compile-time assertion that a type implements Sync.
const fn assert_sync<T: Sync>() {}

/// Compile-time assertion that a type implements both Send and Sync.
const fn assert_send_sync<T: Send + Sync>() {}

/// Test that Base enum is Send and Sync.
///
/// The Base enum should be Send and Sync because:
/// - It's a simple enum with no interior mutability
/// - All variants contain no data or only simple data
/// - It's Copy, which implies Send + Sync for its contents
#[test]
fn base_is_send_sync() {
    assert_send::<Base>();
    assert_sync::<Base>();
    assert_send_sync::<Base>();
}

/// Test that Error type is Send and Sync.
///
/// The Error type should be Send and Sync because:
/// - It uses thiserror which generates Send + Sync implementations
/// - All error variants contain only Send + Sync types
/// - No interior mutability is present
#[test]
fn error_is_send_sync() {
    assert_send::<Error>();
    assert_sync::<Error>();
    assert_send_sync::<Error>();
}

/// Test that `EncodedString` is Send and Sync.
///
/// `EncodedString` should be Send and Sync because:
/// - It contains a Base (which is Send + Sync)
/// - It contains a String (which is Send + Sync)
/// - No interior mutability is present
#[test]
fn encoded_string_is_send_sync() {
    assert_send::<EncodedString>();
    assert_sync::<EncodedString>();
    assert_send_sync::<EncodedString>();
}

/// Test that Result types are Send and Sync.
///
/// Result types should inherit Send + Sync from their contained types.
#[test]
fn result_types_are_send_sync() {
    assert_send::<Result<(Base, Vec<u8>), Error>>();
    assert_sync::<Result<(Base, Vec<u8>), Error>>();
    assert_send_sync::<Result<(Base, Vec<u8>), Error>>();

    assert_send::<Result<EncodedString, Error>>();
    assert_sync::<Result<EncodedString, Error>>();
    assert_send_sync::<Result<EncodedString, Error>>();
}

/// Test that Base can be safely sent between threads.
#[test]
fn base_send_between_threads() {
    use std::thread;

    let base = Base::Base64;
    let handle = thread::spawn(move || {
        // Use the base in another thread
        assert_eq!(base.code(), 'm');
    });
    handle.join().unwrap();
}

/// Test that Base can be safely shared between threads.
#[test]
fn base_sync_between_threads() {
    use std::sync::Arc;
    use std::thread;

    let base = Arc::new(Base::Base58Btc);
    let base_clone = Arc::clone(&base);

    let handle = thread::spawn(move || {
        assert_eq!(base_clone.code(), 'z');
    });

    assert_eq!(base.code(), 'z');
    handle.join().unwrap();
}

/// Test that Error can be safely sent between threads.
#[test]
fn error_send_between_threads() {
    use std::thread;

    let error = Error::UnknownBase { code: 'x' };
    let handle = thread::spawn(move || {
        assert!(matches!(error, Error::UnknownBase { code: 'x' }));
    });
    handle.join().unwrap();
}

/// Test that Error can be safely shared between threads.
#[test]
fn error_sync_between_threads() {
    use std::sync::Arc;
    use std::thread;

    let error = Arc::new(Error::EmptyInput);
    let error_clone = Arc::clone(&error);

    let handle = thread::spawn(move || {
        assert!(matches!(*error_clone, Error::EmptyInput));
    });

    assert!(matches!(*error, Error::EmptyInput));
    handle.join().unwrap();
}

/// Test that `EncodedString` can be safely sent between threads.
#[test]
fn encoded_string_send_between_threads() {
    use std::thread;

    let encoded = EncodedString::new("zCn8eVZg").unwrap();
    let handle = thread::spawn(move || {
        assert_eq!(encoded.base(), Base::Base58Btc);
        assert_eq!(encoded.as_str(), "zCn8eVZg");
    });
    handle.join().unwrap();
}

/// Test that `EncodedString` can be safely shared between threads.
#[test]
fn encoded_string_sync_between_threads() {
    use std::sync::Arc;
    use std::thread;

    let encoded = Arc::new(EncodedString::new("md29ybGQ").unwrap());
    let encoded_clone = Arc::clone(&encoded);

    let handle = thread::spawn(move || {
        assert_eq!(encoded_clone.base(), Base::Base64);
        assert_eq!(encoded_clone.as_str(), "md29ybGQ");
    });

    assert_eq!(encoded.base(), Base::Base64);
    handle.join().unwrap();
}

/// Test concurrent encoding operations from multiple threads.
#[test]
fn concurrent_encoding_correctness() {
    use std::sync::Arc;
    use std::thread;

    let test_data = Arc::new(vec![0xAB; 1000]);
    let mut handles = vec![];

    // Spawn 10 threads that all encode the same data
    for _ in 0..10 {
        let data = Arc::clone(&test_data);
        let handle = thread::spawn(move || {
            let encoded = multi_base::encode(Base::Base64, &*data);
            assert!(encoded.starts_with('m'));
            encoded
        });
        handles.push(handle);
    }

    // All threads should produce identical results
    let results: Vec<String> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    for result in &results[1..] {
        assert_eq!(&results[0], result);
    }
}

/// Test concurrent decoding operations from multiple threads.
#[test]
fn concurrent_decoding_correctness() {
    use std::sync::Arc;
    use std::thread;

    let encoded = Arc::new("zCn8eVZg".to_string());
    let mut handles = vec![];

    // Spawn 10 threads that all decode the same string
    for _ in 0..10 {
        let enc = Arc::clone(&encoded);
        let handle = thread::spawn(move || {
            let (base, decoded) = multi_base::decode(&*enc, true).unwrap();
            assert_eq!(base, Base::Base58Btc);
            decoded
        });
        handles.push(handle);
    }

    // All threads should produce identical results
    let results: Vec<Vec<u8>> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    for result in &results[1..] {
        assert_eq!(&results[0], result);
    }
}

/// Test concurrent `Base::from_code` operations.
#[test]
fn concurrent_base_from_code() {
    use std::thread;

    let codes = vec!['m', 'z', 'f', 'b', 'u', 'M', 'Z', 'F', 'B', 'U'];
    let mut handles = vec![];

    for code in codes {
        let handle = thread::spawn(move || {
            let base = Base::from_code(code).unwrap();
            (code, base)
        });
        handles.push(handle);
    }

    for handle in handles {
        let (code, base) = handle.join().unwrap();
        // Verify round-trip
        assert_eq!(base.code(), code);
    }
}

/// Test concurrent `encode_into` with thread-local buffers.
#[test]
fn concurrent_encode_into_thread_local_buffers() {
    use std::sync::Arc;
    use std::thread;

    let test_data = Arc::new(b"test data for concurrent encoding".to_vec());
    let mut handles = vec![];

    for i in 0..10 {
        let data = Arc::clone(&test_data);
        let handle = thread::spawn(move || {
            // Each thread has its own buffer
            let mut buffer = String::new();
            for _ in 0..100 {
                multi_base::encode_into(Base::Base64, &*data, &mut buffer);
                assert!(buffer.starts_with('m'));
            }
            i
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

/// Test concurrent `decode_into` with thread-local buffers.
#[test]
fn concurrent_decode_into_thread_local_buffers() {
    use std::sync::Arc;
    use std::thread;

    let encoded = Arc::new("md29ybGQ".to_string());
    let mut handles = vec![];

    for i in 0..10 {
        let enc = Arc::clone(&encoded);
        let handle = thread::spawn(move || {
            // Each thread has its own buffer
            let mut buffer = Vec::new();
            for _ in 0..100 {
                let base = multi_base::decode_into(&*enc, true, &mut buffer).unwrap();
                assert_eq!(base, Base::Base64);
                assert_eq!(buffer, b"world");
            }
            i
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

/// Test that multiple threads can work with different bases simultaneously.
#[test]
fn concurrent_multi_base_operations() {
    use std::thread;

    let bases = vec![
        Base::Base2,
        Base::Base8,
        Base::Base10,
        Base::Base16Lower,
        Base::Base32Lower,
        Base::Base58Btc,
        Base::Base64,
        Base::Base64Url,
    ];

    let mut handles = vec![];

    for base in bases {
        let handle = thread::spawn(move || {
            let data = b"concurrent test data";
            let encoded = multi_base::encode(base, data);
            let (decoded_base, decoded) = multi_base::decode(&encoded, true).unwrap();
            assert_eq!(decoded_base, base);
            assert_eq!(&decoded[..], data);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

/// Test that `EncodedString` operations are thread-safe.
#[test]
fn concurrent_encoded_string_operations() {
    use std::sync::Arc;
    use std::thread;

    let encoded_strings = vec![
        EncodedString::new("zCn8eVZg").unwrap(),
        EncodedString::new("md29ybGQ").unwrap(),
        EncodedString::new("f48656c6c6f").unwrap(),
    ];

    let shared = Arc::new(encoded_strings);
    let mut handles = vec![];

    for i in 0..10 {
        let strings = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            for encoded in strings.iter() {
                let _base = encoded.base();
                let _str = encoded.as_str();
                let _decoded = encoded.decode().unwrap();
            }
            i
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

/// Test no data races with concurrent read-only access.
#[test]
fn no_data_races_read_only() {
    use std::sync::Arc;
    use std::thread;

    let base = Arc::new(Base::Base64);
    let mut handles = vec![];

    // Multiple threads reading the same Base concurrently
    for _ in 0..20 {
        let b = Arc::clone(&base);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let _code = b.code();
                let _encoded = b.encode(b"test");
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

/// Test that error handling is thread-safe.
#[test]
fn concurrent_error_handling() {
    use std::thread;

    let invalid_inputs = vec!["", "x123", "!invalid", "?bad", "@wrong"];

    let mut handles = vec![];

    for input in invalid_inputs {
        let handle = thread::spawn(move || {
            let result = multi_base::decode(input, true);
            assert!(result.is_err());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

/// Test stress scenario with many concurrent operations.
#[test]
fn stress_test_concurrent_operations() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;

    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    // Spawn 20 threads that each perform 100 operations
    for _ in 0..20 {
        let c = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for i in 0..100 {
                let data = format!("data{i}");
                let encoded = multi_base::encode(Base::Base64, data.as_bytes());
                let (base, decoded) = multi_base::decode(&encoded, true).unwrap();
                assert_eq!(base, Base::Base64);
                assert_eq!(decoded, data.as_bytes());
                c.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Verify all operations completed
    assert_eq!(counter.load(Ordering::SeqCst), 2000);
}
