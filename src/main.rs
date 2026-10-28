mod cpu;
mod e32image;
mod utils;

use std::env::args;
use std::fs::read;

use crate::cpu::unicorn;
use crate::e32image::{header, loader};

fn main() {
    let file = args().nth(1).unwrap();
    let data = read(file).unwrap();
    let header = header::E32ImageHeader::parse(&data);
    let flags_parsed = header::Flags::parse(data.get(44..48).unwrap().try_into().unwrap());

    println!(
        "type: {}",
        match flags_parsed.executable_type {
            false => "exe",
            true => "dll",
        }
    );

    println!(
        "call entry point: {}",
        match flags_parsed.call_entry_point {
            false => "yes",
            true => "no",
        }
    );

    println!(
        "fixed address: {}",
        match flags_parsed.fixed_address {
            false => "no",
            true => "yes",
        }
    );

    println!(
        "abi: {}",
        match flags_parsed.abi {
            0 => "GCC98r2",
            1 => "EABI",
            _ => "unknown",
        }
    );

    println!(
        "entry point type: {}",
        match flags_parsed.entry_point_type {
            0 => "EKA1",
            1 => "EKA2",
            _ => "unknown",
        }
    );

    println!(
        "header format: {}",
        match flags_parsed.header_format {
            0 => "basic",
            1 => "J",
            2 => "V",
            _ => "unknown",
        }
    );

    println!(
        "import format: {}",
        match flags_parsed.import_format {
            0 => "standard PE",
            1 => "ELF",
            2 => "PE without redunancy",
            _ => "unknown",
        }
    );

    println!(
        "priority: {}",
        match header.priority {
            150 => "low",
            250 => "background",
            350 => "foreground",
            450 => "high",
            650 => "windowserver",
            750 => "fileserver",
            850 => "realtimeserver",
            950 => "supervisor",
            _ => "unknown",
        }
    );
    println!(
        "cpu: {}",
        match header.cpu {
            0x1000 => "x86",
            0x2000 => "armv4",
            0x2001 => "armv5",
            0x2002 => "armv6",
            0x4000 => "mcore",
            _ => "unknown",
        }
    );
    println!(
        "Tools version: {}.{} ({})",
        header.tools_version.major, header.tools_version.minor, header.tools_version.build
    );
    println!(
        "Module version: {}.{}",
        header.module_version.major, header.module_version.minor
    );
}
