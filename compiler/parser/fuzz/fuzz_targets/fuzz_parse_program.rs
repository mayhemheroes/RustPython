#![no_main]
struct FuzzedDataProvider {
    data: Vec<u8>,
    index: usize,
}

impl FuzzedDataProvider {
    fn new(data: &[u8]) -> Self {
        Self {
            data: data.to_vec(),
            index: 0,
        }
    }

    fn remaining_len(&self) -> usize {
        self.data.len() - self.index
    }

    fn consume_i32(&mut self) -> Option<i32> {
        if self.remaining_len() < 4 {
            return None;
        }

        let mut buf = [0u8; 4];
        buf.copy_from_slice(&self.data[self.index..self.index + 4]);
        self.index += 4;
        Some(i32::from_le_bytes(buf))
    }

    fn consume_string(&mut self, len: usize) -> Option<String> {
        if self.remaining_len() < len {
            return None;
        }

        let s = String::from_utf8(self.data[self.index..self.index + len].to_vec()).ok()?;
        self.index += len;
        Some(s)
    }
}

use libfuzzer_sys::fuzz_target;
use rustpython_parser::parse_program;

fuzz_target!(|data: &[u8]| {
    let mut data_provider = FuzzedDataProvider::new(data);

    let str_1_len = data_provider.consume_i32().unwrap_or(0) as usize;
    let str_2_len = data_provider.consume_i32().unwrap_or(0) as usize;

    let str_1 = data_provider.consume_string(str_1_len).unwrap_or(String::new());
    let str_2 = data_provider.consume_string(str_2_len).unwrap_or(String::new());

    if !str_1.is_empty() {
        println!("str_1: {}", str_1);
    }

    if !str_2.is_empty() {
        println!("str_2: {}", str_2);
    }

    let _ = parse_program(&str_1, &str_2);
});