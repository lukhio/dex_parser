use byteorder::{
    LittleEndian,
    ByteOrder,
    ReadBytesExt
};
use std::fs::File;
use std::io::{
    Read,
    Error,
    Seek,
    SeekFrom
};
use nom::*;
mod adler32;

struct Header {
    version: [u8; 3],
    checksum: u32,
    signature: [u8; 20],
    file_size: u32,
    header_size: u32,
    endian_tag: u32,
    link_size: u32,
    link_off: u32,
    map_off: u32,
    string_ids_size: u32,
    string_ids_off: u32,
    type_ids_size: u32,
    type_ids_off: u32,
    proto_ids_size: u32,
    proto_ids_off: u32,
    fields_ids_size: u32,
    fields_ids_off: u32,
    method_ids_size: u32,
    method_ids_off: u32,
    class_defs_size: u32,
    class_defs_off: u32,
    data_size: u32,
    data_off: u32
}

/* Endianness constants */
const ENDIAN_CONSTANT: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
const REVERSE_ENDIAN_CONSTANT: [u8; 4] = [0x78, 0x56, 0x34, 0x12];

impl Header {
    fn from_path(mut file_path: &str, endianness: nom::number::Endianness) -> Result<Self, Error> {
        /* TODO: right now we do not use the endianness arg. We assume the DEX
         * file is little endian (as per the standard) but it might not always
         * be the case.
         */ 

        let mut raw_file = File::open(file_path)
                           .expect("Error: cannot open classes.dex");

        /* DEX version */
        /* There is probably a nicer way to do this */
        let mut magic = [0; 8];
        let _ = raw_file.by_ref().take(8).read(&mut magic);
        let mut version = [0; 3];
        version[0] = magic[4];
        version[1] = magic[5];
        version[2] = magic[6];

        /* Verify Adler-32 checksum */
        let checksum = raw_file.read_u32::<LittleEndian>().unwrap();
        let do_checksum_match = adler32::verify(&file_path, checksum)
                                .expect("Error: cannot verify checksum");
        if !do_checksum_match {
            panic!("Error: Adler32 checksum does not match");
        }

        // TODO: maybe check the signature as well
        let mut signature = [0; 20];
        let _ = raw_file.by_ref().take(20).read(&mut signature);

        let file_size = raw_file.read_u32::<LittleEndian>().unwrap();
        let header_size = raw_file.read_u32::<LittleEndian>().unwrap();

        /* TODO: maybe store the nom::number::Endianness here instead? */
        let endian_tag = raw_file.read_u32::<LittleEndian>().unwrap();

        let link_size = raw_file.read_u32::<LittleEndian>().unwrap();
        let link_off = raw_file.read_u32::<LittleEndian>().unwrap();
        let map_off = raw_file.read_u32::<LittleEndian>().unwrap();
        let string_ids_size = raw_file.read_u32::<LittleEndian>().unwrap();
        let string_ids_off = raw_file.read_u32::<LittleEndian>().unwrap();
        let type_ids_size = raw_file.read_u32::<LittleEndian>().unwrap();
        let type_ids_off = raw_file.read_u32::<LittleEndian>().unwrap();
        let proto_ids_size = raw_file.read_u32::<LittleEndian>().unwrap();
        let proto_ids_off = raw_file.read_u32::<LittleEndian>().unwrap();
        let fields_ids_size = raw_file.read_u32::<LittleEndian>().unwrap();
        let fields_ids_off = raw_file.read_u32::<LittleEndian>().unwrap();
        let method_ids_size = raw_file.read_u32::<LittleEndian>().unwrap();
        let method_ids_off = raw_file.read_u32::<LittleEndian>().unwrap();
        let class_defs_size = raw_file.read_u32::<LittleEndian>().unwrap();
        let class_defs_off = raw_file.read_u32::<LittleEndian>().unwrap();
        let data_size = raw_file.read_u32::<LittleEndian>().unwrap();
        let data_off = raw_file.read_u32::<LittleEndian>().unwrap();

        Ok(Header {
            version: version,
            checksum: checksum,
            signature: signature,
            file_size: file_size,
            header_size: header_size,
            endian_tag: endian_tag,
            link_size: link_size,
            link_off: link_off,
            map_off: map_off,
            string_ids_size: string_ids_size,
            string_ids_off: string_ids_off,
            type_ids_size: type_ids_size,
            type_ids_off: type_ids_off,
            proto_ids_size: proto_ids_size,
            proto_ids_off: proto_ids_off,
            fields_ids_size: fields_ids_size,
            fields_ids_off: fields_ids_off,
            method_ids_size: method_ids_size,
            method_ids_off: method_ids_off,
            class_defs_size: class_defs_size,
            class_defs_off: class_defs_off,
            data_size: data_size,
            data_off:data_off
        })
    }
}

fn main() {
    // TODO: extract from the APK file given as argument
    let file_path = "classes.dex";

    let mut raw_file = File::open(file_path)
                       .expect("Error: cannot open classes.dex");

    /* We first check the endianness. By standard it should be
     * little-endian but it is allowed to change to big-endian.
     */
    raw_file.seek(SeekFrom::Start(40))
            .expect("Error: cannot read endian tag");
    let mut endian_tag = [0; 4];
    let _ = raw_file.by_ref().take(4).read(&mut endian_tag);

    let header = match endian_tag {
        ENDIAN_CONSTANT => Header::from_path(file_path, nom::number::Endianness::Big),
        REVERSE_ENDIAN_CONSTANT => Header::from_path(file_path, nom::number::Endianness::Little),
        _ => return panic!("Bad endian tag in header"),
    };
}
