use std::io::Read;

use crate::{Error, RbrepResult};

pub trait MatchInput {
    fn read(&mut self, offset: usize) -> RbrepResult<u8>;

    // called to trim a possible buffer by n bytes
    // and re-read if required
    fn trim(&mut self, _by: usize) -> RbrepResult<()> {
        Ok(())
    }

    fn eof(&self) -> bool;
}

pub struct FileBufferInput<'a> {
    buffer: Vec<u8>,
    read: &'a mut dyn Read,
    eof: bool,
}

impl<'a> FileBufferInput<'a> {
    pub fn new(read: &'a mut dyn Read) -> Self {
        Self {
            buffer: vec![],
            read,
            eof: false,
        }
    }

    fn remove(&mut self, offset: usize) {
        if self.buffer.len() > offset {
            self.buffer.remove(offset);
        }
    }

    fn read_next(&mut self) -> RbrepResult<usize> {
        let mut next = [0; 1];
        // read a new byte
        let res = self.read.read_exact(&mut next);
        // same here, if it is eof
        // we have simply reached the onf of the file!
        match res {
            Err(ref e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                self.eof = true;
                Ok(0)
            }
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
            self.eof = true;
            Err(Error::EndOfFile)
        } else {
            self.buffer.push(next[0]);
            Ok(next[0])
        }
    }

    fn trim(&mut self, by: usize) -> RbrepResult<()> {
        for _ in 0..by {
            self.remove(0);
            self.read_next()?;
        }
        Ok(())
    }

    fn eof(&self) -> bool {
        self.eof
    }
}
