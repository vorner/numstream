use std::io::{self, Write};

const BLOCK_SIZE: u64 = 1024 * 16;

fn main() {
    let mut block = [0u64; BLOCK_SIZE as usize];
    let stdout_ll = io::stdout();
    let mut stdout = stdout_ll.lock();
    for i in 0.. {
        let offset = i * BLOCK_SIZE;
        for (i, val) in block.iter_mut().enumerate() {
            *val = offset + i as u64;
        }
        // There really should be a nicer way to get the bytes representation
        let block_bytes: &[u8; BLOCK_SIZE as usize * 8] = unsafe { std::mem::transmute(&block) };
        stdout.write_all(block_bytes).unwrap();
    }
}
