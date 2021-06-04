use std::io::Error;

#[allow(non_camel_case_types)]
pub enum OpCode {
    NOP,
    MOVE,
    MOVE_FROM16,
    MOVE_16,
    MOVE_WIDE,
    MOVE_WIDE_FROM16,
    MOVE_WIDE_16,
    MOVE_OBJECT,
    MOVE_OBJECT_FROM16,
    MOVE_OBJECT_16,
    MOVE_RESULT,
    MOVE_RESULT_WIDE,
    MOVE_RESULT_OBJECT,
    MOVE_EXCEPTION,
    RETURN_VOID,
    RETURN,
    RETURN_WIDE,
    RETURN_OBJECT,
    CONST_4,
    CONST_16,
    CONST,
    CONST_HIGH16,
    CONST_WIDE_16,
    CONST_WIDE_32,
    CONST_WIDE,
    CONST_WIDE_HIGH16,
    CONST_STRING,
    CONST_STRING_JUMBO,
    CONST_CLASS,
    MONITOR_ENTER,
    MONITOR_EXIT,
    CHECK_CAST,
    INSTANCE_OF,
    ARRAY_LENGTH,
    NEW_INSTANCE,
    NEW_ARRAY,
    FILLED_NEW_ARRAY,
    FILLED_NEW_ARRAY_RANGE,
    FILL_ARRAY_DATA,
    THROW,
    GOTO,
    GOTO_16,
    GOTO_32,
    PACKED_SWITCH,
    SPARSE_SWITCH,
    CMPL_FLOAT,
    CMPG_FLOAT,
    CMPL_DOUBLE,
    CMPG_DOUBLE,
    CMP_LONG,
    IF_EQ,
    IF_NE,
    IF_LT,
    IF_GE,
    IF_GT,
    IF_LE,
    IF_EQZ,
    IF_NEZ,
    IF_LTZ,
    IF_GEZ,
    IF_GTZ,
    IF_LEZ,
    AGET,
    AGET_WIDE,
    AGET_OBJECT,
    AGET_BOOLEAN,
    AGET_BYTE,
    AGET_CHAR,
    AGET_SHORT,
    APUT,
    APUT_WIDE,
    APUT_OBJECT,
    APUT_BOOLEAN,
    APUT_BYTE,
    APUT_CHAR,
    APUT_SHORT,
    IGET,
    IGET_WIDE,
    IGET_OBJECT,
    IGET_BOOLEAN,
    IGET_BYTE,
    IGET_CHAR,
    IGET_SHORT,
    IPUT,
    IPUT_WIDE,
    IPUT_OBJECT,
    IPUT_BOOLEAN,
    IPUT_BYTE,
    IPUT_CHAR,
    IPUT_SHORT,
    SGET,
    SGET_WIDE,
    SGET_OBJECT,
    SGET_BOOLEAN,
    SGET_BYTE,
    SGET_CHAR,
    SGET_SHORT,
    SPUT,
    SPUT_WIDE,
    SPUT_OBJECT,
    SPUT_BOOLEAN,
    SPUT_BYTE,
    SPUT_CHAR,
    SPUT_SHORT,
    INVOKE_VIRTUAL,
    INVOKE_SUPER,
    INVOKE_DIRECT,
    INVOKE_STATIC,
    INVOKE_INTERFACE,
    INVOKE_VIRTUAL_RANGE,
    INVOKE_SUPER_RANGE,
    INVOKE_DIRECT_RANGE,
    INVOKE_STATIC_RANGE,
    INVOKE_INTERFACE_RANGE,
    NEG_INT,
    NOT_INT,
    NEG_LONG,
    NOT_LONG,
    NEG_FLOAT,
    NEG_DOUBLE,
    INT_TO_LONG,
    INT_TO_FLOAT,
    INT_TO_DOUBLE,
    LONG_TO_INT,
    LONG_TO_FLOAT,
    LONG_TO_DOUBLE,
    FLOAT_TO_INT,
    FLOAT_TO_LONG,
    FLOAT_TO_DOUBLE,
    DOUBLE_TO_INT,
    DOUBLE_TO_LONG,
    DOUBLE_TO_FLOAT,
    INT_TO_BYTE,
    INT_TO_CHAR,
    INT_TO_SHORT,
    ADD_INT,
    SUB_INT,
    MUL_INT,
    DIV_INT,
    REM_INT,
    AND_INT,
    OR_INT,
    XOR_INT,
    SHL_INT,
    SHR_INT,
    USHR_INT,
    ADD_LONG,
    SUB_LONG,
    MUL_LONG,
    DIV_LONG,
    REM_LONG,
    AND_LONG,
    OR_LONG,
    XOR_LONG,
    SHL_LONG,
    SHR_LONG,
    USHR_LONG,
    ADD_FLOAT,
    SUB_FLOAT,
    MUL_FLOAT,
    DIV_FLOAT,
    REM_FLOAT,
    ADD_DOUBLE,
    SUB_DOUBLE,
    MUL_DOUBLE,
    DIV_DOUBLE,
    REM_DOUBLE,
    ADD_INT_2ADDR,
    SUB_INT_2ADDR,
    MUL_INT_2ADDR,
    DIV_INT_2ADDR,
    REM_INT_2ADDR,
    AND_INT_2ADDR,
    OR_INT_2ADDR,
    XOR_INT_2ADDR,
    SHL_INT_2ADDR,
    SHR_INT_2ADDR,
    USHR_INT_2ADDR,
    ADD_LONG_2ADDR,
    SUB_LONG_2ADDR,
    MUL_LONG_2ADDR,
    DIV_LONG_2ADDR,
    REM_LONG_2ADDR,
    AND_LONG_2ADDR,
    OR_LONG_2ADDR,
    XOR_LONG_2ADDR,
    SHL_LONG_2ADDR,
    SHR_LONG_2ADDR,
    USHR_LONG_2ADDR,
    ADD_FLOAT_2ADDR,
    SUB_FLOAT_2ADDR,
    MUL_FLOAT_2ADDR,
    DIV_FLOAT_2ADDR,
    REM_FLOAT_2ADDR,
    ADD_DOUBLE_2ADDR,
    SUB_DOUBLE_2ADDR,
    MUL_DOUBLE_2ADDR,
    DIV_DOUBLE_2ADDR,
    REM_DOUBLE_2ADDR,
    ADD_INT_LIT16,
    RSUB_INT,
    MUL_INT_LIT16,
    DIV_INT_LIT16,
    REM_INT_LIT16,
    AND_INT_LIT16,
    OR_INT_LIT16,
    XOR_INT_LIT16,
    ADD_INT_LIT8,
    RSUB_INT_LIT8,
    MUL_INT_LIT8,
    DIV_INT_LIT8,
    REM_INT_LIT8,
    AND_INT_LIT8,
    OR_INT_LIT8,
    XOR_INT_LIT8,
    SHL_INT_LIT8,
    SHR_INT_LIT8,
    USHR_INT_LIT8,
    INVOKE_POLYMORPHIC,
    INVOKE_POLYMORPHIC_RANGE,
    INVOKE_CUSTOM,
    INVOKE_CUSTOM_RANGE,
    CONST_METHOD_HANDLE,
    CONST_METHOD_TYPE,
}

