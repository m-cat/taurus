#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate taurus;

use taurus::util::convert::*;
use taurus::lang::*;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        for ch in s.chars() {
            let _ = hex_char_to_int(ch);
        }
        let _ = hex_str_to_int(s);
        let _ = color_code_to_rgb(s);
        let _ = cap(s);
    }
});
