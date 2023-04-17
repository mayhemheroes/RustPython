#![no_main]

use libfuzzer_sys::fuzz_target;
use rustpython_parser::parse_program;

// use arbitrary to create two strings from the fuzzed data
use libfuzzer_sys::arbitrary::Arbitrary;
use libfuzzer_sys::arbitrary::Unstructured;

fuzz_target!(|data: &[u8]| {
    // generate unstructured
    let mut unstructured = Unstructured::new(data);

    // generate strings
    let str_1 = String::arbitrary(&mut unstructured).unwrap_or(String::new());
    let str_2 = String::arbitrary(&mut unstructured).unwrap_or(String::new());
    
    // parse program
    let _ = parse_program(&str_1, &str_2);
});