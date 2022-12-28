pub struct Parser {
    src: String,
    pub pos: usize,
}

impl Parser {
    pub fn new(src: &str) -> Self {
        Self {
            src: src.into(),
            pos: 0,
        }
    }

    pub fn trim(&mut self) {
        while self.peek().is_whitespace() {
            self.pos += 1;
        }
    }

    pub fn peek(&self) -> char {
        self.src.chars().nth(self.pos).unwrap_or('\0')
    }

    pub fn is_end(&self) -> bool {
        self.pos >= self.src.len()
    }

    pub fn next(&mut self) -> char {
        self.trim();
        let c = self.peek();

        self.pos += 1;
        c
    }
}
