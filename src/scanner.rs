pub struct Scanner(String);

impl Scanner {
    pub fn new(source: impl Into<String>) -> Self {
        Self(source.into())
    }

    pub fn scan_whitespace(&self) -> Vec<&str> {
        self.0.split_whitespace().collect()
    }
}
