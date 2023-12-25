use clap::Parser;
use clap_num::maybe_hex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;
use zerocopy::FromBytes;

mod edk2;
use edk2::{PoolFree, PoolHead};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File to read
    #[arg(short, long)]
    file: String,

    /// Offset
    #[arg(short, long, default_value_t = 0, value_parser=maybe_hex::<u64>)]
    offset: u64,

    /// Limit
    #[arg(short, long, default_value_t = 4096, value_parser=maybe_hex::<u64>)]
    limit: u64,

    /// Step
    #[arg(short, long, default_value_t = 4, value_parser=maybe_hex::<u8>)]
    step: u8,

    /// Pattern
    #[arg(short, long, default_value = "CyReVolt")]
    pattern: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let offset = args.offset;
    let limit = args.limit;
    let file = args.file;
    let pattern = args.pattern;
    let step = args.step;

    if pattern.len() < 8 {
        panic!("Use an 8-byte pattern!");
    }
    let p: &str = &pattern[..8];

    println!("Scanning {file} from offset 0x{offset:08x} for known patterns");

    let mut f = File::open(file)?;
    for o in (offset..offset + limit).step_by(step as usize) {
        f.seek(SeekFrom::Start(o))?;
        let buf = &mut [0u8; 24];
        let _ = f.read(buf);
        if let Ok(s) = std::str::from_utf8(&buf[..8]) {
            if s.starts_with(p) {
                println!("{o:08x}: {s:?}");
            }
            if s.starts_with(edk2::RUNTSERV) {
                f.seek(SeekFrom::Start(o))?;
                let buf = &mut [0u8; 88];
                let _ = f.read(buf);
                let r = edk2::RuntServ::read_from_prefix(buf).unwrap();
                println!("{o:08x}: {r:#x?}");
            }
        }

        if let Ok(s) = std::str::from_utf8(&buf[..4]) {
            if s.starts_with(edk2::POOL_FREE) {
                let r = PoolFree::read_from_prefix(buf).unwrap();
                println!("{o:08x}: {r:#x?}");
            }
            if s.starts_with(edk2::POOL_HEAD) {
                let r = PoolHead::read_from_prefix(buf).unwrap();
                println!("{o:08x}: {r:#x?}");
            }
            if s.starts_with(edk2::POOLPAGE_HEAD) {
                let r = edk2::PoolPageHead::read_from_prefix(buf).unwrap();
                println!("{o:08x}: {r:#x?}");
            }
            if s.starts_with(edk2::POOL_TAIL) {
                let r = edk2::PoolTail::read_from_prefix(buf).unwrap();
                println!("{o:08x}: {r:#x?}");
            }
            if s.starts_with(edk2::POOL) {
                let r = edk2::Pool::read_from_prefix(buf).unwrap();
                println!("{o:08x}: {r:#x?}");
            }

            if s.starts_with(edk2::EFI_HANDLE) {
                println!("{o:08x}: EFI handle");
            }
            if s.starts_with(edk2::PROTOCOL_ENTRY) {
                println!("{o:08x}: protocol entry");
            }
            if s.starts_with(edk2::PROTOCOL_INTERFACE) {
                println!("{o:08x}: protocol interface");
            }
            if s.starts_with(edk2::OPEN_PROTOCOL_DATA) {
                println!("{o:08x}: open protocol data");
            }
            if s.starts_with(edk2::PROTOCOL_NOTIFY) {
                println!("{o:08x}: protocol notify");
            }
        }
    }
    Ok(())
}
