/* Decode a MUTF-8 encoded string, and returns it as a string.
 * This algorithm is based on the Java JVM specification, section 4.4.7.
 * */
pub fn decode(data: &[u8]) -> String {

    /* Length of the MUTF-8 encoded string */
    let length = data.len();

    /* Vector to hold the decoded code points */
    let mut decoded: Vec<u32> = Vec::new();

    /* Index to iterate over the characters of the MUTF-8 encoded string */
    let mut idx = 0;

    while idx < length {
        /* Current byte */
        let byte = data[idx] as u32;

        /* MUTF-8 encoded strings cannot have a null byte */
        if byte == 0x00 {
            panic!("[MUTF-8] null byte in MUTF-8 string");
        }

        /* No encoding. We can just store the current byte as is. */
        if (byte & !0x7f) == 0x00 {
            decoded.push(byte);

            /* Incrementing the index */
            idx += 1;
            continue;
        }


        /* Two bytes code point */
        if data[idx] >> 5 == 0b110 {
            if idx + 1 >= length {
                panic!("[MUTF-8] error: two bytes code point detected but not enough bytes");
            }

            /* Decoding the code point */
            let code_point = ((data[idx] & 0x1f) << 6) + (data[idx + 1] & 0x3f);
            decoded.push(code_point as u32);

            /* Incrementing the index */
            idx += 2;
            continue;

        }

        /* Three bytes code point */
        if byte >> 4 == 0b1110 {
            if idx + 2 >= length {
                panic!("[MUTF-8] error: three bytes code point detected but not enough bytes");
            }

            /* Decoding the code point.
             * We have to first convert the bytes in u32 to avoid overflow.*/
            let y = data[idx + 1] as u32;
            let z = data[idx + 2] as u32;

            let code_point = ((byte & 0xf) << 12) + ((y & 0x3f) << 6) + (z & 0x3f);
            decoded.push(code_point as u32);

            /* Incrementing the index */
            idx += 3;
            continue;
        }

        /* Six bytes code point */
        if byte == 0b11101101 {
            if idx + 5 >= length {
                panic!("[MUTF-8] error: six bytes code point detected but not enough bytes");
            }

            /* Decoding the code point.
             * We have to first convert the bytes in u32 to avoid overflow.*/
            let v = data[idx + 1] as u32;
            let w = data[idx + 2] as u32;
            let y = data[idx + 4] as u32;
            let z = data[idx + 5] as u32;

            let code_point = 0x10000 + ((v & 0x0f) << 16) +
                                       ((w & 0x3f) << 10) +
                                       ((y & 0x0f) << 6)  +
                                       (z & 0x3f);
            decoded.push(code_point as u32);

            /* Incrementing the index */
            idx += 6;
            continue;
        }

        panic!("[MUTF-8] error: unrecognized code point: {:02X} ({:b})", byte, byte);
    }

    /* Converting the decoding code points (which are u32) into chars, which
     * we just append to the decoded string */
    let mut decoded_str = String::new();
    for byte in decoded.iter() {
        decoded_str.push(char::from_u32(*byte).unwrap());
    }

    return decoded_str;
}
