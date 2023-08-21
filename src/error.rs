#[derive(Debug)]
pub struct LoxError {
    pub line: usize,
    pub message: String,
}

impl LoxError {
    pub fn new(line: usize, message: String) -> Self {
        Self { line, message }
    }

    pub fn report(&self) {
        eprintln!("[line {}] Error: {}", self.line, self.message);
    }
}
