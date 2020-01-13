#[derive(Debug, Clone, PartialEq)]
pub struct ParserError {
    remaining: String,
    reason: Option<String>,
}
impl ParserError {
    pub fn new(remaining: String) -> Self {
        Self {
            remaining,
            reason: None,
        }
    }
    pub fn newr(remaining: String, reason: String) -> Self {
        Self {
            remaining,
            reason: Some(reason),
        }
    }
    pub fn remaining(&self) -> String {
        self.remaining.clone()
    }
}
