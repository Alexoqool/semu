// https://www.thouky.co.uk/software/psifs/sis.html

use super::common::get_name_string;
use super::header::{HeaderOptions, SISHeader};
use crate::utils::{read_u16, read_u32};
use miniz_oxide::inflate::decompress_to_vec_zlib;
use std::fs::{create_dir, write};
use std::str::from_utf8;

fn get_language_string(data: u16) -> String {
    match data {
        0 => String::from("Test"),
        1 => String::from("UK English"),
        2 => String::from("French"),
        3 => String::from("German"),
        4 => String::from("Spanish"),
        5 => String::from("Italian"),
        6 => String::from("Swedish"),
        7 => String::from("Danish"),
        8 => String::from("Norwegian"),
        9 => String::from("Finnish"),
        10 => String::from("American English"),
        11 => String::from("Swiss French"),
        12 => String::from("Swiss German"),
        13 => String::from("Portuguese"),
        14 => String::from("Turkish"),
        15 => String::from("Icelandic"),
        16 => String::from("Russian"),
        17 => String::from("Hungarian"),
        18 => String::from("Dutch"),
        19 => String::from("Belgian Flemish"),
        20 => String::from("Australian English"),
        21 => String::from("Belgian French"),
        22 => String::from("Austrian German"),
        23 => String::from("New Zealand English"),
        24 => String::from("Internetional French"),
        25 => String::from("Czech"),
        26 => String::from("Slovak"),
        27 => String::from("Polish"),
        28 => String::from("Slovenian"),
        29 => String::from("Taiwan Chinese"),
        30 => String::from("Hong Kong Chinese"),
        31 => String::from("PRC Chinese"),
        32 => String::from("Japanese"),
        33 => String::from("Thai"),
        34 => String::from("Afrikaans"),
        35 => String::from("Albanian"),
        36 => String::from("Amharic"),
        37 => String::from("Arabic"),
        38 => String::from("Armenian"),
        39 => String::from("Tagalog"),
        40 => String::from("Belarussian"),
        41 => String::from("Bengali"),
        42 => String::from("Bulgarian"),
        43 => String::from("Burmese"),
        44 => String::from("Catalan"),
        45 => String::from("Croatian"),
        46 => String::from("Canadian English"),
        47 => String::from("International English"),
        48 => String::from("South African English"),
        49 => String::from("Estonian"),
        50 => String::from("Farsi"),
        51 => String::from("Canadian French"),
        52 => String::from("Scots Gaelic"),
        53 => String::from("Georgian"),
        54 => String::from("Greek"),
        55 => String::from("Cyprus Greek"),
        56 => String::from("Gujarati"),
        57 => String::from("Hebrew"),
        58 => String::from("Hindi"),
        59 => String::from("Indonesian"),
        60 => String::from("Irish"),
        61 => String::from("Swiss Italian"),
        62 => String::from("Kannada"),
        63 => String::from("Kazakh"),
        64 => String::from("Khmer"),
        65 => String::from("Korean"),
        66 => String::from("Laothian"),
        67 => String::from("Latvian"),
        68 => String::from("Lithuanian"),
        69 => String::from("Macedonian"),
        70 => String::from("Malay"),
        71 => String::from("Malayalam"),
        72 => String::from("Marathi"),
        73 => String::from("Moldavian"),
        74 => String::from("Mongolian"),
        75 => String::from("Norwegian-Nynorsk"),
        76 => String::from("Brazilian Portuguese"),
        77 => String::from("Punjabi"),
        78 => String::from("Romanian"),
        79 => String::from("Serbian"),
        80 => String::from("Sinhalese"),
        81 => String::from("Somali"),
        82 => String::from("International Spanish"),
        83 => String::from("Latin American Spanish"),
        84 => String::from("Swahili"),
        85 => String::from("Finland Swedish"),
        87 => String::from("Tamil"),
        88 => String::from("Telugu"),
        89 => String::from("Tibetan"),
        90 => String::from("Tigrinya"),
        91 => String::from("Cyprus Turkish"),
        92 => String::from("Turkmen"),
        93 => String::from("Ukrainian"),
        94 => String::from("Urdu"),
        95 => String::from("Vietnamese"),
        96 => String::from("Welsh"),
        97 => String::from("Zulu"),
        _ => String::from("unknown"),
    }
}

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
        let langs_count = image.header.langs_count as usize;
        let rec_type = read_u32(record, 0);
        let mut file_lens: Vec<u32> = vec![];
        let mut file_ptrs: Vec<u32> = vec![];
        let mut orig_file_lens: Vec<u32> = vec![];

        match rec_type {
            0 | 1 => {
                for n in 0..langs_count {
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
            mime_type_len: read_u32(record, 40 + 40 * (langs_count - 1)),
            mime_type_ptr: read_u32(record, 44 - 40 * (langs_count - 1)),
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
        let langs_rec_start = image.header.langs_ptr as usize;
        let files_rec_start = image.header.files_ptr as usize;
        let files_count = image.header.files_count as usize;
        let langs_count = image.header.langs_count as usize;
        let mut langs: Vec<String> = vec![];
        let mut records: Vec<SMFileRecord> = vec![];

        create_dir(&name).expect("Failed to create dir");
        for l in 0..langs_count {
            let record_start = langs_rec_start + 2 * l;
            let lang_data = read_u16(&image.bin_data, record_start);
            let lang = get_language_string(lang_data);
            langs.push(lang);
            create_dir(format!("{name}/{}", langs[l])).expect("Failed to create dir");

            for f in 0..files_count {
                let record_start = files_rec_start + 48 * f;
                let record_end = record_start + 48;
                let record_data = image.bin_data[record_start..record_end].to_vec();
                let record = SMFileRecord::parse(&image, &record_data);
                records.push(record);

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

                println!("\nFile {f} language: {}", langs[l]);

                println!(
                    "File {f} record type: {}",
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

                if records[f].file_type == 0 {
                    let file_data_inflated = if image.options.no_compress {
                        file_data
                    } else {
                        match decompress_to_vec_zlib(&file_data) {
                            Ok(v) => v,
                            Err(e) => panic!("Failed to inflate data: {e}"),
                        }
                    };

                    write(
                        format!("{name}/{}/{file_name}", langs[l]),
                        file_data_inflated,
                    )
                    .expect("Failed to write file");
                }
            }
        }
    }
}
