#![no_main]

use libfuzzer_sys::fuzz_target;
use multi_base::Base;

fuzz_target!(|data: &[u8]| {
    // Test round-trip property: decode(encode(x)) should equal x (for valid UTF-8 in Identity)
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

    // Limit data size for performance (Base58 is particularly slow)
    let limited_data = if data.len() > 500 {
        &data[..500]
    } else {
        data
    };

    for base in &bases {
        // Encode the data
        let encoded = multi_base::encode(*base, limited_data);

        // Decode it back
        if let Ok((decoded_base, decoded_data)) = multi_base::decode(&encoded, true) {
            // Base should match
            assert_eq!(decoded_base, *base);

            // Data should match (except for Identity with invalid UTF-8)
            if *base != Base::Identity || std::str::from_utf8(limited_data).is_ok() {
                assert_eq!(&decoded_data[..], limited_data);
            }
        }

        // Test with buffer reuse
        let mut encode_buffer = String::new();
        multi_base::encode_into(*base, limited_data, &mut encode_buffer);

        let mut decode_buffer = Vec::new();
        if let Ok(decoded_base) = multi_base::decode_into(&encode_buffer, true, &mut decode_buffer) {
            assert_eq!(decoded_base, *base);

            if *base != Base::Identity || std::str::from_utf8(limited_data).is_ok() {
                assert_eq!(&decode_buffer[..], limited_data);
            }
        }
    }

    // The goal is to verify round-trip consistency and catch any logic errors
});
