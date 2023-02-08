pub trait MatchOutput: Clone + Default {
    // add new byte to output
    fn push(&mut self, out: ExprOutData);

    // how many bytes were already written
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn as_slice(&self) -> &[ExprOutData];
}

#[derive(Clone, Default, Debug)]
pub struct ExprOutData {
    pub highlight: bool,
    pub value: u8,
}

impl ExprOutData {
    pub fn new(value: u8, highlight: bool) -> Self {
        Self { highlight, value }
    }
}

#[derive(Default, Clone)]
pub struct ExprOutput {
    data: Vec<ExprOutData>,
}

impl MatchOutput for ExprOutput {
    fn push(&mut self, out: ExprOutData) {
        self.data.push(out);
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn as_slice(&self) -> &[ExprOutData] {
        self.data.as_slice()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
