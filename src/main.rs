mod cpu;
mod e32image;
mod sis;
mod utils;

use std::env::args;
use std::fs::read;

use crate::cpu::unicorn;
use crate::e32image::header::{
    HeaderFlags, V1Header, V1HeaderJ, V1HeaderV, V2Header, V2HeaderJ, V2HeaderV,
};
use crate::sis::header::{SISHeader, SISHeader6};
use crate::utils::read_u32;

fn main() {
    let file = args().nth(1).unwrap();
    let data = read(&file).unwrap();

    if file.ends_with(".sis") || file.ends_with(".sisx") {
        let header = SISHeader::parse(&data);
        let header6 = SISHeader6::parse(&data);

        SISHeader::print_info(&header);
    } else {
        if [0x1000, 0x2000, 0x4000].contains(&read_u32(&data, 20)) {
            println!("v1 image header detected");
            let header = V1Header::parse(&data);
            let header_j = V1HeaderJ::parse(&data);
            let header_v = V1HeaderV::parse(&data);
            let flags = read_u32(&data, 44);
            let flags_parsed = HeaderFlags::parse(flags);

            V1Header::print_info(&header, &flags_parsed);

            if flags_parsed.header_format >= 1 {
                V1HeaderJ::print_info(&header_j);
            }
        } else {
            println!("v2 image header detected");
            let header = V2Header::parse(&data);
            let header_j = V2HeaderJ::parse(&data);
            let header_v = V2HeaderV::parse(&data);
            let flags = read_u32(&data, 44);
            let flags_parsed = HeaderFlags::parse(flags);

            V2Header::print_info(&header, flags);

            if flags_parsed.header_format >= 1 {
                V2HeaderJ::print_info(&header_j);
            }
        }
    }
}
