use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

// http://publications.alex-ionescu.com/Recon/ReconBru%202017%20-%20Getting%20Physical%20with%20USB%20Type-C,%20Windows%2010%20RAM%20Forensics%20and%20UEFI%20Attacks.pdf
// • The table itself has a header with the signature ‘RUNTSERV’ and the
// size of the table (which has always remained constant – 0x88)
// • By looking for RUNTSERV and then comparing the POOL_HEAD, we
// can validate that we have found the right data structure

/*
09a30b90: b000 0000 0000 0000 5255 4e54 5345 5256  ........RUNTSERV
09a30ba0: 3200 0200 8800 0000 7ec9 36e6 0000 0000  2.......~.6.....
09a30bb0: f4e2 b1fb feff ffff 00e3 b1fb feff ffff  ................
09a30bc0: 1ce3 b1fb feff ffff 28e3 b1fb feff ffff  ........(.......
09a30bd0: 5493 f9be 0000 0000 b892 f9be 0000 0000  T...............
09a30be0: 3453 b2fb feff ffff 1455 b2fb feff ffff  4S.......U......
09a30bf0: b456 b2fb feff ffff 3492 b1fb feff ffff  .V......4.......
09a30c00: 9c52 b5fb feff ffff f886 b7fb feff ffff  .R..............
09a30c10: 6489 b7fb feff ffff 0458 b2fb feff ffff  d........X......
*/

pub const RUNTSERV: &str = "RUNTSERV";

#[derive(AsBytes, FromBytes, FromZeroes, Clone, Copy, Debug)]
#[repr(C)]
pub struct RuntServ {
    signature: [u8; 8],
    rev: u32,
    size: u32,
    crc32: u32,
    _res: u32,
    get_time: u32,
    set_time: u32,
    get_wakeup_time: u32,
    set_wakeup_time: u32,
    set_virtual_address_map: u32,
    convert_pointer: u32,
    get_variable: u32,
    get_next_variable_name: u32,
    set_variable: u32,
    get_next_high_mono_count: u32,
    reset_system: u32,
    update_capsule: u32,
    query_capsule_capabilities: u32,
    query_variable_info: u32,
}

// https://github.com/tianocore/edk2/blob/5d533bbc27732a421e3bf35c5af77782b8a85e6f/MdeModulePkg/Core/Dxe/Mem/Pool.c#L30

pub const POOL_FREE: &str = "pfr0";
pub const POOL_HEAD: &str = "phd0";
pub const POOLPAGE_HEAD: &str = "phd1";
pub const POOL_TAIL: &str = "ptal";
pub const POOL: &str = "plst";

// https://github.com/tianocore/edk2/blob/5d533bbc27732a421e3bf35c5af77782b8a85e6f/MdeModulePkg/Core/Dxe/Hand/Handle.h#L47

pub const EFI_HANDLE: &str = "hndl";
pub const PROTOCOL_ENTRY: &str = "prte";
pub const PROTOCOL_INTERFACE: &str = "pifc";
pub const OPEN_PROTOCOL_DATA: &str = "podl";
pub const PROTOCOL_NOTIFY: &str = "prtn";

#[derive(AsBytes, FromBytes, FromZeroes, Clone, Copy, Debug)]
#[repr(C)]
pub struct PoolFree {
    signature: [u8; 4],
    index: u32,
    link: u32,
}

#[derive(AsBytes, FromBytes, FromZeroes, Clone, Copy, Debug)]
#[repr(C)]
pub struct PoolHead {
    signature: [u8; 4],
    _res: u32,
    mem_type: u32,
    size: u32,
    data: [u8; 8],
}

#[derive(AsBytes, FromBytes, FromZeroes, Clone, Copy, Debug)]
#[repr(C)]
pub struct PoolPageHead {
    signature: [u8; 4],
    _res: u32,
    mem_type: u32,
    size: u32,
    data: [u8; 8],
}

#[derive(AsBytes, FromBytes, FromZeroes, Clone, Copy, Debug)]
#[repr(C)]
pub struct PoolTail {
    signature: [u8; 4],
    _res: u32,
    size: u32,
}

#[derive(AsBytes, FromBytes, FromZeroes, Clone, Copy, Debug)]
#[repr(C)]
pub struct Pool {
    signature: [u8; 4],
    used: u32,
    mem_type: u32,
    free_list: u32,
    link: u32,
}
