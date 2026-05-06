use crate::byteview::ByteView;
use crate::error::ParseError;
use crate::fileformat::FileFormat;
use crate::elf::Elf;

pub enum Binary<'a> {
    Elf(Elf<'a>),
    // Pe(Pe<'a>),
}

impl<'a> Binary<'a> {
    pub fn parse(view: ByteView<'a>) -> Result<Self, ParseError> {
        if Elf::probe(&view) {
            return Ok(Binary::Elf(Elf::parse(view)?));
        }
        // if Pe::probe(&view) { ... }

        Err(ParseError::Unsupported)
    }

    pub fn as_format(&self) -> &dyn FileFormat {
        match self {
            Binary::Elf(e) => e,
            // Binary::Pe(p) => p,
        }
    }
}
