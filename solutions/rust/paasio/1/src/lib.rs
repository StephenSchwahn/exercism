use std::io::{Read, Result, Write};

// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

pub struct ReadStats<R> {
    data: R,
    total_bytes_read: usize,
    num_reads: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            data: wrapped,
            total_bytes_read: 0,
            num_reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.total_bytes_read
    }

    pub fn reads(&self) -> usize {
        self.num_reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let result = self.data.read(buf);
        self.num_reads += 1;
        self.total_bytes_read += result.as_ref().map_or(0, |len| *len);
        result
    }
}

pub struct WriteStats<W> {
    data: W,
    total_bytes_written: usize,
    num_writes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            data: wrapped,
            total_bytes_written: 0,
            num_writes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.total_bytes_written
    }

    pub fn writes(&self) -> usize {
        self.num_writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let write_result = self.data.write(buf);
        self.num_writes += 1;
        self.total_bytes_written += write_result.as_ref().map_or(0, |len| *len);
        write_result
    }

    fn flush(&mut self) -> Result<()> {
        self.data.flush()
    }
}
