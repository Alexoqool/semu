use crate::utils::{read_u16, read_u32};

fn get_compression_string(data: u32) -> String {
    match data {
        0 => String::from("none"),
        0x101F7AFC => String::from("DEFLATE"),
        0x102822AA => String::from("byte-pair"),
        _ => String::from("unknown"),
    }
}

fn get_cpu_string(data: u16) -> String {
    match data {
        0x1000 => String::from("X86"),
        0x2000 => String::from("ARMv4"),
        0x2001 => String::from("ARMv5"),
        0x2002 => String::from("ARMv6"),
        0x4000 => String::from("MCore"),
        _ => String::from("unknown"),
    }
}

pub struct HeaderFlags {
    pub executable_type: bool,
    pub call_entry_point: bool,
    pub fixed_address: bool,
    pub abi: bool,
    pub entry_point_type: bool,
    pub header_format: u32,
    pub import_format: u32,
}

#[allow(dead_code)]
pub struct V1Header {
    pub uid1: u32,
    pub uid2: u32,
    pub uid3: u32,
    pub uid_checksum: u32,
    pub signature: u32,
    pub cpu: u32,
    pub code_checksum: u16,
    pub data_checksun: u16,
    pub tools_major_ver: u8,
    pub tools_minor_ver: u8,
    pub tools_build: u16,
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
}

#[allow(dead_code)]
pub struct V2Header {
    pub uid1: u32,
    pub uid2: u32,
    pub uid3: u32,
    pub uid_checksum: u32,
    pub signature: u32,
    pub header_crc: u32,
    pub major_ver: u16,
    pub minor_ver: u16,
    pub compression_type: u32,
    pub tools_major_ver: u8,
    pub tools_minor_ver: u8,
    pub tools_build: u16,
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

#[allow(dead_code)]
pub struct V2HeaderJ {
    pub header: V2Header,
    pub uncompressed_size: u32,
}

#[allow(dead_code)]
pub struct V2HeaderV {
    pub header_j: V2HeaderJ,
    pub secure_id: u32,
    pub vendor_id: u32,
    pub capabilities: [u32; 2],
    pub exception_descriptor: u32,
    pub spare2: u32,
    pub export_desc_size: u16,
    pub export_desc_type: u8,
    pub export_desc: Vec<u8>,
}

impl HeaderFlags {
    pub fn parse(flags: u32) -> Self {
        Self {
            executable_type: (flags & 1) == 0,
            call_entry_point: ((flags >> 1) & 1) == 0,
            fixed_address: ((flags >> 2) & 1) == 0,
            abi: ((flags >> 3) & 3) == 0,
            entry_point_type: ((flags >> 5) & 7) == 0,
            header_format: (flags >> 24) & 0xF,
            import_format: (flags >> 28) & 0xF,
        }
    }
}

impl V1Header {
    pub fn parse(data: &Vec<u8>) -> Self {
        Self {
            uid1: read_u32(data, 0),
            uid2: read_u32(data, 4),
            uid3: read_u32(data, 8),
            uid_checksum: read_u32(data, 12),
            signature: read_u32(data, 16),
            cpu: read_u32(data, 20),
            code_checksum: read_u16(data, 24),
            data_checksun: read_u16(data, 26),
            tools_major_ver: data[28],
            tools_minor_ver: data[29],
            tools_build: read_u16(data, 30),
            time_lo: read_u32(data, 32),
            time_hi: read_u32(data, 36),
            flags: read_u32(data, 40),
            code_size: read_u32(data, 44),
            data_size: read_u32(data, 48),
            heap_size_min: read_u32(data, 52),
            heap_size_max: read_u32(data, 56),
            stack_size: read_u32(data, 60),
            bss_size: read_u32(data, 64),
            entry_point: read_u32(data, 68),
            code_base: read_u32(data, 72),
            data_base: read_u32(data, 76),
            dll_ref_table_count: read_u32(data, 80),
            export_dir_offset: read_u32(data, 84),
            export_dir_count: read_u32(data, 88),
            text_size: read_u32(data, 92),
            code_offset: read_u32(data, 96),
            data_offset: read_u32(data, 100),
            import_offset: read_u32(data, 104),
            code_reloc_offset: read_u32(data, 108),
            data_reloc_offset: read_u32(data, 112),
            priority: read_u16(data, 116),
        }
    }

