use byteorder::{
    LittleEndian,
    ReadBytesExt
};
use std::fs;
use std::io::{
    Read,
    Error,
    Seek,
    SeekFrom
};
mod adler32;

struct Header {
    _version: [u8; 3],
    _checksum: u32,
    _signature: [u8; 20],
    _file_size: u32,
    _header_size: u32,
    _endian_tag: u32,
    _link_size: u32,
    _link_off: u32,
    _map_off: u32,
    _string_ids_size: u32,
    _string_ids_off: u32,
    _type_ids_size: u32,
    _type_ids_off: u32,
    _proto_ids_size: u32,
    _proto_ids_off: u32,
    _fields_ids_size: u32,
    _fields_ids_off: u32,
    _method_ids_size: u32,
    _method_ids_off: u32,
    _class_defs_size: u32,
    _class_defs_off: u32,
    _data_size: u32,
    _data_off: u32
}

/* Endianness constants */
const ENDIAN_CONSTANT: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
const REVERSE_ENDIAN_CONSTANT: [u8; 4] = [0x78, 0x56, 0x34, 0x12];

impl Header {
    fn from_path(file_path: &str, endianness: nom::number::Endianness) -> Result<Self, Error> {
        /* TODO: right now we do not use the endianness arg. We assume the DEX
         * file is little endian (as per the standard) but it might not always
         * be the case. */ 

        let mut raw_file = fs::File::open(file_path)
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
                _version: version,
                _checksum: checksum,
                _signature: signature,
                _file_size: file_size,
                _header_size: header_size,
                _endian_tag: endian_tag,
                _link_size: link_size,
                _link_off: link_off,
                _map_off: map_off,
                _string_ids_size: string_ids_size,
                _string_ids_off: string_ids_off,
                _type_ids_size: type_ids_size,
                _type_ids_off: type_ids_off,
                _proto_ids_size: proto_ids_size,
                _proto_ids_off: proto_ids_off,
                _fields_ids_size: fields_ids_size,
                _fields_ids_off: fields_ids_off,
                _method_ids_size: method_ids_size,
                _method_ids_off: method_ids_off,
                _class_defs_size: class_defs_size,
                _class_defs_off: class_defs_off,
                _data_size: data_size,
                _data_off:data_off
            })
    }

    fn print_version(&self) -> Result<&str, Error> {
        let s = match std::str::from_utf8(&self._version) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        println!("{:#}", &s);

        Ok(s)
    }

    fn print_checksum(&self) -> Result<String, Error> {
        let s = self._checksum.to_string();
        println!("{:#02X}", self._checksum);

        Ok(s)
    }

    fn get_checksum(&self) -> Result<u32, Error> {
        Ok(self._checksum)
    }
}

fn main() {
    // TODO: extract from the APK file given as argument
    let file_path = "classes.dex";

    let mut raw_file = fs::File::open(file_path)
                       .expect("Error: cannot open classes.dex");

    /* We first check the endianness. By standard it should be
     * little-endian but it is allowed to change to big-endian.
     */
    raw_file.seek(SeekFrom::Start(40))
            .expect("Error: cannot read endian tag");
    let mut endian_tag = [0; 4];
    let _ = raw_file.by_ref().take(4).read(&mut endian_tag);

    let endianness = match endian_tag {
        ENDIAN_CONSTANT => nom::number::Endianness::Big,
        REVERSE_ENDIAN_CONSTANT => nom::number::Endianness::Little,
        _ => panic!("Bad endian tag in header"),
    };

    let header = Header::from_path(file_path, endianness)
                 .expect("Error: cannot parse header");

    let checksum = header.get_checksum().expect("Error: cannot get checksum");
    println!("{:#02X}", checksum);
}
