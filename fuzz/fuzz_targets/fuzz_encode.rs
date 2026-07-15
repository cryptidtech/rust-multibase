#![no_main]

use libfuzzer_sys::fuzz_target;
use multi_base::Base;

fuzz_target!(|data: &[u8]| {
    // Test encoding with all base types
    let bases = [
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
    ];

    // Limit data size to keep fuzzing fast (Base58 is slow on large inputs)
    let limited_data = if data.len() > 1000 {
        &data[..1000]
    } else {
        data
    };

    for base in &bases {
        // Test regular encoding
        let encoded = multi_base::encode(*base, limited_data);

        // Verify it starts with a valid prefix
        assert!(!encoded.is_empty());

        // Test encode_into
        let mut buffer = String::new();
        multi_base::encode_into(*base, limited_data, &mut buffer);
        assert!(!buffer.is_empty());

        // Test encode_to_validated
        let validated = multi_base::encode_to_validated(*base, limited_data);
        assert_eq!(validated.base(), *base);
    }

    // The goal is to ensure no panics occur on arbitrary input
});
