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

    pub fn next_if(&mut self, expected: char) -> bool {
        if self.peek() == expected {
            self.next();
            true
        } else {
            false
        }
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

    // get a slice of the input stream starting at pos
    // until the condition in f is false
    pub fn until(&mut self, f: fn(x: char) -> bool) -> &str {
        let from = self.pos;

        loop {
            if self.is_end() || !f(self.peek()) {
                break;
            } else {
                self.next();
            }
        }
        let to = self.pos;
        &self.src[from..to]
    }
}
