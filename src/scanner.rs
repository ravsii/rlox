pub struct Scanner {
    source: String,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner { source }
    }

    pub fn scan_tokens(&self) -> Vec<&str> {
        self.source.split_whitespace().collect()
    }
}
