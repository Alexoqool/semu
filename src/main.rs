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
}