    pub fn print_info(header: &Self, flags_parsed: &HeaderFlags) {
        println!(
            "Type: {} {}",
            match flags_parsed.executable_type {
                true => "EXE",
                false => "DLL",
            },
            get_cpu_string(header.cpu.try_into().unwrap())
        );

        println!(
            "Call entry point: {}",
            match flags_parsed.call_entry_point {
                true => "yes",
                false => "no",
            }
        );

        println!(
            "Fixed address: {}",
            match flags_parsed.fixed_address {
                true => "no",
                false => "yes",
            }
        );

        println!(
            "ABI: {}",
            match flags_parsed.abi {
                true => "GCC98r2",
                false => "EABI",
            }
        );

        println!(
            "Entry point type: {}",
            match flags_parsed.entry_point_type {
                true => "EKA1",
                false => "EKA2",
            }
        );

        println!(
            "Header format: {}",
            match flags_parsed.header_format {
                0 => "basic",
                1 => "J",
                2 => "V",
                _ => "unknown",
            }
        );

        println!(
            "Import format: {}",
            match flags_parsed.import_format {
                0 => "standard PE",
                1 => "ELF",
                2 => "PE without redunancy",
                _ => "unknown",
            }
        );
    }
}

impl V2Header {
    pub fn parse(data: &Vec<u8>) -> Self {
        Self {
            uid1: read_u32(data, 0),
            uid2: read_u32(data, 4),
            uid3: read_u32(data, 8),
            uid_checksum: read_u32(data, 12),
            signature: read_u32(data, 16),
            header_crc: read_u32(data, 20),
            major_ver: read_u16(data, 24),
            minor_ver: read_u16(data, 26),
            compression_type: read_u32(data, 28),
            tools_major_ver: data[32],
            tools_minor_ver: data[33],
            tools_build: read_u16(data, 34),
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

    pub fn print_info(header: &Self, flags_parsed: &HeaderFlags) {
        println!(
            "Type: {} {}",
            match flags_parsed.executable_type {
                true => "EXE",
                false => "DLL",
            },
            match header.cpu {
                0x1000 => "X86",
                0x2000 => "ARMv4",
                0x2001 => "ARMv5",
                0x2002 => "ARMv6",
                0x4000 => "MCore",
                _ => "unknown",
            }
        );

        println!(
            "Call entry point: {}",
            match flags_parsed.call_entry_point {
                true => "yes",
                false => "no",
            }
        );

        println!(
            "Fixed address: {}",
            match flags_parsed.fixed_address {
                true => "no",
                false => "yes",
            }
        );

        println!(
            "ABI: {}",
            match flags_parsed.abi {
                true => "GCC98r2",
                false => "EABI",
            }
        );

        println!(
            "Entry point type: {}",
            match flags_parsed.entry_point_type {
                true => "EKA1",
                false => "EKA2",
            }
        );

        println!(
            "Header format: {}",
            match flags_parsed.header_format {
                0 => "basic",
                1 => "J",
                2 => "V",
                _ => "unknown",
            }
        );

        println!(
            "Import format: {}",
            match flags_parsed.import_format {
                0 => "standard PE",
                1 => "ELF",
                2 => "PE without redunancy",
                _ => "unknown",
            }
        );

        println!(
            "Compression type: {}",
            get_compression_string(header.compression_type)
        );
    }
}

impl V2HeaderJ {
    pub fn parse(data: &Vec<u8>) -> Self {
        Self {
            header: V2Header::parse(data),
            uncompressed_size: read_u32(data, 124),
        }
    }

    pub fn print_info(header: &Self) {
        println!("Uncompressed size: {}", header.uncompressed_size);
    }
}

impl V2HeaderV {
    pub fn parse(data: &Vec<u8>) -> Self {
        Self {
            header_j: V2HeaderJ::parse(data),
            secure_id: read_u32(data, 128),
            vendor_id: read_u32(data, 132),
            capabilities: [read_u32(data, 136), read_u32(data, 140)],
            exception_descriptor: read_u32(data, 144),
            spare2: read_u32(data, 148),
            export_desc_size: read_u16(data, 152),
            export_desc_type: data[154],
            export_desc: vec![data[155]],
        }
    }
}
