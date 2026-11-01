// https://www.thouky.co.uk/software/psifs/sis.html
// TODO: support for Symbian 9.1+ SIS format

use super::common::get_name_string;
use super::image::SISImage;
use crate::utils::{read_u16, read_u32};

fn get_installer_ver_string(image: &SISImage) -> String {
    match image.header.installer_ver {
        200 => String::from("Symbian 6"),
        _ => String::from("unknown"),
    }
}

fn get_sis_type_string(image: &SISImage) -> String {
    match image.header.sis_type {
        0 => String::from("SISAPP"),
        1 => String::from("SISSYSTEM"),
        2 => String::from("SISOPTION"),
        3 => String::from("SISCONFIG"),
        4 => String::from("SISPATCH"),
        5 => String::from("SISUPGRADE"),
        _ => String::from("unknown"),
    }
}

pub struct HeaderOptions {
    pub is_unicode: bool,
    pub is_distributable: bool,
    pub no_compress: bool,
    pub shutdown_apps: bool,
}

#[allow(dead_code)]
pub struct SISHeader {
    pub uid1: u32,
    pub uid2: u32,
    pub uid3: u32,
    pub uid_checksum: u32,
    pub data_checksum: u16,
    pub langs_count: u16,
    pub files_count: u16,
    pub reqs_count: u16,
    pub inst_lang: u16,
    pub inst_files: u16,
    pub inst_drive: u16,
    pub caps_count: u16,
    pub installer_ver: u32,
    pub options: u16,
    pub sis_type: u16,
    pub major_ver: u16,
    pub minor_ver: u16,
    pub variant: u32,
    pub langs_ptr: u32,
    pub files_ptr: u32,
    pub reqs_ptr: u32,
    pub certs_ptr: u32,
    pub name_ptr: u32,
    pub sig_ptr: u32,
    pub caps_ptr: u32,
    pub inst_space: u32,
    pub max_inst_space: u32,
}

impl HeaderOptions {
    pub fn parse(options: u16) -> Self {
        Self {
            is_unicode: (options & 1) == 1,
            is_distributable: (options & 2) == 1,
            no_compress: (options & 8) == 1,
            shutdown_apps: (options & 16) == 1,
        }
    }
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
            reqs_count: read_u16(data, 22),
            inst_lang: read_u16(data, 24),
            inst_files: read_u16(data, 26),
            inst_drive: read_u16(data, 28),
            caps_count: read_u16(data, 30),
            installer_ver: read_u32(data, 32),
            options: read_u16(data, 36),
            sis_type: read_u16(data, 38),
            major_ver: read_u16(data, 40),
            minor_ver: read_u16(data, 42),
            variant: read_u32(data, 44),
            langs_ptr: read_u32(data, 48),
            files_ptr: read_u32(data, 52),
            reqs_ptr: read_u32(data, 56),
            certs_ptr: read_u32(data, 60),
            name_ptr: read_u32(data, 64),
            sig_ptr: read_u32(data, 68),
            caps_ptr: read_u32(data, 72),
            inst_space: read_u32(data, 76),
            max_inst_space: read_u32(data, 80),
        }
    }

    pub fn print_info(image: &SISImage) {
        let name = get_name_string(image);
        let installer_ver = get_installer_ver_string(image);

        println!(
            "Component: {} ({}.{})",
            name, image.header.major_ver, image.header.minor_ver
        );

        println!(
            "Installer version: {} ({:#X})",
            installer_ver, image.header.installer_ver
        );

        println!("IsUnicode: {}", image.options.is_unicode);
        println!("IsDistributable: {}", image.options.is_distributable);
        println!("NoCompress: {}", image.options.no_compress);
        println!("ShutdownApps: {}", image.options.shutdown_apps);

        println!("Languages count: {}", image.header.langs_count);
        println!("Files count: {}", image.header.files_count);
        println!("Capabilities count: {}", image.header.caps_count);
        println!("Type: {}", get_sis_type_string(image));
        println!("Languages pointer: {}", image.header.langs_ptr);
        println!("Files pointer: {}", image.header.files_ptr);
        println!("Requisites pointer: {}", image.header.reqs_ptr);
        println!("Certificates pointer: {}", image.header.certs_ptr);
        println!("Component name pointer: {}", image.header.name_ptr);
        println!("Signature pointer: {}", image.header.sig_ptr);
        println!("Capabilities pointer: {}", image.header.caps_ptr);
    }
}
