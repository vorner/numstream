extern crate byteorder;

use std::io::{self, Write};
use byteorder::{ByteOrder, NativeEndian};

const BLOCK_SIZE: u64 = 1024 * 16;
const BLOCK_SIZE_BYTES: usize = BLOCK_SIZE as usize * 8;

fn main() {
    let mut block = [0u8; BLOCK_SIZE_BYTES];
    let stdout_ll = io::stdout();
    let mut stdout = stdout_ll.lock();
    for i in 0.. {
        let offset = i * BLOCK_SIZE;
        for i in 0..BLOCK_SIZE {
            let index = i as usize;
            NativeEndian::write_u64(&mut block[index * 8..(index + 1) * 8], i + offset);
        }
        stdout.write_all(&block).unwrap();
    }
}
