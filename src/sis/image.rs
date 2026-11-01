// https://www.thouky.co.uk/software/psifs/sis.html

use super::common::get_name_string;
use super::header::{HeaderOptions, SISHeader};
use crate::utils::{read_u16, read_u32};
use miniz_oxide::inflate::decompress_to_vec_zlib;
use std::fs::{create_dir, write};
use std::str::from_utf8;

fn get_rec_type_string(data: u32) -> String {
    match data {
        0 => String::from("Simple file line"),
        1 => String::from("Multiple language files line"),
        2 => String::from("If line"),
        3 => String::from("ElseIf line"),
        4 => String::from("Else line"),
        5 => String::from("EndIf line"),
        _ => String::from("unknown"),
    }
}

fn get_file_type_string(data: u32) -> String {
    match data {
        0 => String::from("FILE"),
        1 => String::from("FILETEXT"),
        2 => String::from("SIS component file"),
        3 => String::from("FILERUN"),
        4 => String::from("FILENULL"),
        5 => String::from("FILEMIME"),
        _ => String::from("unknown"),
    }
}

fn get_file_det_string(data: u32, ft: u32) -> String {
    match ft {
        // FILERUN
        3 => match data {
            0 => String::from("RUNINSTALL"),
            1 => String::from("RUNREMOVE"),
            2 => String::from("RUNBOTH"),
            0x100 => String::from("RUNSENDED"),
            0x200 => String::from("RUNWAIT"),
            _ => String::from("unknown"),
        },
        // FILEMIME
        5 => match data {
            256 => String::from("OPENCLOSE"),
            512 => String::from("OPENWAIT"),
            _ => String::from("unknown"),
        },
        _ => match data {
            0 => String::from("TEXTCONTINUE"),
            1 => String::from("TEXTSKIP"),
            2 => String::from("TEXTABORT"),
            3 => String::from("TEXTEXIT"),
            _ => String::from("unknown"),
        },
    }
}

pub struct NameRecord {
    pub name_lens: Vec<u32>,
    pub name_ptrs: Vec<u32>,
}

// (S)imple and (m)ulti language
#[allow(dead_code)]
pub struct SMFileRecord {
    pub rec_type: u32,
    pub file_type: u32,
    pub file_det: u32,
    pub src_len: u32,
    pub src_ptr: u32,
    pub dest_len: u32,
    pub dest_ptr: u32,
    pub file_lens: Vec<u32>,
    pub file_ptrs: Vec<u32>,
    pub orig_file_lens: Vec<u32>,
    pub mime_type_len: u32,
    pub mime_type_ptr: u32,
}

#[allow(dead_code)]
pub struct SISImage {
    pub bin_data: Vec<u8>,
    pub header: SISHeader,
    pub options: HeaderOptions,
}

impl NameRecord {
    pub fn parse(image: &SISImage) -> Self {
        let langs_count = image.header.langs_count as usize;
        let rec_start = image.header.name_ptr as usize;
        let rec_end = rec_start + 8 * langs_count;
        let name_data = image.bin_data[rec_start..rec_end].to_vec();
        let mut name_lens: Vec<u32> = vec![];
        let mut name_ptrs: Vec<u32> = vec![];

        for n in 0..langs_count {
            let name_len = read_u32(&name_data, 0 + 4 * n);
            let name_ptr = read_u32(&name_data, 4 + 4 * n);
            name_lens.push(name_len);
            name_ptrs.push(name_ptr);
        }

        Self {
            name_lens: name_lens,
            name_ptrs: name_ptrs,
        }
    }
}

impl SMFileRecord {
    pub fn parse(image: &SISImage, record: &Vec<u8>) -> Self {
        let lcount = image.header.langs_count as usize;
        let rec_type = read_u32(record, 0);
        let mut file_lens: Vec<u32> = vec![];
        let mut file_ptrs: Vec<u32> = vec![];
        let mut orig_file_lens: Vec<u32> = vec![];

        match rec_type {
            0 | 1 => {
                for n in 0..lcount {
                    let file_len = read_u32(record, 28 + 28 * n);
                    let file_ptr = read_u32(record, 32 + 32 * n);
                    let orig_file_len = read_u32(record, 36 + 36 * n);
                    file_lens.push(file_len);
                    file_ptrs.push(file_ptr);
                    orig_file_lens.push(orig_file_len);
                }
            }
            _ => todo!("File record type {rec_type}"),
        }

        Self {
            rec_type: rec_type,
            file_type: read_u32(record, 4),
            file_det: read_u32(record, 8),
            src_len: read_u32(record, 12),
            src_ptr: read_u32(record, 16),
            dest_len: read_u32(record, 20),
            dest_ptr: read_u32(record, 24),
            file_lens: file_lens,
            file_ptrs: file_ptrs,
            orig_file_lens: orig_file_lens,
            mime_type_len: read_u32(record, 40 + 40 * (lcount - 1)),
            mime_type_ptr: read_u32(record, 44 - 40 * (lcount - 1)),
        }
    }
}

impl SISImage {
    pub fn parse(data: &Vec<u8>) -> Self {
        Self {
            bin_data: data.to_vec(),
            header: SISHeader::parse(data),
            options: HeaderOptions::parse(read_u16(data, 36)),
        }
    }

    pub fn extract_files(image: &Self) {
        let name = get_name_string(image);
        let files_rec_start = image.header.files_ptr as usize;
        let files_count = image.header.files_count as usize;
        let langs_count = image.header.langs_count as usize;
        let mut records: Vec<SMFileRecord> = vec![];

        create_dir(&name).expect("Failed to create dir");

        for f in 0..files_count {
            let record_start = files_rec_start + 48 * f;
            let record_end = record_start + 48;
            let record_data = image.bin_data[record_start..record_end].to_vec();
            let record = SMFileRecord::parse(&image, &record_data);
            records.push(record);

            for l in 0..langs_count {
                let dest_start = records[f].dest_ptr as usize;
                let dest_end = dest_start + records[f].dest_len as usize;
                let dest_data = image.bin_data[dest_start..dest_end].to_vec();
                let file_start = records[f].file_ptrs[l] as usize;
                let file_end = file_start + records[f].file_lens[l] as usize;
                let file_data = image.bin_data[file_start..file_end].to_vec();
                let file_name = from_utf8(&dest_data)
                    .unwrap()
                    .split("\\")
                    .last()
                    .unwrap()
                    .replace("\0", "");

                let file_data_inflated = if image.options.no_compress {
                    file_data
                } else {
                    match decompress_to_vec_zlib(&file_data) {
                        Ok(v) => v,
                        Err(e) => panic!("Failed to inflate data: {e}"),
                    }
                };

                println!(
                    "\nFile {f} record type: {}",
                    get_rec_type_string(records[f].rec_type)
                );

                println!(
                    "File {f} type: {}",
                    get_file_type_string(records[f].file_type)
                );

                println!(
                    "File {f} details: {}",
                    get_file_det_string(records[f].file_det, records[f].file_type)
                );

                println!("File {f} length: {}", records[f].file_lens[l]);
                println!("File {f} pointer: {}", records[f].file_ptrs[l]);
                println!("File {f} original length: {}", records[f].orig_file_lens[l]);

                write(format!("{name}/{file_name}"), file_data_inflated).expect("Failed to write");
            }
        }
    }
}
