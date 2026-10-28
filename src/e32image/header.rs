use crate::utils::{read_u16, read_u32};

struct ToolVersion {
    major: u8,
    minor: u8,
    build: u16,
}

struct SecurityInfo {
    secure_id: u32,
    vendor_id: u32,
    capabilities: [u32; 2],
}

pub struct E32ImageHeader {
    pub uid1: u32,
    pub uid2: u32,
    pub uid3: u32,
    pub uid_checksum: u32,
    pub signature: u32,
    pub header_crc: u32,
    pub module_version: u32,
    pub compression_type: u32,
    pub tools_version: ToolVersion,
    pub time_lo: u32,
    pub time_hi: u32,
    pub flags: u32,
    pub code_size: u32,
    pub data_size: u32,
    pub heap_size_min: u32,
    pub heap_size_max: u32,
    pub stack_size: u32,
    pub bss_size: u32,
    pub entry_point: u32,
    pub code_base: u32,
    pub data_base: u32,
    pub dll_ref_table_count: u32,
    pub export_dir_offset: u32,
    pub export_dir_count: u32,
    pub text_size: u32,
    pub code_offset: u32,
    pub data_offset: u32,
    pub import_offset: u32,
    pub code_reloc_offset: u32,
    pub data_reloc_offset: u32,
    pub priority: u16,
    pub cpu: u16,
}

// untested
pub struct E32ImageHeaderJ {
    pub basic: E32ImageHeader,
    pub uncompressex_size: u32,
}

// untested
pub struct E32ImageHeaderV {
    pub basic: E32ImageHeaderJ,
    pub security_info: SecurityInfo,
    pub exception_descriptor: u32,
    pub spare2: u32,
    pub export_desc_size: u16,
    pub export_desc_type: u8,
    pub export_desc: [u8; 1],
}

impl E32ImageHeader {
    pub fn parse(data: &Vec<u8>) -> Self {
        Self {
            uid1: read_u32(data, 0),
            uid2: read_u32(data, 4),
            uid3: read_u32(data, 8),
            uid_checksum: read_u32(data, 12),
            signature: read_u32(data, 16),
            header_crc: read_u32(data, 20),
            module_version: read_u32(data, 24),
            compression_type: read_u32(data, 28),
            tools_version: ToolVersion {
                major: data[32],
                minor: data[33],
                build: read_u16(data, 34),
            },
            time_lo: read_u32(data, 36),
            time_hi: read_u32(data, 40),
            flags: read_u32(data, 44),
            code_size: read_u32(data, 48),
            data_size: read_u32(data, 52),
            heap_size_min: read_u32(data, 56),
            heap_size_max: read_u32(data, 60),
            stack_size: read_u32(data, 64),
            bss_size: read_u32(data, 68),
            entry_point: read_u32(data, 72),
            code_base: read_u32(data, 76),
            data_base: read_u32(data, 80),
            dll_ref_table_count: read_u32(data, 84),
            export_dir_offset: read_u32(data, 88),
            export_dir_count: read_u32(data, 92),
            text_size: read_u32(data, 96),
            code_offset: read_u32(data, 100),
            data_offset: read_u32(data, 104),
            import_offset: read_u32(data, 108),
            code_reloc_offset: read_u32(data, 112),
            data_reloc_offset: read_u32(data, 116),
            priority: read_u16(data, 120),
            cpu: read_u16(data, 122),
        }
    }
}
