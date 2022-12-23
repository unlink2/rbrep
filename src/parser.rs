pub struct Parser {
    src: String,
    pos: usize,
}

impl Parser {
    pub fn new(src: &str) -> Self {
        Self {
            src: src.into(),
            pos: 0,
        }
    }
}
