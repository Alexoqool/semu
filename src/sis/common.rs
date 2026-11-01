use super::image::{NameRecord, SISImage};
use std::str::from_utf8;

pub fn get_name_string(image: &SISImage) -> String {
    let name_record = NameRecord::parse(image);
    let name_start = name_record.name_ptrs[0] as usize;
    let name_end = name_start + name_record.name_lens[0] as usize;
    let name_data = image.bin_data[name_start..name_end].to_vec();

    from_utf8(&name_data).unwrap().replace("\0", "")
}
