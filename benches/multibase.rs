use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use rand::RngExt;

use multi_base::{Base, decode, decode_into, encode, encode_into, encode_to_validated};

fn bench_encode(c: &mut Criterion) {
    let mut rng = rand::rng();
    let data_large: Vec<u8> = (0..1024).map(|_| rng.random()).collect();
    let data_small: Vec<u8> = (0..32).map(|_| rng.random()).collect();

    let mut group = c.benchmark_group("encode");

    // Large data benchmarks (1KB)
    group.bench_function("base32_large", |b| {
        b.iter(|| {
            let _ = black_box(encode(Base::Base32Upper, &data_large));
        });
    });
    group.bench_function("base58btc_large", |b| {
        b.iter(|| {
            let _ = black_box(encode(Base::Base58Btc, &data_large));
        });
    });
    group.bench_function("base64_large", |b| {
        b.iter(|| {
            let _ = black_box(encode(Base::Base64, &data_large));
        });
    });

    // Small data benchmarks (32 bytes) - shows insert(0) impact more clearly
    group.bench_function("base32_small", |b| {
        b.iter(|| {
            let _ = black_box(encode(Base::Base32Upper, &data_small));
        });
    });
    group.bench_function("base58btc_small", |b| {
        b.iter(|| {
            let _ = black_box(encode(Base::Base58Btc, &data_small));
        });
    });
    group.bench_function("base64_small", |b| {
        b.iter(|| {
            let _ = black_box(encode(Base::Base64, &data_small));
        });
    });

    group.finish();
}

fn bench_decode(c: &mut Criterion) {
    let mut rng = rand::rng();
    let data: Vec<usize> = (0..1024).map(|_| rng.random::<u32>() as usize).collect();

    let base32 = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let base58 = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    let base64 = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut base32_data = data
        .iter()
        .map(|i| base32[i % 31] as char)
        .collect::<String>();
    base32_data.insert(0, Base::Base32Upper.code());
    let mut base58_data = data
        .iter()
        .map(|i| base58[i % 57] as char)
        .collect::<String>();
    base58_data.insert(0, Base::Base58Btc.code());
    let mut base64_data = data
        .iter()
        .map(|i| base64[i % 64] as char)
        .collect::<String>();
    base64_data.insert(0, Base::Base64.code());

    let mut group = c.benchmark_group("decode");
    group.bench_function("base32", |b| {
        b.iter(|| {
            let _ = black_box(decode(&base32_data, false).unwrap());
        });
    });
    group.bench_function("base58btc", |b| {
        b.iter(|| {
            let _ = black_box(decode(&base58_data, false).unwrap());
        });
    });
    group.bench_function("base64", |b| {
        b.iter(|| {
            let _ = black_box(decode(&base64_data, false).unwrap());
        });
    });
    group.finish();
}

// Benchmark zero-copy encode_into API
fn bench_encode_into(c: &mut Criterion) {
    let mut rng = rand::rng();
    let data: Vec<u8> = (0..1024).map(|_| rng.random()).collect();

    let mut group = c.benchmark_group("encode_into");

    for base in &[Base::Base16Lower, Base::Base32Lower, Base::Base64] {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{base:?}")),
            base,
            |b, &base| {
                let mut buffer = String::new();
                b.iter(|| {
                    encode_into(black_box(base), black_box(&data), &mut buffer);
                    black_box(&buffer);
                });
            },
        );
    }

    group.finish();
}

// Benchmark zero-copy decode_into API
fn bench_decode_into(c: &mut Criterion) {
    let mut rng = rand::rng();
    let data: Vec<u8> = (0..1024).map(|_| rng.random()).collect();

    let encoded_base16 = encode(Base::Base16Lower, &data);
    let encoded_base32 = encode(Base::Base32Lower, &data);
    let encoded_base64 = encode(Base::Base64, &data);

    let mut group = c.benchmark_group("decode_into");

    group.bench_function("base16_lower", |b| {
        let mut buffer = Vec::new();
        b.iter(|| {
            let _ = black_box(decode_into(&encoded_base16, false, &mut buffer).unwrap());
            black_box(&buffer);
        });
    });

    group.bench_function("base32_lower", |b| {
        let mut buffer = Vec::new();
        b.iter(|| {
            let _ = black_box(decode_into(&encoded_base32, false, &mut buffer).unwrap());
            black_box(&buffer);
        });
    });

    group.bench_function("base64", |b| {
        let mut buffer = Vec::new();
        b.iter(|| {
            let _ = black_box(decode_into(&encoded_base64, false, &mut buffer).unwrap());
            black_box(&buffer);
        });
    });

    group.finish();
}

