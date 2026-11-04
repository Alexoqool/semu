// https://www.thouky.co.uk/software/psifs/sis.html

use super::common::get_name_string;
use super::header::{HeaderOptions, SISHeader};
use crate::utils::{read_u16, read_u32};
use miniz_oxide::inflate::decompress_to_vec_zlib;
use std::fs::{create_dir, write};
use std::str::from_utf8;

fn get_language_string(data: u16) -> String {
    String::from(match data {
        0 => "Test",
        1 => "UK English",
        2 => "French",
        3 => "German",
        4 => "Spanish",
        5 => "Italian",
        6 => "Swedish",
        7 => "Danish",
        8 => "Norwegian",
        9 => "Finnish",
        10 => "American English",
        11 => "Swiss French",
        12 => "Swiss German",
        13 => "Portuguese",
        14 => "Turkish",
        15 => "Icelandic",
        16 => "Russian",
        17 => "Hungarian",
        18 => "Dutch",
        19 => "Belgian Flemish",
        20 => "Australian English",
        21 => "Belgian French",
        22 => "Austrian German",
        23 => "New Zealand English",
        24 => "Internetional French",
        25 => "Czech",
        26 => "Slovak",
        27 => "Polish",
        28 => "Slovenian",
        29 => "Taiwan Chinese",
        30 => "Hong Kong Chinese",
        31 => "PRC Chinese",
        32 => "Japanese",
        33 => "Thai",
        34 => "Afrikaans",
        35 => "Albanian",
        36 => "Amharic",
        37 => "Arabic",
        38 => "Armenian",
        39 => "Tagalog",
        40 => "Belarussian",
        41 => "Bengali",
        42 => "Bulgarian",
        43 => "Burmese",
        44 => "Catalan",
        45 => "Croatian",
        46 => "Canadian English",
        47 => "International English",
        48 => "South African English",
        49 => "Estonian",
        50 => "Farsi",
        51 => "Canadian French",
        52 => "Scots Gaelic",
        53 => "Georgian",
        54 => "Greek",
        55 => "Cyprus Greek",
        56 => "Gujarati",
        57 => "Hebrew",
        58 => "Hindi",
        59 => "Indonesian",
        60 => "Irish",
        61 => "Swiss Italian",
        62 => "Kannada",
        63 => "Kazakh",
        64 => "Khmer",
        65 => "Korean",
        66 => "Laothian",
        67 => "Latvian",
        68 => "Lithuanian",
        69 => "Macedonian",
        70 => "Malay",
        71 => "Malayalam",
        72 => "Marathi",
        73 => "Moldavian",
        74 => "Mongolian",
        75 => "Norwegian-Nynorsk",
        76 => "Brazilian Portuguese",
        77 => "Punjabi",
        78 => "Romanian",
        79 => "Serbian",
        80 => "Sinhalese",
        81 => "Somali",
        82 => "International Spanish",
        83 => "Latin American Spanish",
        84 => "Swahili",
        85 => "Finland Swedish",
        87 => "Tamil",
        88 => "Telugu",
        89 => "Tibetan",
        90 => "Tigrinya",
        91 => "Cyprus Turkish",
        92 => "Turkmen",
        93 => "Ukrainian",
        94 => "Urdu",
        95 => "Vietnamese",
        96 => "Welsh",
        97 => "Zulu",
        _ => "unknown",
    })
}

fn get_rec_type_string(data: u32) -> String {
    String::from(match data {
        0 => "Simple file line",
        1 => "Multiple language files line",
        2 => "Options line",
        3 => "If line",
        4 => "ElseIf line",
        5 => "Else line",
        6 => "EndIf line",
        _ => "unknown",
    })
}

fn get_file_type_string(data: u32) -> String {
    String::from(match data {
        0 => "FILE",
        1 => "FILETEXT",
        2 => "SIS component file",
        3 => "FILERUN",
        4 => "FILENULL",
        5 => "FILEMIME",
        _ => "unknown",
    })
}

