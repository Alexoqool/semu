use crate::utils::{read_bit, read_u16, read_u32};

pub struct HeaderFlags {
    pub executable_type: bool,
    pub call_entry_point: bool,
    pub fixed_address: bool,
    pub abi: u32,
    pub entry_point_type: u32,
    pub header_format: u32,
    pub import_format: u32,
}

pub struct ModuleVersion {
    pub major: u16,
    pub minor: u16,
}

pub struct ToolVersion {
    pub major: u8,
    pub minor: u8,
    pub build: u16,
}

pub struct SecurityInfo {
    pub secure_id: u32,
    pub vendor_id: u32,
    pub capabilities: [u32; 2],
}

pub struct E32ImageHeader {
    pub uid1: u32,
    pub uid2: u32,
    pub uid3: u32,
    pub uid_checksum: u32,
    pub signature: u32,
    pub header_crc: u32,
    pub module_version: ModuleVersion,
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

pub struct E32ImageHeaderJ {
    pub header: E32ImageHeader,
    pub uncompressed_size: u32,
}

pub struct E32ImageHeaderV {
    pub header_j: E32ImageHeaderJ,
    pub security_info: SecurityInfo,
    pub exception_descriptor: u32,
    pub spare2: u32,
    pub export_desc_size: u16,
    pub export_desc_type: u8,
    pub export_desc: Vec<u8>,
}

impl HeaderFlags {
    pub fn parse(bytes: [u8; 4]) -> Self {
        let flags = u32::from_le_bytes(bytes);
        Self {
            executable_type: read_bit(bytes[0], 0),
            call_entry_point: read_bit(bytes[0], 1),
            fixed_address: read_bit(bytes[0], 2),
            abi: (flags >> 3) & 0x3,
            entry_point_type: (flags >> 5) & 7,
            header_format: (flags >> 24) & 0xF,
            import_format: (flags >> 28) & 0xF,
        }
    }
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
            module_version: ModuleVersion {
                major: read_u16(data, 24),
                minor: read_u16(data, 26),
            },
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

impl E32ImageHeaderJ {
    pub fn parse(base_header: E32ImageHeader, data: &Vec<u8>) -> Self {
        Self {
            header: base_header,
            uncompressed_size: read_u32(data, 124),
        }
    }
}

impl E32ImageHeaderV {
    pub fn parse(base_header: E32ImageHeaderJ, data: &Vec<u8>) -> Self {
        Self {
            header_j: base_header,
            security_info: SecurityInfo {
                secure_id: read_u32(data, 128),
                vendor_id: read_u32(data, 132),
                capabilities: [read_u32(data, 136), read_u32(data, 140)],
            },
            exception_descriptor: read_u32(data, 144),
            spare2: read_u32(data, 148),
            export_desc_size: read_u16(data, 152),
            export_desc_type: data[154],
            // uint8_t  iExportDesc[1];
            // description of holes in export table - extend
            export_desc: vec![data[155]],
        }
    }
}
