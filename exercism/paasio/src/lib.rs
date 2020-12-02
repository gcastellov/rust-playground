use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    value: R,
    reads_stats: usize,
    bytes_stats: usize
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats { 
            value: wrapped, 
            reads_stats: 0, 
            bytes_stats: 0 }
    }

    pub fn get_ref(&self) -> &R {
        &self.value
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_stats
    }

    pub fn reads(&self) -> usize {
        self.reads_stats
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let len = self.value.read(buf).unwrap_or_default();
        self.reads_stats += 1;
        self.bytes_stats += len;
        Ok(len)
    }
}

pub struct WriteStats<W> {
    value: W,
    writes_stats: usize,
    bytes_stats: usize
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats { 
            value: wrapped, 
            writes_stats: 0,
            bytes_stats: 0
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.value
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_stats
    }

    pub fn writes(&self) -> usize {
        self.writes_stats
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let len = self.value.write(buf).unwrap_or_default();
        self.writes_stats += 1;
        self.bytes_stats += len;
        Ok(len)
    }

    fn flush(&mut self) -> Result<()> {
        self.value.flush()
    }
}
