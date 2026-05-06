use crate::error::ParseError;

pub struct ByteView<'a> {
    data: &'a [u8],
}

impl<'a> ByteView<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {data}
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn slice(&self, offset: usize, size: usize) -> Result<&'a [u8], ParseError> {
        self.check(offset, size)?;
        Ok(&self.data[offset..offset + size])
    }

    pub fn get_u8(&self, offset:usize) -> Result<u8, ParseError> {
        self.check(offset, 1)?;
        Ok(self.data[offset])
    }

    fn check(&self, offset: usize, size: usize) -> Result<(), ParseError> {
        if offset <=self.data.len() && size <= self.data.len() - offset {
            Ok(())
        } else {
            Err(ParseError::OutOfBounds)
        }
    }
}
