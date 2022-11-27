use super::Object;

pub fn encode(object: &Object) -> Vec<u8> {
    object.to_raw().to_bytes()
}