// Benchmark roundtrip operations
fn bench_roundtrip(c: &mut Criterion) {
    let mut rng = rand::rng();
    let data: Vec<u8> = (0..256).map(|_| rng.random()).collect();

    let mut group = c.benchmark_group("roundtrip");

    for base in &[
        Base::Base16Lower,
        Base::Base32Lower,
        Base::Base58Btc,
        Base::Base64,
    ] {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{base:?}")),
            base,
            |b, &base| {
                b.iter(|| {
                    let encoded = encode(black_box(base), black_box(&data));
                    let (decoded_base, decoded_data) = decode(black_box(&encoded), false).unwrap();
                    black_box((decoded_base, decoded_data));
                });
            },
        );
    }

    group.finish();
}

// Benchmark various data sizes
fn bench_data_sizes(c: &mut Criterion) {
    let mut rng = rand::rng();
    let sizes = vec![0, 1, 16, 64, 256, 1024, 4096];

    let mut group = c.benchmark_group("data_sizes");

    for size in sizes {
        let data: Vec<u8> = (0..size).map(|_| rng.random()).collect();

        group.bench_with_input(BenchmarkId::new("base64_encode", size), &data, |b, data| {
            b.iter(|| {
                let _ = black_box(encode(Base::Base64, black_box(data)));
            });
        });
    }

    group.finish();
}

// Benchmark all base types
fn bench_all_bases(c: &mut Criterion) {
    let mut rng = rand::rng();
    let data: Vec<u8> = (0..128).map(|_| rng.random()).collect();

    let bases = vec![
        Base::Base2,
        Base::Base8,
        Base::Base10,
        Base::Base16Lower,
        Base::Base16Upper,
        Base::Base32Lower,
        Base::Base32Upper,
        Base::Base36Lower,
        Base::Base36Upper,
        Base::Base58Btc,
        Base::Base64,
        Base::Base64Url,
    ];

    let mut group = c.benchmark_group("all_bases_encode");

    for base in bases {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{base:?}")),
            &base,
            |b, &base| {
                b.iter(|| {
                    let _ = black_box(encode(black_box(base), black_box(&data)));
                });
            },
        );
    }

    group.finish();
}

// Benchmark EncodedString operations
fn bench_encoded_string(c: &mut Criterion) {
    let mut rng = rand::rng();
    let data: Vec<u8> = (0..256).map(|_| rng.random()).collect();

    let mut group = c.benchmark_group("encoded_string");

    // Benchmark encode_to_validated
    group.bench_function("encode_to_validated", |b| {
        b.iter(|| {
            let _ = black_box(encode_to_validated(Base::Base64, black_box(&data)));
        });
    });

    // Benchmark EncodedString::new (parsing)
    let encoded_str = encode(Base::Base64, &data);
    group.bench_function("parse", |b| {
        b.iter(|| {
            let _ = black_box(multi_base::parse_encoded(black_box(&encoded_str)).unwrap());
        });
    });

    // Benchmark EncodedString::decode
    let encoded = encode_to_validated(Base::Base64, &data);
    group.bench_function("decode", |b| {
        b.iter(|| {
            let _ = black_box(encoded.decode().unwrap());
        });
    });

    group.finish();
}

// Benchmark Base::from_code (frequently called)
fn bench_base_from_code(c: &mut Criterion) {
    let codes = vec!['0', '7', '9', 'f', 'F', 'b', 'B', 'z', 'm', 'u'];

    let mut group = c.benchmark_group("base_from_code");

    for code in codes {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("'{code}'")),
            &code,
            |b, &code| {
                b.iter(|| {
                    let _ = black_box(Base::from_code(black_box(code)).unwrap());
                });
            },
        );
    }

    group.finish();
}

// Benchmark comparison: encode vs encode_into
fn bench_encode_comparison(c: &mut Criterion) {
    let mut rng = rand::rng();
    let data: Vec<u8> = (0..1024).map(|_| rng.random()).collect();

    let mut group = c.benchmark_group("encode_comparison");

    group.bench_function("encode", |b| {
        b.iter(|| {
            let _ = black_box(encode(Base::Base64, black_box(&data)));
        });
    });

    group.bench_function("encode_into", |b| {
        let mut buffer = String::new();
        b.iter(|| {
            encode_into(Base::Base64, black_box(&data), &mut buffer);
            black_box(&buffer);
        });
    });

    // Simulate real-world scenario: encoding multiple values in a loop
    group.bench_function("encode_loop_10", |b| {
        b.iter(|| {
            for _ in 0..10 {
                let _ = black_box(encode(Base::Base64, black_box(&data)));
            }
        });
    });

    group.bench_function("encode_into_loop_10", |b| {
        let mut buffer = String::new();
        b.iter(|| {
            for _ in 0..10 {
                encode_into(Base::Base64, black_box(&data), &mut buffer);
                black_box(&buffer);
            }
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_encode,
    bench_decode,
    bench_encode_into,
    bench_decode_into,
    bench_roundtrip,
    bench_data_sizes,
    bench_all_bases,
    bench_encoded_string,
    bench_base_from_code,
    bench_encode_comparison
);
criterion_main!(benches);
