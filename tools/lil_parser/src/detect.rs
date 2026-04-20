pub enum FileType {
    MachO(String),
    ELF,
    PE,
    Another,
}

pub fn detect_type(buf: &[u8]) -> FileType {
    if buf.len() < 4 {
        return FileType::Another;
    }

    match &buf[0..4] {
        [0xCF, 0xFA, 0xED, 0xFE] => FileType::MachO("x86_64 / arm64 (MH_MAGIC_64 LE)".to_string()),
        [0xFE, 0xED, 0xFA, 0xCF] => FileType::MachO("x86_64 / arm64 (MH_CIGAM_64 BE)".to_string()),
        [0xCE, 0xFA, 0xED, 0xFE] => FileType::MachO("32-bit (MH_MAGIC LE)".to_string()),
        [0xFE, 0xED, 0xFA, 0xCE] => FileType::MachO("32-bit (MH_CIGAM BE)".to_string()),
        [0xCA, 0xFE, 0xBA, 0xBE] => FileType::MachO("FAT/Universal (MH_MAGIC_FAT)".to_string()),
        [0x7F, b'E', b'L', b'F'] => FileType::ELF,
        [b'M', b'Z', ..] => {
            if buf.len() > 0x3C + 4 && &buf[0x3C..0x40] == b"PE\0\0" {
                FileType::PE
            } else {
                FileType::Another
            }
        }
        _ => FileType::Another,
    }
}
