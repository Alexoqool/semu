mod cpu;
mod e32image;
mod sis;
mod utils;

use std::env::args;
use std::fs::read;

use crate::cpu::unicorn;
use crate::e32image::header::{HeaderFlags, V1Header, V2Header, V2HeaderJ, V2HeaderV};
use crate::sis::header::SISHeader;
use crate::sis::image::SISImage;
use crate::utils::read_u32;

fn main() {
    let args: Vec<String> = args().collect();
    let file = &args[1];
    let data = read(&file).unwrap();

    if file.to_uppercase().ends_with(".SIS") || file.to_uppercase().ends_with(".SISX") {
        let image = SISImage::parse(&data);

        SISHeader::print_info(&image);

        if args.contains(&String::from("--extract")) {
            SISImage::extract_files(&image);
        }
    } else {
        if [0x1000, 0x2000, 0x4000].contains(&read_u32(&data, 20)) {
            println!("v1 image header detected");
            let header = V1Header::parse(&data);
            let flags_parsed = HeaderFlags::parse(header.flags);

            V1Header::print_info(&header, &flags_parsed);
        } else {
            println!("v2 image header detected");
            let header = V2Header::parse(&data);
            let header_j = V2HeaderJ::parse(&data);
            let header_v = V2HeaderV::parse(&data);
            let flags_parsed = HeaderFlags::parse(header.flags);

            V2Header::print_info(&header, &flags_parsed);

            if flags_parsed.header_format >= 1 {
                V2HeaderJ::print_info(&header_j);
            }
        }
    }
}
