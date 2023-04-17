#![no_main]

use rustpython_compiler::Mode;
use rustpython_compiler::CompileOpts;
use rustpython_compiler::compile;

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // generate source
    let source = String::from_utf8_lossy(data).to_string();
    
    // parse program
    let _ = compile(&source, Mode::Eval, String::from("<fuzz>"), CompileOpts::default());
});