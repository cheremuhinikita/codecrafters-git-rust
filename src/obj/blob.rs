pub struct Blob(pub Vec<u8>);

impl Blob {
    pub fn new(bytes: &[u8]) -> Self {
        Self(bytes.to_vec())
    }

    pub fn parse(input: &str) -> Self {
        Self::new(input.as_bytes())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }
}
