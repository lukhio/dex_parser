use byteorder::{
    LittleEndian,
    ReadBytesExt
};
use std::env;
use std::fs;
use std::process::exit;
use std::io::{
    Read,
    Error,
};

mod adler32;
mod raw_types;
mod mutf8;

/* Endianness constants */
const ENDIAN_CONSTANT: (u8, u8, u8, u8) = (0x12, 0x34, 0x56, 0x78);
const REVERSE_ENDIAN_CONSTANT: (u8, u8, u8, u8) = (0x78, 0x56, 0x34, 0x12);

struct DexHeader {
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

impl DexHeader {
    fn from_raw_data(mut raw_data: &[u8], endianness: nom::number::Endianness) -> Result<Self, Error> {
        /* TODO: right now we do not use the endianness arg. We assume the DEX
         * file is little endian (as per the standard) but it might not always
         * be the case. It should be possible to use nom here, and call e.g., u32! */

        /* DEX version */
        let mut magic = [0; 8];
        let _ = raw_data.by_ref().take(8).read(&mut magic);
        let mut version = [0; 3];
        version[0] = magic[4];
        version[1] = magic[5];
        version[2] = magic[6];

        let checksum = raw_data.read_u32::<LittleEndian>().unwrap();
        println!("{:X?}", checksum);

        let mut signature = [0; 20];
        let _ = raw_data.by_ref().take(20).read(&mut signature);

        let file_size = raw_data.read_u32::<LittleEndian>().unwrap();
        let header_size = raw_data.read_u32::<LittleEndian>().unwrap();
        let endian_tag = raw_data.read_u32::<LittleEndian>().unwrap();

        let link_size = raw_data.read_u32::<LittleEndian>().unwrap();
        let link_off = raw_data.read_u32::<LittleEndian>().unwrap();
        let map_off = raw_data.read_u32::<LittleEndian>().unwrap();
        let string_ids_size = raw_data.read_u32::<LittleEndian>().unwrap();
        let string_ids_off = raw_data.read_u32::<LittleEndian>().unwrap();
        let type_ids_size = raw_data.read_u32::<LittleEndian>().unwrap();
        let type_ids_off = raw_data.read_u32::<LittleEndian>().unwrap();
        let proto_ids_size = raw_data.read_u32::<LittleEndian>().unwrap();
        let proto_ids_off = raw_data.read_u32::<LittleEndian>().unwrap();
        let fields_ids_size = raw_data.read_u32::<LittleEndian>().unwrap();
        let fields_ids_off = raw_data.read_u32::<LittleEndian>().unwrap();
        let method_ids_size = raw_data.read_u32::<LittleEndian>().unwrap();
        let method_ids_off = raw_data.read_u32::<LittleEndian>().unwrap();
        let class_defs_size = raw_data.read_u32::<LittleEndian>().unwrap();
        let class_defs_off = raw_data.read_u32::<LittleEndian>().unwrap();
        let data_size = raw_data.read_u32::<LittleEndian>().unwrap();
        let data_off = raw_data.read_u32::<LittleEndian>().unwrap();

        Ok(DexHeader {
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
}

#[allow(dead_code)]
struct HeaderMapItem {
    item_type: u16,
    size_entries: u32,
    size_bytes: u32,
    offset: u32
}

impl HeaderMapItem {
    fn build(item_type: u16, item_size: u32, item_offset: u32) -> Result<Self, Error> {
        Ok(HeaderMapItem {
            item_type: item_type,
            size_entries: item_size,
            size_bytes: 0,
            offset: item_offset
        })
    }
}

#[allow(dead_code)]
struct HeaderMap {
    size_entries: u32,
    size_bytes: u32,
    items: Vec<HeaderMapItem>
}

impl HeaderMap {
    fn build(map_size: u32, map_offset: u32, dex_file_size: u32) -> Result<Self, Error> {
        let map_bytes = dex_file_size - map_offset;

        Ok(HeaderMap {
            size_entries: map_size,
            size_bytes: map_bytes,
            items: Vec::new()
        })
    }

    fn add_item(&mut self, item_type: u16, item_size: u32, item_offset: u32) {
        let item = HeaderMapItem::build(item_type, item_size, item_offset)
                   .expect("Error: cannot create HeaderMapItem object");
        self.items.push(item);
    }

    fn compute_entries_size_bytes(&mut self) {
        println!("========== begin compute_entries_size_bytes ==========");
        let mut last_offset = 0;
        for mut entry in self.items.iter_mut() {
            match entry.item_type {
                raw_types::MapItemType::HEADER_ITEM => continue,
                raw_types::MapItemType::MAP_LIST => continue,
                _ => {
                    entry.size_bytes = entry.offset - last_offset;
                    last_offset = entry.offset;
                }
            };
        }

        for entry in self.items.iter() {
            println!("{:02X} | {:}", entry.item_type, entry.size_bytes);
        }
        println!("========== end compute_entries_size_bytes ==========");
    }
}

struct DexData {
    _strings: Vec<String>,
}

impl DexData {
    fn parse_string_data(mut strings_data: &[u8], item_size: u32) {
        println!("===================");
        println!("BEGIN Parse strings");
        println!("===================");

        let mut idx: u64 = 0;
        let mut string_idx = 0;
        while string_idx < item_size {
            /* Read string size, which is an unsigned LEB128 */
            let string_size = leb128::read::unsigned(&mut strings_data)
                              .expect("Error: cannot get string size");
            println!("{:} | {:}", idx, string_size);

            /* let s = match std::str::from_utf8(&strings_data[i as usize .. (i + string_size) as usize]) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };

            println!("result: {}", s); */

            let start: usize = idx as usize;
            let end: usize = (idx + string_size) as usize;

            println!("{:}", strings_data.len());
            for i in start..end {
                println!("{:}", strings_data[i]);
            }
            let s = &strings_data[start..end];
            // let s2 = &s.into_iter().map(|e| e as u32).copied().collect();
            println!("{:02X?}", s);
            println!("----------");
            let decoded = mutf8::decode(s);
            println!("{:}", decoded);

            /* Increment idx of its own size */
            idx += 4;


            idx += string_size;

            /* Increment strings index */
            string_idx += 1;
        }

        println!("===================");
        println!("END Parse strings");
        println!("===================");
    }
}

fn main() {
    /* Check CLI arguments */
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./{:} [APK]", args[0]);
        exit(22);
    }

    let apk_path = &args[1];
    /*
     * FIXME: this is just temporary, delete this when this seems to be working
     *
    println!("[+] Parsing {}", apk_path);

    let raw_file = fs::File::open(apk_path).expect("Error: cannot open APK");
    let mut zip = ZipArchive::new(raw_file).expect("Error: cannot create ZipArchive object");

    println!("[+] Loading classes.dex from the APK");

    /* TODO: support merging of multiple DEX files */
    let mut dex_entry = zip.by_name("classes.dex")
                        .expect("Error: cannot find classes.dex in the APK");
    */
    let mut dex_entry = fs::File::open(apk_path).expect("Error; cannot read file");
    let mut dex_buffer = Vec::new();
    dex_entry.read_to_end(&mut dex_buffer).expect("Error: cannot unzip classes.dex file");

    /* We first check the endianness. By standard it should be
     * little-endian but it is allowed to change to big-endian.
     */
    let endian_tag = &dex_buffer[40..44];

    let endianness = match (endian_tag[0], endian_tag[1], endian_tag[2], endian_tag[3]) {
        ENDIAN_CONSTANT => nom::number::Endianness::Big,
        REVERSE_ENDIAN_CONSTANT => nom::number::Endianness::Little,
        _ => panic!("Bad endian tag in header"),
    };

    println!("[+] Decoding DEX header");
    let header = DexHeader::from_raw_data(&dex_buffer, endianness)
                 .expect("Error: cannot parse header");

    // TODO: check after header is parsed, before going further
    println!("[+] Verifying Adler-32 checksum");
    let do_checksum_match = adler32::verify_from_raw_data(&dex_buffer, header._checksum)
                            .expect("Error: cannot verify checksum");
    if !do_checksum_match {
        panic!("Error: Adler32 checksum does not match");
    }

    println!("[+] Parsing header map");
    let mut map_data = &dex_buffer[header._map_off as usize ..];
    let map_size = map_data.read_u32::<LittleEndian>().unwrap();

    let mut header_map = HeaderMap::build(map_size, header._map_off, header._file_size)
                         .expect("Error: cannot build HeaderMap object");

    for _ in 0..map_size {
        /* Item type */
        let item_type = map_data.read_u16::<LittleEndian>().unwrap();
        /* Unused */
        let _ = map_data.read_u16::<LittleEndian>().unwrap();
        /* Item size (number of entries) */
        let item_size = map_data.read_u32::<LittleEndian>().unwrap();
        /* Item offset */
        let item_offset = map_data.read_u32::<LittleEndian>().unwrap();

        header_map.add_item(item_type, item_size, item_offset);
    }

    header_map.compute_entries_size_bytes();

    /* for _ in 0..map_size {
        let item_type = map_data.read_u16::<LittleEndian>().unwrap();
        let _ = map_data.read_u16::<LittleEndian>().unwrap();  // unused
        let item_size = map_data.read_u32::<LittleEndian>().unwrap();
        let item_offset = map_data.read_u32::<LittleEndian>().unwrap();

        header_map.add_item(item_type, item_size, item_offset);

        let start = item_offset as usize;
        let end = start + item_size as usize;
        println!("START {:?}", start);
        println!("END {:?}", end);

        match item_type {
            // raw_types::MapItemType::STRING_DATA_ITEM => DexData::parse_string_data(&dex_buffer[start..end], item_size),
            _ => {
                println!("===== Not implemented =====");
                println!("item_type {:02X}", item_type);
                println!("item_size {:02X}", item_size);
                println!("item_offset {:02X}", item_offset);
            }
        };*/
}
