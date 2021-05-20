#![allow(dead_code)]

use std::fs::File;
use std::io::{
    Read,
    Error
};

const MOD_ADLER: u32 = 65521;

/* Each DEX header contains an Adler-32 checksum of the file, minus the first
 * 11 bytes, which correspond to the space taken by the magic and the checksum.
 * This function computes the checksum of the file, and compares it to the one
 * found in the header.
 */
pub fn verify_from_path(fpath: &str, checksum: u32) -> Result<bool, Error> {

    /* Open file */
    let mut fp = File::open(fpath).expect("[Adler32] Error: cannot open file");

    /* Load file contents into a vector */
    let mut data = Vec::new();
    fp.read_to_end(&mut data).expect("[Adler32] Error: cannot read file");

    /* We must ignore the first 11 bytes of the file (which correspond to the
     * magic number and the checksum). We just drop them from the vector.
     *
     * Note: using remove will shift all the data to the left, which is why we
     * always remove the item at index 0 */
    for _ in 0..12 {
        data.remove(0);
    }

    /* Define variable for checksum computation */
    let mut a: u32 = 1;
    let mut b: u32 = 0;

    /* Main computation */
    for byte in data.iter() {
        a = (a + *byte as u32) % MOD_ADLER;
        b = (b + a) % MOD_ADLER;
    }

    /* Concatenating A and B */
    let computed_checksum = (b << 16) | a;

    /* Verification of the checksum read from the DEX header */
    Ok(checksum == computed_checksum)
}

/* This function does the same, but from an array of u8 which represent a DEX file */
pub fn verify_from_raw_data(raw_data: &[u8], checksum: u32) -> Result<bool, Error> {

    /* Define variable for checksum computation */
    let mut a: u32 = 1;
    let mut b: u32 = 0;

    /* Main computation
     * We must ignore the first 11 bytes of the file (which
     * correspond to the magic number and the checksum). */
    for byte in &raw_data[12..] {
        a = (a + *byte as u32) % MOD_ADLER;
        b = (b + a) % MOD_ADLER;
    }

    /* Concatenating A and B */
    let computed_checksum = (b << 16) | a;

    /* Verification of the checksum read from the DEX header */
    Ok(checksum == computed_checksum)
}
