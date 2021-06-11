use byteorder::{
    LittleEndian,
    ReadBytesExt
};
use std::io::{
    Error,
};
use std::collections::HashMap;

#[path = "opcodes.rs"]
mod opcodes;

/* Common traits */
/* Get bytes from stream and create an object for the corresponding instruction */
trait Parse {
    fn parse(&self, opcode: opcodes::OpCode, stream: &[u8]) -> Result<Self, Error> where Self: Sized;
}

/* Get length in bytes from instruction */
pub trait Length {
    fn get_length(&self) -> u8;
}

/* Unused for now */
/* pub enum Instruction {
    Instruction00x { length: u8 },
    Instruction21c {
        length: u8,
        op: u8,
        AA: u8,
        BBBB: u16
    }
}
pub trait Parse {
    fn parse(&self, opcode: u8, mut stream: &[u8]) -> Result<Self, Error>;
}

impl Instruction {
    pub fn parse(&self, opcode: u8, mut stream: &[u8]) -> Result<Self, Error> {
        match self {
            Instruction::Instruction21c { length, op, AA, BBBB } => {
                let length = 4;
                let AA = stream.read_u8().unwrap();
                let BBBB = stream.read_u16::<LittleEndian>().unwrap();

                Ok(Instruction::Instruction21c {
                    length: length,
                    op: opcode,
                    AA: AA,
                    BBBB: BBBB
                })
            },
            _ => panic!("AAAAAAHHH!!!1!"),
        }
    }
} */

pub struct Instruction {
    length: u8,
    opcode: opcodes::OpCode,
    values: HashMap<String, u64>,
}

