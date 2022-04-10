//! Extract content of the BTF section data from vmlinuz of aarch64
//! architecture. Note that the extract-vmlinux script that is located in the
//! kernel source works correctly only if vmlinuz is built for x86_64
//! architecture.
use std::{env, fs, ptr};

#[repr(C)]
#[derive(Debug)]
pub struct btf_header {
    pub magic: u16,
    pub version: u8,
    pub flags: u8,
    pub hdr_len: u32,
    pub type_off: u32,
    pub type_len: u32,
    pub str_off: u32,
    pub str_len: u32,
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        eprintln!("error: specify vmlinuz of aarch64");
        return;
    }

    let vmlinux = &args[1];
    let bytes = fs::read(vmlinux).expect("failed to read given file");
    let patt = [
        /* magic */ 0x9f, 0xeb, /* version */ 0x01, /* flags */ 0x00,
        /* hdr_len */ 0x18, 0x00, 0x00, 0x00,
    ];
    let pos = bytes
        .as_slice()
        .windows(patt.len())
        .position(|win| win == patt)
        .expect("btf header not found");
    let btf_hdr = unsafe { ptr::read_unaligned(bytes.as_ptr().add(pos) as *const btf_header) };
    let start = pos;
    let end = start + (btf_hdr.hdr_len + btf_hdr.str_off + btf_hdr.str_len) as usize;
    let btf_bytes = &bytes[start..end];
    let output = &args[2];
    fs::write(output, btf_bytes).expect("failed to write btf data");
}
