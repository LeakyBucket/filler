use std::fmt;

#[derive(Debug)]
pub struct Placeholder {
    pub source: String,
    pub label: String,
    pub version: Option<String>
}

impl fmt::Display for Placeholder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.version {
            Some(ver) => write!(f, "{}:{}:{}", self.source, self.label, ver),
            None => write!(f, "{}:{}", self.source, self.label)
        }
    }
}