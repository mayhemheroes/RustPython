#![no_main]

use libfuzzer_sys::fuzz_target;
use rustpython_parser::parse_program;

fuzz_target!(|data: &[u8]| {
    // generate source
    let source = String::from_utf8_lossy(data).to_string();
    
    // parse program
    let _ = parse_program(&source, "<fuzz>");
});