#[allow(non_snake_case)]
impl Instruction {
    pub fn parse(raw_opcode: u8, mut stream: &[u8]) -> Result<Self, Error> {
        println!("==========");
        println!("Raw opcode: {:02X}", raw_opcode);
        let opcode = opcodes::OpCode::parse(raw_opcode).unwrap();
        println!("==========");
        match opcode {
            /* 10x format */
            opcodes::OpCode::NOP                    |
            opcodes::OpCode::RETURN_VOID            => {
                Ok(Instruction {
                    length: 1,
                    opcode: opcode,
                    values: HashMap::<String, u64>::new(),
                })
            }

            /* 12x format */
            opcodes::OpCode::MOVE                   |
            opcodes::OpCode::MOVE_WIDE              |
            opcodes::OpCode::MOVE_OBJECT            |
            opcodes::OpCode::ARRAY_LENGTH           |
            opcodes::OpCode::NEG_INT                |
            opcodes::OpCode::NOT_INT                |
            opcodes::OpCode::NEG_LONG               |
            opcodes::OpCode::NOT_LONG               |
            opcodes::OpCode::NEG_FLOAT              |
            opcodes::OpCode::NEG_DOUBLE             |
            opcodes::OpCode::INT_TO_LONG            |
            opcodes::OpCode::INT_TO_FLOAT           |
            opcodes::OpCode::INT_TO_DOUBLE          |
            opcodes::OpCode::LONG_TO_INT            |
            opcodes::OpCode::LONG_TO_FLOAT          |
            opcodes::OpCode::LONG_TO_DOUBLE         |
            opcodes::OpCode::FLOAT_TO_INT           |
            opcodes::OpCode::FLOAT_TO_LONG          |
            opcodes::OpCode::FLOAT_TO_DOUBLE        |
            opcodes::OpCode::DOUBLE_TO_INT          |
            opcodes::OpCode::DOUBLE_TO_LONG         |
            opcodes::OpCode::DOUBLE_TO_FLOAT        |
            opcodes::OpCode::INT_TO_BYTE            |
            opcodes::OpCode::INT_TO_CHAR            |
            opcodes::OpCode::INT_TO_SHORT           |
            opcodes::OpCode::ADD_INT_2ADDR          |
            opcodes::OpCode::SUB_INT_2ADDR          |
            opcodes::OpCode::MUL_INT_2ADDR          |
            opcodes::OpCode::DIV_INT_2ADDR          |
            opcodes::OpCode::REM_INT_2ADDR          |
            opcodes::OpCode::AND_INT_2ADDR          |
            opcodes::OpCode::OR_INT_2ADDR           |
            opcodes::OpCode::XOR_INT_2ADDR          |
            opcodes::OpCode::SHL_INT_2ADDR          |
            opcodes::OpCode::SHR_INT_2ADDR          |
            opcodes::OpCode::USHR_INT_2ADDR         |
            opcodes::OpCode::ADD_LONG_2ADDR         |
            opcodes::OpCode::SUB_LONG_2ADDR         |
            opcodes::OpCode::MUL_LONG_2ADDR         |
            opcodes::OpCode::DIV_LONG_2ADDR         |
            opcodes::OpCode::REM_LONG_2ADDR         |
            opcodes::OpCode::AND_LONG_2ADDR         |
            opcodes::OpCode::OR_LONG_2ADDR          |
            opcodes::OpCode::XOR_LONG_2ADDR         |
            opcodes::OpCode::SHL_LONG_2ADDR         |
            opcodes::OpCode::SHR_LONG_2ADDR         |
            opcodes::OpCode::USHR_LONG_2ADDR        |
            opcodes::OpCode::ADD_FLOAT_2ADDR        |
            opcodes::OpCode::SUB_FLOAT_2ADDR        |
            opcodes::OpCode::MUL_FLOAT_2ADDR        |
            opcodes::OpCode::DIV_FLOAT_2ADDR        |
            opcodes::OpCode::REM_FLOAT_2ADDR        |
            opcodes::OpCode::ADD_DOUBLE_2ADDR       |
            opcodes::OpCode::SUB_DOUBLE_2ADDR       |
            opcodes::OpCode::MUL_DOUBLE_2ADDR       |
            opcodes::OpCode::DIV_DOUBLE_2ADDR       |
            opcodes::OpCode::REM_DOUBLE_2ADDR       => {
                let length = 1;
                let val = stream.read_u8().unwrap();
                let vA = val & 0x0f;
                let vB = (val & 0xf0) >> 4;

                let mut values = HashMap::<String, u64>::new();
                values.insert(String::from("vA"), vA.into());
                values.insert(String::from("vB"), vB.into());

                Ok(Instruction {
                    length: length,
                    opcode: opcode,
                    values: values,
                })
            }

            /* 11n format */
            opcodes::OpCode::CONST_4                => {
                let length = 1;
                let val = stream.read_u8().unwrap();
                let vA = val & 0x0f;
                let B = (val & 0xf0) >> 4;

                let mut values = HashMap::<String, u64>::new();
                values.insert(String::from("vA"), vA.into());
                values.insert(String::from("B"), B.into());

                Ok(Instruction {
                    length: length,
                    opcode: opcode,
                    values: values,
                })
            }

            /* 11x format */
            opcodes::OpCode::MOVE_RESULT            |
            opcodes::OpCode::MOVE_RESULT_WIDE       |
            opcodes::OpCode::MOVE_RESULT_OBJECT     |
            opcodes::OpCode::MOVE_EXCEPTION         |
            opcodes::OpCode::RETURN                 |
            opcodes::OpCode::RETURN_WIDE            |
            opcodes::OpCode::RETURN_OBJECT          |
            opcodes::OpCode::MONITOR_ENTER          |
            opcodes::OpCode::MONITOR_EXIT           |
            opcodes::OpCode::THROW                  => {
                let length = 1;
                let vAA = stream.read_u8().unwrap();

                let mut values = HashMap::<String, u64>::new();
                values.insert(String::from("vAA"), vAA.into());

                Ok(Instruction {
                    length: length,
                    opcode: opcode,
                    values: values,
                })
            }

            /* 10t format */
            opcodes::OpCode::GOTO                   => {
                let length = 1;
                let AA = stream.read_u8().unwrap();

                let mut values = HashMap::<String, u64>::new();
                values.insert(String::from("AA"), AA.into());

                Ok(Instruction {
                    length: length,
                    opcode: opcode,
                    values: values,
                })
            }

            /* 20t format */
            opcodes::OpCode::GOTO_16                => {
                let length = 2;
                let AAAA = stream.read_u16::<LittleEndian>().unwrap();

                let mut values = HashMap::<String, u64>::new();
                values.insert(String::from("AAAA"), AAAA.into());

                Ok(Instruction {
                    length: length,
                    opcode: opcode,
                    values: values,
                })
            }

            /* 20bc format --- no opcode with this format */

            /* 22x format */
            opcodes::OpCode::MOVE_FROM16            |
            opcodes::OpCode::MOVE_WIDE_FROM16       |
            opcodes::OpCode::MOVE_OBJECT_FROM16     => {
                let length = 2;
                let vAA = stream.read_u8().unwrap();
                let BBBB = stream.read_u16::<LittleEndian>().unwrap();

                let mut values = HashMap::<String, u64>::new();
                values.insert(String::from("vAA"), vAA.into());
                values.insert(String::from("BBBB"), BBBB.into());

                Ok(Instruction {
                    length: length,
                    opcode: opcode,
                    values: values,
                })
            }

            /* 21t format */
            opcodes::OpCode::IF_EQZ                 |
            opcodes::OpCode::IF_NEZ                 |
            opcodes::OpCode::IF_LTZ                 |
            opcodes::OpCode::IF_GEZ                 |
            opcodes::OpCode::IF_GTZ                 |
            opcodes::OpCode::IF_LEZ                 => {
                let length = 2;
                let vAA = stream.read_u8().unwrap();
                let BBBB = stream.read_u16::<LittleEndian>().unwrap();

                let mut values = HashMap::<String, u64>::new();
                values.insert(String::from("vAA"), vAA.into());
                values.insert(String::from("BBBB"), BBBB.into());

                Ok(Instruction {
                    length: length,
                    opcode: opcode,
                    values: values,
                })
            }

            /* 21s format */
            opcodes::OpCode::CONST_16               |
            opcodes::OpCode::CONST_WIDE_16          => {
                let length = 2;
                let vAA = stream.read_u8().unwrap();
                let BBBB = stream.read_u16::<LittleEndian>().unwrap();

                let mut values = HashMap::<String, u64>::new();
                values.insert(String::from("vAA"), vAA.into());
                values.insert(String::from("BBBB"), BBBB.into());

                Ok(Instruction {
                    length: length,
                    opcode: opcode,
                    values: values,
                })
            }

            /* TODO: these two next instructions deal with signed integers, but the HashMap to hold
             * the values uses unsigned integers. For now I don't convert them but this needs to be
             * fixed. */

            /* 21h format --- format one
             * There are two possiblity here, so we have two arms */
            opcodes::OpCode::CONST_HIGH16           => {
                let length = 2;
                let vAA = stream.read_u8().unwrap();
                let mut BBBB0000: u32 = stream.read_u16::<LittleEndian>().unwrap().into();
                BBBB0000 <<= 16;

                let mut values = HashMap::<String, u64>::new();
                values.insert(String::from("vAA"), vAA.into());
                values.insert(String::from("BBBB0000"), BBBB0000.into());

                Ok(Instruction {
                    length: length,
                    opcode: opcode,
                    values: values,
                })
            }

            /* 21h format --- format two */
            opcodes::OpCode::CONST_WIDE_HIGH16      => {
                let length = 2;
                let vAA = stream.read_u8().unwrap();
                let mut BBBB000000000000: u64 = stream.read_u16::<LittleEndian>().unwrap().into();
                BBBB000000000000 <<= 48;

                let mut values = HashMap::<String, u64>::new();
                values.insert(String::from("vAA"), vAA.into());
                values.insert(String::from("BBBB000000000000"), BBBB000000000000.into());

                Ok(Instruction {
                    length: length,
                    opcode: opcode,
                    values: values,
                })
            }

            /* 21c format */
            opcodes::OpCode::CONST_STRING           |
            opcodes::OpCode::CONST_CLASS            |
            opcodes::OpCode::CHECK_CAST             |
            opcodes::OpCode::NEW_INSTANCE           |
            opcodes::OpCode::SGET                   |
            opcodes::OpCode::SGET_WIDE              |
            opcodes::OpCode::SGET_OBJECT            |
            opcodes::OpCode::SGET_BOOLEAN           |
            opcodes::OpCode::SGET_BYTE              |
            opcodes::OpCode::SGET_CHAR              |
            opcodes::OpCode::SGET_SHORT             |
            opcodes::OpCode::SPUT                   |
            opcodes::OpCode::SPUT_WIDE              |
            opcodes::OpCode::SPUT_OBJECT            |
            opcodes::OpCode::SPUT_BOOLEAN           |
            opcodes::OpCode::SPUT_BYTE              |
            opcodes::OpCode::SPUT_CHAR              |
            opcodes::OpCode::SPUT_SHORT             |
            opcodes::OpCode::CONST_METHOD_HANDLE    |
            opcodes::OpCode::CONST_METHOD_TYPE      => {
                let length = 2;
                let AA = stream.read_u8().unwrap();
                let BBBB = stream.read_u16::<LittleEndian>().unwrap();

                let mut values = HashMap::<String, u64>::new();
                values.insert(String::from("AA"), AA.into());
                values.insert(String::from("BBBB"), BBBB.into());

                Ok(Instruction {
                    length: length,
                    opcode: opcode,
                    values: values,
                })
            },
            _ => panic!("AAAAAAHHH!!!1!"),
        }
    }

    pub fn get_length(&self) -> u8 {
        return self.length;
    }

    pub fn get_values(&self) -> HashMap::<String, u64> {
        return self.values.clone();
    }
}
