use std::io::Read;

use crate::{Error, RbrepResult};

pub trait MatchInput {
    fn read(&mut self, offset: usize) -> RbrepResult<u8>;
}

pub struct FileBufferInput<'a> {
    buffer: Vec<u8>,
    read: &'a mut dyn Read,
}

impl<'a> FileBufferInput<'a> {
    pub fn new(read: &'a mut dyn Read) -> Self {
        Self {
            buffer: vec![],
            read,
        }
    }

    pub fn remove(&mut self, offset: usize) {
        self.buffer.remove(offset);
    }

    pub fn read_next(&mut self) -> RbrepResult<usize> {
        let mut next = [0; 1];
        // read a new byte
        let res = self.read.read_exact(&mut next);
        // same here, if it is eof
        // we have simply reached the onf of the file!
        match res {
            Err(ref e) if e.kind() == std::io::ErrorKind::UnexpectedEof => Ok(0),
            Err(_) => Err(Error::Io),
            _ => {
                self.buffer.push(next[0]);
                Ok(1)
            }
        }
    }
}

impl<'a> MatchInput for FileBufferInput<'a> {
    fn read(&mut self, offset: usize) -> RbrepResult<u8> {
        let mut next = [0; 1];
        if let Some(val) = self.buffer.get(offset) {
            Ok(*val)
        } else if self.read.read_exact(&mut next).is_err() {
            // FIXME this may not be correct
            Err(Error::EndOfFile)
        } else {
            self.buffer.push(next[0]);
            Ok(next[0])
        }
    }
}
