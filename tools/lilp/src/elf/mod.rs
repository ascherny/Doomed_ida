pub mod types;
pub mod header;

use crate::byteview::ByteView;
use crate::error::ParseError;
use crate::fileformat::FileFormat;
use crate::metadata::Metadata;
use crate::reader::Endian;
use self::header::{Class, ElfHeader};

pub struct Elf<'a> {
    view: ByteView<'a>,
    header: ElfHeader,
}

impl<'a> Elf<'a> {
    pub fn probe(view: &ByteView<'a>) -> bool {
        view.slice(0, 4)
            .map_or(false, |magic| magic == &[0x7f, b'E', b'L', b'F'])
    }

    pub fn parse(view: ByteView<'a>) -> Result<Self, ParseError> {
        let header = ElfHeader::parse(&view)?;
        Ok(Self { view, header })
    }
}

impl<'a> FileFormat for Elf<'a> {
    fn name(&self) -> &'static str {
        "ELF"
    }

    fn metadata(&self) -> Metadata {
        let h = &self.header;
        let mut meta = Metadata::new();

        meta
            .add("class",    match h.class {
                Class::Elf32 => "ELF32",
                Class::Elf64 => "ELF64",
            })
            .add("endian",   match h.endian {
                Endian::Little => "little-endian",
                Endian::Big    => "big-endian",
            })
            .add("type",     h.e_type)
            .add("machine",  h.machine)
            .add("entry",    format!("{:#018x}", h.entry))
            .add("phoff",    format!("{:#018x}", h.phoff))
            .add("shoff",    format!("{:#018x}", h.shoff))
            .add("phnum",    h.phnum)
            .add("shnum",    h.shnum);

        meta
    }
}
