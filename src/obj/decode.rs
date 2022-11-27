use std::str;

use super::{parser::parse_raw_object, Object};
use crate::{error::Error, Result};

pub fn decode(input: &[u8]) -> Result<Object> {
    let input = str::from_utf8(input)?;
    let (input, raw) = parse_raw_object(input).map_err(|e| Error::ParseObject(e.to_string()))?;

    if !input.is_empty() {
        Err(Error::ParseObject(String::from("input not complete")))
    } else {
        Object::from_raw(raw)
    }
}
