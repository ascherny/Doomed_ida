use crate::byteview::ByteView;
use crate::error::ParseError;
use crate::reader::{BinaryReader, Endian};
use crate::elf::types::{Machine, Type};

const ELFMAG:      [u8; 4] = [0x7f, b'E', b'L', b'F'];
const ELFCLASS32:  u8 = 1;
const ELFCLASS64:  u8 = 2;
const ELFDATALSB: u8 = 1;
const ELFDATAMSB: u8 = 2;

#[derive(Debug, Clone, Copy)]
pub enum Class {
    Elf32,
    Elf64,
}

#[derive(Debug)]
pub struct ElfHeader {
    pub class:   Class,
    pub endian:  Endian,
    pub e_type:  Type,
    pub machine: Machine,
    pub entry:   u64,
    pub phoff:   u64,
    pub shoff:   u64,
    pub phnum:   u16,
    pub shnum:   u16,
}

impl ElfHeader {
    pub fn parse(view: &ByteView<'_>) -> Result<Self, ParseError> {
        let class = match view.get_u8(4)? {
            ELFCLASS32 => Class::Elf32,
            ELFCLASS64 => Class::Elf64,
            _ => return Err(ParseError::Unsupported),
        };

        let endian = match view.get_u8(5)? {
            ELFDATALSB => Endian::Little,
            ELFDATAMSB => Endian::Big,
            _ => return Err(ParseError::Unsupported),
        };

        let r = BinaryReader::new(view, endian);

        let e_type  = Type::from(r.read_u16(0x10)?);
        let machine = Machine::from(r.read_u16(0x12)?);

        let (entry, phoff, shoff) = match class {
            Class::Elf32 => (
                r.read_u32(0x18)? as u64,
                r.read_u32(0x1c)? as u64,
                r.read_u32(0x20)? as u64,
            ),
            Class::Elf64 => (
                r.read_u64(0x18)?,
                r.read_u64(0x20)?,
                r.read_u64(0x28)?,
            ),
        };

        let (phnum, shnum) = match class {
            Class::Elf32 => (
                r.read_u16(0x2c)?,
                r.read_u16(0x30)?,
            ),
            Class::Elf64 => (
                r.read_u16(0x38)?,
                r.read_u16(0x3c)?,
            ),
        };

        Ok(Self {
            class,
            endian,
            e_type,
            machine,
            entry,
            phoff,
            shoff,
            phnum,
            shnum,
        })
    }

    pub fn probe(view: &ByteView<'_>) -> bool {
        view.slice(0, 4).map_or(false, |magic| magic == &ELFMAG)
    }
}
