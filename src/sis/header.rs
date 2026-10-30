// TODO: support for Symbian 9.1+ SIS format

use crate::utils::{read_u16, read_u32};

pub struct SISHeader {
    pub uid1: u32,
    pub uid2: u32,
    pub uid3: u32,
    pub uid_checksum: u32,
    pub data_checksum: u16,
    pub langs_count: u16,
    pub files_count: u16,
    pub requisites_count: u16,
    pub install_lang: u16,
    pub install_files: u16,
    pub install_drive: u16,
    pub capabilities_count: u16,
    pub installer_ver: u32,
    pub options: u16,
    pub sis_type: u16,
    pub major_ver: u16,
    pub minor_ver: u16,
    pub variant: u32,
    pub langs_offset: u32,
    pub files_offset: u32,
    pub requisites_offset: u32,
    pub certificates_offset: u32,
    pub name_offset: u32,
}

pub struct SISHeader6 {
    header: SISHeader,
    pub signature_offset: u32,
    pub capabilities_offset: u32,
    pub installed_space: u32,
    pub max_installed_space: u32,
}

impl SISHeader {
    pub fn parse(data: &Vec<u8>) -> Self {
        Self {
            uid1: read_u32(data, 0),
            uid2: read_u32(data, 4),
            uid3: read_u32(data, 8),
            uid_checksum: read_u32(data, 12),
            data_checksum: read_u16(data, 16),
            langs_count: read_u16(data, 18),
            files_count: read_u16(data, 20),
            requisites_count: read_u16(data, 22),
            install_lang: read_u16(data, 24),
            install_files: read_u16(data, 26),
            install_drive: read_u16(data, 28),
            capabilities_count: read_u16(data, 30),
            installer_ver: read_u32(data, 32),
            options: read_u16(data, 36),
            sis_type: read_u16(data, 38),
            major_ver: read_u16(data, 40),
            minor_ver: read_u16(data, 42),
            variant: read_u32(data, 44),
            langs_offset: read_u32(data, 48),
            files_offset: read_u32(data, 52),
            requisites_offset: read_u32(data, 56),
            certificates_offset: read_u32(data, 60),
            name_offset: read_u32(data, 64),
        }
    }
}

impl SISHeader6 {
    pub fn parse(data: &Vec<u8>) -> Self {
        Self {
            header: SISHeader::parse(data),
            signature_offset: read_u32(data, 68),
            capabilities_offset: read_u32(data, 72),
            installed_space: read_u32(data, 76),
            max_installed_space: read_u32(data, 80),
        }
    }
}
