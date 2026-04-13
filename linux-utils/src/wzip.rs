use std::{cmp, io::Read, mem::MaybeUninit};

pub fn zip(file_path: &std::path::Path) -> anyhow::Result<()> {
    let file = std::fs::File::open(file_path)?;
    // file.read_exact(buf);
    Ok(())
}

pub struct Buffer {
    buf: Box<[MaybeUninit<u8>]>,
    pos: usize,
    filled: usize,
    initialized: usize,
}

impl Buffer {
    pub fn with_cap(cap: usize) -> Self {
        Self {
            buf: Box::new_uninit_slice(cap),
            pos: 0,
            filled: 0,
            initialized: 0,
        }
    }

    #[inline]
    pub fn buffer(&self) -> &[u8] {
        unsafe {
            self.buf
                .get_unchecked(self.pos..self.filled)
                .assume_init_ref()
        }
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    #[inline]
    pub fn discard_buffer(&mut self) {
        self.filled = 0;
        self.pos = 0;
    }

    #[inline]
    pub fn consume(&mut self, amt: usize) {
        self.pos = cmp::min(self.pos + amt, self.filled);
    }

    #[inline]
    pub fn unconsume(&mut self, amt: usize) {
        self.pos = self.pos.saturating_sub(amt);
    }

    #[inline]
    pub fn fill_buf(&mut self, mut reader: impl Read) -> std::io::Result<&[u8]> {
        if self.pos >= self.filled {
            debug_assert!(self.pos == self.filled);
            let mut buf = std::io::BorrowedBuf::from(&mut *self.buf);
            unsafe {
                buf.set_init(self.initialized);
            }
            let result = reader.read_buf(buf.unfilled());

            self.pos = 0;
            self.filled = buf.len();
            self.initialized = buf.init_len();

            result?;
        }

        Ok(self.buffer())
    }
}

pub struct BufReader {
    file: std::fs::File,
    buffer: Buffer,
}

impl BufReader {}
