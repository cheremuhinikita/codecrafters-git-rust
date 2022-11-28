#[derive(Debug, PartialEq, Eq)]
pub struct RawObject {
    pub kind: String,
    pub size: usize,
    pub content: Vec<u8>,
}

impl RawObject {
    pub fn new(kind: impl ToString, content: &[u8]) -> Self {
        Self {
            kind: kind.to_string(),
            size: content.len(),
            content: content.to_vec(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::<u8>::new();

        result.extend_from_slice(self.kind.as_bytes());
        result.push(b' ');
        result.extend_from_slice(self.size.to_string().as_bytes());
        result.push(b'\0');
        result.extend_from_slice(&self.content);

        result
    }
}