fn get_file_det_string(data: u32, ft: u32) -> String {
    String::from(match ft {
        // FILERUN
        3 => match data {
            0 => "RUNINSTALL",
            1 => "RUNREMOVE",
            2 => "RUNBOTH",
            0x100 => "RUNSENDED",
            0x200 => "RUNWAIT",
            _ => "unknown",
        },
        // FILEMIME
        5 => match data {
            256 => "OPENCLOSE",
            512 => "OPENWAIT",
            _ => "unknown",
        },
        _ => match data {
            0 => "TEXTCONTINUE",
            1 => "TEXTSKIP",
            2 => "TEXTABORT",
            3 => "TEXTEXIT",
            _ => "unknown",
        },
    })
}

pub struct OptionString {
    pub opt_len: u32,
    pub opt_ptr: u32,
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
pub struct OptionsRecord {
    pub rec_type: u32,
    pub opts_count: u32,
    pub opt_strs: Vec<OptionString>,
    pub selected: u32,
}

#[allow(dead_code)]
pub struct IEIfRecord {
    pub rec_type: u32,
    pub cond_exp_len: u32,
    pub cond_exp: Vec<u8>,
}

#[allow(dead_code)]
pub struct EEIfRecord {
    pub rec_type: u32,
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
        let mut file_lens: Vec<u32> = vec![];
        let mut file_ptrs: Vec<u32> = vec![];
        let mut orig_file_lens: Vec<u32> = vec![];

        for n in 0..langs_count {
            let file_len = read_u32(record, 28 + 28 * n);
            let file_ptr = read_u32(record, 32 + 32 * n);
            let orig_file_len = read_u32(record, 36 + 36 * n);
            file_lens.push(file_len);
            file_ptrs.push(file_ptr);
            orig_file_lens.push(orig_file_len);
        }

