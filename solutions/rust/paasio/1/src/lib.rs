use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    wrapped:R,
    bytes_read:usize,
    reads:usize
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self {
            wrapped,
            bytes_read:0,
            reads:0
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_read
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = self.wrapped.read(buf)?;
        self.bytes_read += n;
        self.reads += 1;
        Ok(n)
    }
}


pub struct WriteStats<W> {
    wrapped:W,
    bytes_write:usize,
    writes:usize
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self {
            wrapped,
            bytes_write:0,
            writes:0
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_write
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let n  = self.wrapped.write(buf)?;
        self.bytes_write += n;
        self.writes += 1;
        Ok(n)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}