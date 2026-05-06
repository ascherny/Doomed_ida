use crate::byteview::ByteView;
use crate::error::ParseError;

#[derive(Debug, Copy, Clone)]
pub enum Endian {
    Little,
    Big,
}

pub struct BinaryReader<'a> {
    view: &'a ByteView<'a>,
    endian: Endian,
}

impl<'a> BinaryReader<'a> {
    pub fn new(view: &'a ByteView<'a>, endian: Endian) -> Self {
        Self { view, endian }
    }

    pub fn read_u8(&self, offset: usize) -> Result<u8, ParseError> {
        self.view.get_u8(offset)
    }

    pub fn read_u16(&self, offset: usize) -> Result<u16, ParseError> {
        let b = self.view.slice(offset, 2)?;
        Ok(match self.endian {
            Endian::Little => (b[0] as u16) | ((b[1] as u16) << 8),
            Endian::Big => ((b[0] as u16) << 8) | (b[1] as u16),
        })
    }

    pub fn read_u32(&self, offset: usize) -> Result<u32, ParseError> {
        let b = self.view.slice(offset, 4)?;
        Ok(match self.endian {
            Endian::Little => (b[0] as u32) | ((b[1] as u32) << 8)  |
                              ((b[2] as u32) << 16) | ((b[3] as u32) << 24),
            Endian::Big => ((b[0] as u32) << 24) | ((b[1] as u32) << 16) |
                              ((b[2] as u32) << 8)  | (b[3] as u32),
        })
    }

    pub fn read_u64(&self, offset: usize) -> Result<u64, ParseError> {
        let b = self.view.slice(offset, 8)?;
        Ok(match self.endian {
            Endian::Little => (0..8).fold(0u64, |acc, i| acc | ((b[i] as u64) << (i * 8))),
            Endian::Big => (0..8).fold(0u64, |acc, i| acc | ((b[i] as u64) << ((7 - i) * 8))),
        })
    }
}
