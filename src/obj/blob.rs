pub struct Blob(pub Vec<u8>);

impl Blob {
    pub fn parse(input: &str) -> Self {
        Self(input.as_bytes().to_vec())
    }
}
