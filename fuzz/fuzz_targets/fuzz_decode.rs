#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Convert bytes to string (may contain invalid UTF-8)
    if let Ok(s) = std::str::from_utf8(data) {
        // Try decoding with strict mode
        let _ = multi_base::decode(s, true);

        // Try decoding with permissive mode
        let _ = multi_base::decode(s, false);

        // Try parsing as EncodedString
        let _ = multi_base::parse_encoded(s);

        // Try decode_into with a buffer
        let mut buffer = Vec::new();
        let _ = multi_base::decode_into(s, true, &mut buffer);
    }

    // The goal is to ensure no panics occur on arbitrary input
});
