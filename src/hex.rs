use std::{fmt::Write, num::ParseIntError};

pub fn decode(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn encode(bytes: &[u8]) -> String {
    let mut string = String::with_capacity(bytes.len() * 2);

    for &b in bytes {
        write!(&mut string, "{:02x}", b).unwrap();
    }

    string
}