        Self {
            rec_type: read_u32(record, 0),
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

// untested
impl OptionsRecord {
    pub fn parse(image: &SISImage, record: &Vec<u8>) -> Self {
        let langs_count = image.header.langs_count as usize;
        let opts_count = read_u32(record, 4) as usize;
        let mut opt_strs: Vec<OptionString> = vec![];

        for m in 0..opts_count {
            for n in 0..langs_count {
                let opt_str = OptionString {
                    opt_len: read_u32(record, 8 + 8 * n * m),
                    opt_ptr: read_u32(record, 12 + 12 * n * m),
                };
                opt_strs.push(opt_str);
            }
        }

        Self {
            rec_type: read_u32(record, 0),
            opts_count: read_u32(record, 4),
            opt_strs: opt_strs,
            selected: 0,
        }
    }
}

impl IEIfRecord {
    pub fn parse(record: &Vec<u8>) -> Self {
        let cond_exp_len = read_u32(record, 4);
        let cond_exp_end = (cond_exp_len + 8) as usize;

        IEIfRecord {
            rec_type: read_u32(record, 0),
            cond_exp_len: cond_exp_len,
            cond_exp: record[8..cond_exp_end].to_vec(),
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
        let mut file_records: Vec<SMFileRecord> = vec![];
        let mut opts_records: Vec<OptionsRecord> = vec![];
        let mut ieif_records: Vec<IEIfRecord> = vec![];
        let mut eeif_records: Vec<EEIfRecord> = vec![];
        let mut cur_file_rec = 0;
        let mut cur_opts_rec = 0;
        let mut cur_ieif_rec = 0;
        let mut cur_eeif_rec = 0;
        let mut next_rec_ptr = files_rec_start;

        create_dir(&name).expect("Failed to create dir");
        for l in 0..langs_count {
            let record_start = langs_rec_start + 2 * l;
            let lang_data = read_u16(&image.bin_data, record_start);
            let lang = get_language_string(lang_data);
            langs.push(lang);
            create_dir(format!("{name}/{}", langs[l])).expect("Failed to create dir");

            for f in 0..files_count {
                let rec_type = read_u32(&image.bin_data, next_rec_ptr);

                match rec_type {
                    0 | 1 => {
                        let rec_start = next_rec_ptr;
                        let rec_end = rec_start + 48 + 12 * l;
                        let rec_data = image.bin_data[rec_start..rec_end].to_vec();
                        let record = SMFileRecord::parse(&image, &rec_data);

                        file_records.push(record);
                        next_rec_ptr += 48 + 12 * l;

                        if file_records[cur_file_rec].file_type == 0 {
                            let dest_start = file_records[cur_file_rec].dest_ptr as usize;
                            let dest_end =
                                dest_start + file_records[cur_file_rec].dest_len as usize;
                            let dest_data = image.bin_data[dest_start..dest_end].to_vec();
                            let file_name = from_utf8(&dest_data)
                                .unwrap()
                                .split("\\")
                                .last()
                                .unwrap()
                                .replace("\0", "");

                            let file_start = file_records[cur_file_rec].file_ptrs[l] as usize;
                            let file_end =
                                file_start + file_records[cur_file_rec].file_lens[l] as usize;
                            let file_data = image.bin_data[file_start..file_end].to_vec();
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

                        println!("\nFile {f} language: {}", langs[l]);
                        println!(
                            "File {f} record type: {}",
                            get_rec_type_string(file_records[cur_file_rec].rec_type)
                        );

                        println!(
                            "File {f} type: {}",
                            get_file_type_string(file_records[cur_file_rec].file_type)
                        );

                        println!(
                            "File {f} details: {}",
                            get_file_det_string(
                                file_records[cur_file_rec].file_det,
                                file_records[cur_file_rec].file_type
                            )
                        );

                        println!(
                            "File {f} length: {}",
                            file_records[cur_file_rec].file_lens[l]
                        );

                        println!(
                            "File {f} pointer: {}",
                            file_records[cur_file_rec].file_ptrs[l]
                        );

                        println!(
                            "File {f} original length: {}",
                            file_records[cur_file_rec].orig_file_lens[l]
                        );

                        cur_file_rec += 1;
                    }
                    2 => {
                        let opts_count = read_u32(&image.bin_data, next_rec_ptr + 4) as usize;
                        let rec_start = next_rec_ptr;
                        let rec_end = rec_start + 16 + 8 * l * opts_count;
                        let rec_data = image.bin_data[rec_start..rec_end].to_vec();
                        let record = OptionsRecord::parse(image, &rec_data);

                        opts_records.push(record);
                        next_rec_ptr += 16 + 8 * l * opts_count;

                        println!("\nFile {f} language: {}", langs[l]);
                        println!(
                            "File {f} record type: {}",
                            get_rec_type_string(opts_records[cur_opts_rec].rec_type)
                        );

                        for n in 0..opts_count {
                            let opt_name_start =
                                opts_records[cur_opts_rec].opt_strs[cur_opts_rec].opt_ptr as usize;

                            let opt_name_end = opt_name_start
                                + opts_records[cur_opts_rec].opt_strs[cur_opts_rec].opt_len
                                    as usize;

                            let opt_name_data =
                                image.bin_data[opt_name_start..opt_name_end].to_vec();

                            let opt_name = from_utf8(&opt_name_data).unwrap().replace("\0", "");

                            println!("Option {n} name: {opt_name}");
                        }

                        cur_file_rec -= 1;
                        cur_opts_rec += 1;
                    }
                    3 | 4 => {
                        let cond_exp_len = read_u32(&image.bin_data, next_rec_ptr + 4) as usize;
                        let rec_start = next_rec_ptr;
                        let rec_end = rec_start + cond_exp_len + 8;
                        let rec_data = image.bin_data[rec_start..rec_end].to_vec();
                        let record = IEIfRecord::parse(&rec_data);

                        ieif_records.push(record);
                        next_rec_ptr += 8 + cond_exp_len;

                        println!("\nFile {f} language: {}", langs[l]);
                        println!(
                            "File {f} record type: {}",
                            get_rec_type_string(ieif_records[cur_ieif_rec].rec_type)
                        );

                        println!(
                            "File {f} expression size: {}",
                            ieif_records[cur_ieif_rec].cond_exp_len
                        );

                        cur_file_rec -= 1;
                        cur_ieif_rec += 1;
                    }
                    5 | 6 => {
                        let rec_start = next_rec_ptr;
                        let rec_end = rec_start + 4;
                        let rec_data = image.bin_data[rec_start..rec_end].to_vec();
                        let record = EEIfRecord {
                            rec_type: read_u32(&rec_data, 0),
                        };

                        eeif_records.push(record);
                        next_rec_ptr += 4;

                        println!("\nFile {f} language: {}", langs[l]);
                        println!(
                            "File {f} record type: {}",
                            get_rec_type_string(eeif_records[cur_eeif_rec].rec_type)
                        );

                        cur_file_rec -= 1;
                        cur_eeif_rec += 1;
                    }
                    _ => panic!("Unknown record type {rec_type}"),
                }
            }
        }
    }
}