impl OpCode {
    pub fn parse(value: u8) -> Result<Self, Error> {
        match value {
            0x00 => Ok(OpCode::NOP),                        // Instruction 10x
            0x01 => Ok(OpCode::MOVE),                       // Instruction 12x
            0x02 => Ok(OpCode::MOVE_FROM16),                // Instruction 22x
            0x03 => Ok(OpCode::MOVE_16),                    // Instruction 32x
            0x04 => Ok(OpCode::MOVE_WIDE),                  // Instruction 12x
            0x05 => Ok(OpCode::MOVE_WIDE_FROM16),           // Instruction 22x
            0x06 => Ok(OpCode::MOVE_WIDE_16),               // Instruction 32x
            0x07 => Ok(OpCode::MOVE_OBJECT),                // Instruction 12x
            0x08 => Ok(OpCode::MOVE_OBJECT_FROM16),         // Instruction 22x
            0x09 => Ok(OpCode::MOVE_OBJECT_16),             // Instruction 32x
            0x0a => Ok(OpCode::MOVE_RESULT),                // Instruction 11x
            0x0b => Ok(OpCode::MOVE_RESULT_WIDE),           // Instruction 11x
            0x0c => Ok(OpCode::MOVE_RESULT_OBJECT),         // Instruction 11x
            0x0d => Ok(OpCode::MOVE_EXCEPTION),             // Instruction 11x
            0x0e => Ok(OpCode::RETURN_VOID),                // Instruction 10x
            0x0f => Ok(OpCode::RETURN),                     // Instruction 11x
            0x10 => Ok(OpCode::RETURN_WIDE),                // Instruction 11x
            0x11 => Ok(OpCode::RETURN_OBJECT),              // Instruction 11x
            0x12 => Ok(OpCode::CONST_4),                    // Instruction 11n
            0x13 => Ok(OpCode::CONST_16),                   // Instruction 21s
            0x14 => Ok(OpCode::CONST),                      // Instruction 31i
            0x15 => Ok(OpCode::CONST_HIGH16),               // Instruction 21h
            0x16 => Ok(OpCode::CONST_WIDE_16),              // Instruction 21s
            0x17 => Ok(OpCode::CONST_WIDE_32),              // Instruction 31i
            0x18 => Ok(OpCode::CONST_WIDE),                 // Instruction 51l
            0x19 => Ok(OpCode::CONST_WIDE_HIGH16),          // Instruction 21h
            0x1a => Ok(OpCode::CONST_STRING),               // Instruction 21c
            0x1b => Ok(OpCode::CONST_STRING_JUMBO),         // Instruction 31c
            0x1c => Ok(OpCode::CONST_CLASS),                // Instruction 21c
            0x1d => Ok(OpCode::MONITOR_ENTER),              // Instruction 11x
            0x1e => Ok(OpCode::MONITOR_EXIT),               // Instruction 11x
            0x1f => Ok(OpCode::CHECK_CAST),                 // Instruction 21c
            0x20 => Ok(OpCode::INSTANCE_OF),                // Instruction 22c
            0x21 => Ok(OpCode::ARRAY_LENGTH),               // Instruction 12x
            0x22 => Ok(OpCode::NEW_INSTANCE),               // Instruction 21c
            0x23 => Ok(OpCode::NEW_ARRAY),                  // Instruction 22c
            0x24 => Ok(OpCode::FILLED_NEW_ARRAY),           // Instruction 35c
            0x25 => Ok(OpCode::FILLED_NEW_ARRAY_RANGE),     // Instruction 3rc
            0x26 => Ok(OpCode::FILL_ARRAY_DATA),            // Instruction 31t
            0x27 => Ok(OpCode::THROW),                      // Instruction 11x
            0x28 => Ok(OpCode::GOTO),                       // Instruction 10t
            0x29 => Ok(OpCode::GOTO_16),                    // Instruction 20t
            0x2a => Ok(OpCode::GOTO_32),                    // Instruction 30t
            0x2b => Ok(OpCode::PACKED_SWITCH),              // Instruction 31t
            0x2c => Ok(OpCode::SPARSE_SWITCH),              // Instruction 31t
            0x2d => Ok(OpCode::CMPL_FLOAT),                 // Instruction 23x
            0x2e => Ok(OpCode::CMPG_FLOAT),                 // Instruction 23x
            0x2f => Ok(OpCode::CMPL_DOUBLE),                // Instruction 23x
            0x30 => Ok(OpCode::CMPG_DOUBLE),                // Instruction 23x
            0x31 => Ok(OpCode::CMP_LONG),                   // Instruction 23x
            0x32 => Ok(OpCode::IF_EQ),                      // Instruction 22t
            0x33 => Ok(OpCode::IF_NE),                      // Instruction 22t
            0x34 => Ok(OpCode::IF_LT),                      // Instruction 22t
            0x35 => Ok(OpCode::IF_GE),                      // Instruction 22t
            0x36 => Ok(OpCode::IF_GT),                      // Instruction 22t
            0x37 => Ok(OpCode::IF_LE),                      // Instruction 22t
            0x38 => Ok(OpCode::IF_EQZ),                     // Instruction 21t
            0x39 => Ok(OpCode::IF_NEZ),                     // Instruction 21t
            0x3a => Ok(OpCode::IF_LTZ),                     // Instruction 21t
            0x3b => Ok(OpCode::IF_GEZ),                     // Instruction 21t
            0x3c => Ok(OpCode::IF_GTZ),                     // Instruction 21t
            0x3d => Ok(OpCode::IF_LEZ),                     // Instruction 21t

            /* Unused */
            0x3e => panic!("Warning: use of undefined opcode {}", value),
            0x3f => panic!("Warning: use of undefined opcode {}", value),
            0x40 => panic!("Warning: use of undefined opcode {}", value),
            0x41 => panic!("Warning: use of undefined opcode {}", value),
            0x42 => panic!("Warning: use of undefined opcode {}", value),
            0x43 => panic!("Warning: use of undefined opcode {}", value),

            0x44 => Ok(OpCode::AGET),                       // Instruction 23x
            0x45 => Ok(OpCode::AGET_WIDE),                  // Instruction 23x
            0x46 => Ok(OpCode::AGET_OBJECT),                // Instruction 23x
            0x47 => Ok(OpCode::AGET_BOOLEAN),               // Instruction 23x
            0x48 => Ok(OpCode::AGET_BYTE),                  // Instruction 23x
            0x49 => Ok(OpCode::AGET_CHAR),                  // Instruction 23x
            0x4a => Ok(OpCode::AGET_SHORT),                 // Instruction 23x
            0x4b => Ok(OpCode::APUT),                       // Instruction 23x
            0x4c => Ok(OpCode::APUT_WIDE),                  // Instruction 23x
            0x4d => Ok(OpCode::APUT_OBJECT),                // Instruction 23x
            0x4e => Ok(OpCode::APUT_BOOLEAN),               // Instruction 23x
            0x4f => Ok(OpCode::APUT_BYTE),                  // Instruction 23x
            0x50 => Ok(OpCode::APUT_CHAR),                  // Instruction 23x
            0x51 => Ok(OpCode::APUT_SHORT),                 // Instruction 23x
            0x52 => Ok(OpCode::IGET),                       // Instruction 22c
            0x53 => Ok(OpCode::IGET_WIDE),                  // Instruction 22c
            0x54 => Ok(OpCode::IGET_OBJECT),                // Instruction 22c
            0x55 => Ok(OpCode::IGET_BOOLEAN),               // Instruction 22c
            0x56 => Ok(OpCode::IGET_BYTE),                  // Instruction 22c
            0x57 => Ok(OpCode::IGET_CHAR),                  // Instruction 22c
            0x58 => Ok(OpCode::IGET_SHORT),                 // Instruction 22c
            0x59 => Ok(OpCode::IPUT),                       // Instruction 22c
            0x5a => Ok(OpCode::IPUT_WIDE),                  // Instruction 22c
            0x5b => Ok(OpCode::IPUT_OBJECT),                // Instruction 22c
            0x5c => Ok(OpCode::IPUT_BOOLEAN),               // Instruction 22c
            0x5d => Ok(OpCode::IPUT_BYTE),                  // Instruction 22c
            0x5e => Ok(OpCode::IPUT_CHAR),                  // Instruction 22c
            0x5f => Ok(OpCode::IPUT_SHORT),                 // Instruction 22c
            0x60 => Ok(OpCode::SGET),                       // Instruction 21c
            0x61 => Ok(OpCode::SGET_WIDE),                  // Instruction 21c
            0x62 => Ok(OpCode::SGET_OBJECT),                // Instruction 21c
            0x63 => Ok(OpCode::SGET_BOOLEAN),               // Instruction 21c
            0x64 => Ok(OpCode::SGET_BYTE),                  // Instruction 21c
            0x65 => Ok(OpCode::SGET_CHAR),                  // Instruction 21c
            0x66 => Ok(OpCode::SGET_SHORT),                 // Instruction 21c
            0x67 => Ok(OpCode::SPUT),                       // Instruction 21c
            0x68 => Ok(OpCode::SPUT_WIDE),                  // Instruction 21c
            0x69 => Ok(OpCode::SPUT_OBJECT),                // Instruction 21c
            0x6a => Ok(OpCode::SPUT_BOOLEAN),               // Instruction 21c
            0x6b => Ok(OpCode::SPUT_BYTE),                  // Instruction 21c
            0x6c => Ok(OpCode::SPUT_CHAR),                  // Instruction 21c
            0x6d => Ok(OpCode::SPUT_SHORT),                 // Instruction 21c
            0x6e => Ok(OpCode::INVOKE_VIRTUAL),             // Instruction 35c
            0x6f => Ok(OpCode::INVOKE_SUPER),               // Instruction 35c
            0x70 => Ok(OpCode::INVOKE_DIRECT),              // Instruction 35c
            0x71 => Ok(OpCode::INVOKE_STATIC),              // Instruction 35c
            0x72 => Ok(OpCode::INVOKE_INTERFACE),           // Instruction 35c

            /* Unused */
            0x73 => panic!("Warning: use of undefined opcode {}", value),

            0x74 => Ok(OpCode::INVOKE_VIRTUAL_RANGE),       // Instruction 3rc
            0x75 => Ok(OpCode::INVOKE_SUPER_RANGE),         // Instruction 3rc
            0x76 => Ok(OpCode::INVOKE_DIRECT_RANGE),        // Instruction 3rc
            0x77 => Ok(OpCode::INVOKE_STATIC_RANGE),        // Instruction 3rc
            0x78 => Ok(OpCode::INVOKE_INTERFACE_RANGE),     // Instruction 3rc

            /* Unused, we panic for now but we should probably just log the error and carry on */
            0x79 => panic!("Warning: use of undefined opcode {}", value),
            0x7a => panic!("Warning: use of undefined opcode {}", value),

            0x7b => Ok(OpCode::NEG_INT),                    // Instruction 12x
            0x7c => Ok(OpCode::NOT_INT),                    // Instruction 12x
            0x7d => Ok(OpCode::NEG_LONG),                   // Instruction 12x
            0x7e => Ok(OpCode::NOT_LONG),                   // Instruction 12x
            0x7f => Ok(OpCode::NEG_FLOAT),                  // Instruction 12x
            0x80 => Ok(OpCode::NEG_DOUBLE),                 // Instruction 12x
            0x81 => Ok(OpCode::INT_TO_LONG),                // Instruction 12x
            0x82 => Ok(OpCode::INT_TO_FLOAT),               // Instruction 12x
            0x83 => Ok(OpCode::INT_TO_DOUBLE),              // Instruction 12x
            0x84 => Ok(OpCode::LONG_TO_INT),                // Instruction 12x
            0x85 => Ok(OpCode::LONG_TO_FLOAT),              // Instruction 12x
            0x86 => Ok(OpCode::LONG_TO_DOUBLE),             // Instruction 12x
            0x87 => Ok(OpCode::FLOAT_TO_INT),               // Instruction 12x
            0x88 => Ok(OpCode::FLOAT_TO_LONG),              // Instruction 12x
            0x89 => Ok(OpCode::FLOAT_TO_DOUBLE),            // Instruction 12x
            0x8a => Ok(OpCode::DOUBLE_TO_INT),              // Instruction 12x
            0x8b => Ok(OpCode::DOUBLE_TO_LONG),             // Instruction 12x
            0x8c => Ok(OpCode::DOUBLE_TO_FLOAT),            // Instruction 12x
            0x8d => Ok(OpCode::INT_TO_BYTE),                // Instruction 12x
            0x8e => Ok(OpCode::INT_TO_CHAR),                // Instruction 12x
            0x8f => Ok(OpCode::INT_TO_SHORT),               // Instruction 12x
            0x90 => Ok(OpCode::ADD_INT),                    // Instruction 23x
            0x91 => Ok(OpCode::SUB_INT),                    // Instruction 23x
            0x92 => Ok(OpCode::MUL_INT),                    // Instruction 23x
            0x93 => Ok(OpCode::DIV_INT),                    // Instruction 23x
            0x94 => Ok(OpCode::REM_INT),                    // Instruction 23x
            0x95 => Ok(OpCode::AND_INT),                    // Instruction 23x
            0x96 => Ok(OpCode::OR_INT),                     // Instruction 23x
            0x97 => Ok(OpCode::XOR_INT),                    // Instruction 23x
            0x98 => Ok(OpCode::SHL_INT),                    // Instruction 23x
            0x99 => Ok(OpCode::SHR_INT),                    // Instruction 23x
            0x9a => Ok(OpCode::USHR_INT),                   // Instruction 23x
            0x9b => Ok(OpCode::ADD_LONG),                   // Instruction 23x
            0x9c => Ok(OpCode::SUB_LONG),                   // Instruction 23x
            0x9d => Ok(OpCode::MUL_LONG),                   // Instruction 23x
            0x9e => Ok(OpCode::DIV_LONG),                   // Instruction 23x
            0x9f => Ok(OpCode::REM_LONG),                   // Instruction 23x
            0xa0 => Ok(OpCode::AND_LONG),                   // Instruction 23x
            0xa1 => Ok(OpCode::OR_LONG),                    // Instruction 23x
            0xa2 => Ok(OpCode::XOR_LONG),                   // Instruction 23x
            0xa3 => Ok(OpCode::SHL_LONG),                   // Instruction 23x
            0xa4 => Ok(OpCode::SHR_LONG),                   // Instruction 23x
            0xa5 => Ok(OpCode::USHR_LONG),                  // Instruction 23x
            0xa6 => Ok(OpCode::ADD_FLOAT),                  // Instruction 23x
            0xa7 => Ok(OpCode::SUB_FLOAT),                  // Instruction 23x
            0xa8 => Ok(OpCode::MUL_FLOAT),                  // Instruction 23x
            0xa9 => Ok(OpCode::DIV_FLOAT),                  // Instruction 23x
            0xaa => Ok(OpCode::REM_FLOAT),                  // Instruction 23x
            0xab => Ok(OpCode::ADD_DOUBLE),                 // Instruction 23x
            0xac => Ok(OpCode::SUB_DOUBLE),                 // Instruction 23x
            0xad => Ok(OpCode::MUL_DOUBLE),                 // Instruction 23x
            0xae => Ok(OpCode::DIV_DOUBLE),                 // Instruction 23x
            0xaf => Ok(OpCode::REM_DOUBLE),                 // Instruction 23x
            0xb0 => Ok(OpCode::ADD_INT_2ADDR),              // Instruction 12x
            0xb1 => Ok(OpCode::SUB_INT_2ADDR),              // Instruction 12x
            0xb2 => Ok(OpCode::MUL_INT_2ADDR),              // Instruction 12x
            0xb3 => Ok(OpCode::DIV_INT_2ADDR),              // Instruction 12x
            0xb4 => Ok(OpCode::REM_INT_2ADDR),              // Instruction 12x
            0xb5 => Ok(OpCode::AND_INT_2ADDR),              // Instruction 12x
            0xb6 => Ok(OpCode::OR_INT_2ADDR),               // Instruction 12x
            0xb7 => Ok(OpCode::XOR_INT_2ADDR),              // Instruction 12x
            0xb8 => Ok(OpCode::SHL_INT_2ADDR),              // Instruction 12x
            0xb9 => Ok(OpCode::SHR_INT_2ADDR),              // Instruction 12x
            0xba => Ok(OpCode::USHR_INT_2ADDR),             // Instruction 12x
            0xbb => Ok(OpCode::ADD_LONG_2ADDR),             // Instruction 12x
            0xbc => Ok(OpCode::SUB_LONG_2ADDR),             // Instruction 12x
            0xbd => Ok(OpCode::MUL_LONG_2ADDR),             // Instruction 12x
            0xbe => Ok(OpCode::DIV_LONG_2ADDR),             // Instruction 12x
            0xbf => Ok(OpCode::REM_LONG_2ADDR),             // Instruction 12x
            0xc0 => Ok(OpCode::AND_LONG_2ADDR),             // Instruction 12x
            0xc1 => Ok(OpCode::OR_LONG_2ADDR),              // Instruction 12x
            0xc2 => Ok(OpCode::XOR_LONG_2ADDR),             // Instruction 12x
            0xc3 => Ok(OpCode::SHL_LONG_2ADDR),             // Instruction 12x
            0xc4 => Ok(OpCode::SHR_LONG_2ADDR),             // Instruction 12x
            0xc5 => Ok(OpCode::USHR_LONG_2ADDR),            // Instruction 12x
            0xc6 => Ok(OpCode::ADD_FLOAT_2ADDR),            // Instruction 12x
            0xc7 => Ok(OpCode::SUB_FLOAT_2ADDR),            // Instruction 12x
            0xc8 => Ok(OpCode::MUL_FLOAT_2ADDR),            // Instruction 12x
            0xc9 => Ok(OpCode::DIV_FLOAT_2ADDR),            // Instruction 12x
            0xca => Ok(OpCode::REM_FLOAT_2ADDR),            // Instruction 12x
            0xcb => Ok(OpCode::ADD_DOUBLE_2ADDR),           // Instruction 12x
            0xcc => Ok(OpCode::SUB_DOUBLE_2ADDR),           // Instruction 12x
            0xcd => Ok(OpCode::MUL_DOUBLE_2ADDR),           // Instruction 12x
            0xce => Ok(OpCode::DIV_DOUBLE_2ADDR),           // Instruction 12x
            0xcf => Ok(OpCode::REM_DOUBLE_2ADDR),           // Instruction 12x
            0xd0 => Ok(OpCode::ADD_INT_LIT16),              // Instruction 22s
            0xd1 => Ok(OpCode::RSUB_INT),                   // Instruction 22s
            0xd2 => Ok(OpCode::MUL_INT_LIT16),              // Instruction 22s
            0xd3 => Ok(OpCode::DIV_INT_LIT16),              // Instruction 22s
            0xd4 => Ok(OpCode::REM_INT_LIT16),              // Instruction 22s
            0xd5 => Ok(OpCode::AND_INT_LIT16),              // Instruction 22s
            0xd6 => Ok(OpCode::OR_INT_LIT16),               // Instruction 22s
            0xd7 => Ok(OpCode::XOR_INT_LIT16),              // Instruction 22s
            0xd8 => Ok(OpCode::ADD_INT_LIT8),               // Instruction 22b
            0xd9 => Ok(OpCode::RSUB_INT_LIT8),              // Instruction 22b
            0xda => Ok(OpCode::MUL_INT_LIT8),               // Instruction 22b
            0xdb => Ok(OpCode::DIV_INT_LIT8),               // Instruction 22b
            0xdc => Ok(OpCode::REM_INT_LIT8),               // Instruction 22b
            0xdd => Ok(OpCode::AND_INT_LIT8),               // Instruction 22b
            0xde => Ok(OpCode::OR_INT_LIT8),                // Instruction 22b
            0xdf => Ok(OpCode::XOR_INT_LIT8),               // Instruction 22b
            0xe0 => Ok(OpCode::SHL_INT_LIT8),               // Instruction 22b
            0xe1 => Ok(OpCode::SHR_INT_LIT8),               // Instruction 22b
            0xe2 => Ok(OpCode::USHR_INT_LIT8),              // Instruction 22b

            /* Unused */
            0xe3 => panic!("Warning: use of undefined opcode {}", value),
            0xe4 => panic!("Warning: use of undefined opcode {}", value),
            0xe5 => panic!("Warning: use of undefined opcode {}", value),
            0xe6 => panic!("Warning: use of undefined opcode {}", value),
            0xe7 => panic!("Warning: use of undefined opcode {}", value),
            0xe8 => panic!("Warning: use of undefined opcode {}", value),
            0xe9 => panic!("Warning: use of undefined opcode {}", value),
            0xea => panic!("Warning: use of undefined opcode {}", value),
            0xeb => panic!("Warning: use of undefined opcode {}", value),
            0xec => panic!("Warning: use of undefined opcode {}", value),
            0xed => panic!("Warning: use of undefined opcode {}", value),
            0xee => panic!("Warning: use of undefined opcode {}", value),
            0xef => panic!("Warning: use of undefined opcode {}", value),
            0xf0 => panic!("Warning: use of undefined opcode {}", value),
            0xf1 => panic!("Warning: use of undefined opcode {}", value),
            0xf2 => panic!("Warning: use of undefined opcode {}", value),
            0xf3 => panic!("Warning: use of undefined opcode {}", value),
            0xf4 => panic!("Warning: use of undefined opcode {}", value),
            0xf5 => panic!("Warning: use of undefined opcode {}", value),
            0xf6 => panic!("Warning: use of undefined opcode {}", value),
            0xf7 => panic!("Warning: use of undefined opcode {}", value),
            0xf8 => panic!("Warning: use of undefined opcode {}", value),
            0xf9 => panic!("Warning: use of undefined opcode {}", value),

            0xfa => Ok(OpCode::INVOKE_POLYMORPHIC),         // Instruction 45cc,
            0xfb => Ok(OpCode::INVOKE_POLYMORPHIC_RANGE),   // Instruction 4rcc,
            0xfc => Ok(OpCode::INVOKE_CUSTOM),              // Instruction 35c,
            0xfd => Ok(OpCode::INVOKE_CUSTOM_RANGE),        // Instruction 3rc,
            0xfe => Ok(OpCode::CONST_METHOD_HANDLE),        // Instruction 21c,
            0xff => Ok(OpCode::CONST_METHOD_TYPE),          // Instruction 21c,

            /* Default */
            _ => panic!("Warning: nknown opcode {}", value),
        }
    }
}
