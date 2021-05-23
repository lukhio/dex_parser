pub fn decode(data: &[u8]) -> String {
    let length = data.len();
    let mut decoded: Vec<u32> = Vec::new();
    let mut idx = 0;

    while idx < length {
        let byte = data[idx] as u32;

        /* MUTF-8 encoded strings cannot have a null byte */
        if byte == 0x00 {
            panic!("[MUTF-8] null byte in MUTF-8 string");
        }

        /* No encoding */
        if (byte & !0x7f) == 0x00 {
            decoded.push(byte);

            idx += 1;
            continue;
        }


        /* Two bytes code point */
        if data[idx] >> 5 == 0b110 {
            if idx + 1 >= length {
                panic!("[MUTF-8] error: two bytes code point detected but not enough bytes");
            }

            let code_point = ((data[idx] & 0x1f) << 6) + (data[idx + 1] & 0x3f);
            decoded.push(code_point as u32);

            idx += 2;
            continue;

        }

        /* Three bytes code point */
        if byte >> 4 == 0b1110 {
            if idx + 2 >= length {
                panic!("[MUTF-8] error: three bytes code point detected but not enough bytes");
            }

            let y = data[idx + 1] as u32;
            let z = data[idx + 2] as u32;

            let code_point = ((byte & 0xf) << 12) + ((y & 0x3f) << 6) + (z & 0x3f);
            decoded.push(code_point as u32);

            idx += 3;
            continue;
        }

        /* Six bytes code point */
        if byte == 0b11101101 {
            if idx + 5 >= length {
                panic!("[MUTF-8] error: six bytes code point detected but not enough bytes");
            }

            let v = data[idx + 1] as u32;
            let w = data[idx + 2] as u32;
            let y = data[idx + 4] as u32;
            let z = data[idx + 5] as u32;

            let code_point = 0x10000 + ((v & 0x0f) << 16) +
                                       ((w & 0x3f) << 10) +
                                       ((y & 0x0f) << 6)  +
                                       (z & 0x3f);
            decoded.push(code_point as u32);

            idx += 6;
            continue;
        }

        panic!("[MUTF-8] error: unrecognized code point: {:02X} ({:b})", byte, byte);
    }

    let mut decoded_str = String::new();
    for byte in decoded.iter() {
        decoded_str.push(char::from_u32(*byte).unwrap());
    }
    return decoded_str;
}
