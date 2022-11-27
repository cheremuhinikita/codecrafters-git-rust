#[derive(Debug, PartialEq)]
pub struct RawObject {
    pub kind: String,
    pub size: usize,
    pub content: String,
}

impl RawObject {
    pub fn new(kind: impl ToString, content: impl ToString) -> Self {
        let content = content.to_string();

        Self {
            kind: kind.to_string(),
            size: content.len(),
            content,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::<u8>::new();

        result.extend_from_slice(self.kind.as_bytes());
        result.push(b' ');
        result.extend_from_slice(self.size.to_string().as_bytes());
        result.push(b'\0');
        result.extend_from_slice(self.content.as_bytes());

        result
    }
}
