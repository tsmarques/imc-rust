use crate::engine::Tokens::Field;
use std::fmt;
use std::fmt::Formatter;

#[derive(PartialEq, Debug)]
pub enum ImcType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    Fp32,
    Fp64,
    Raw,
    PlainText,
    Enum,
    Bitfield,
    Message,
    MessageList,
    Unknown,
}

impl ImcType {
    fn size(&self) -> Option<i32> {
        match *self {
            ImcType::U8 | ImcType::I8 => Option::from(1),
            ImcType::U16 | ImcType::I16 => Option::from(2),
            ImcType::U32 | ImcType::I32 => Option::from(4),
            ImcType::U64 | ImcType::I64 => Option::from(8),
            ImcType::Fp32 => Option::from(4),
            ImcType::Fp64 => Option::from(8),
            ImcType::Enum | ImcType::Bitfield => Option::from(1),
            ImcType::Unknown | ImcType::Raw | ImcType::Message | ImcType::MessageList => {
                Option::None
            }
            _ => panic!("unknown type"),
        }
    }
}

impl fmt::Display for ImcType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            ImcType::U8 => write!(f, "u8"),
            ImcType::U16 => write!(f, "u16"),
            ImcType::U32 => write!(f, "u32"),
            ImcType::I8 => write!(f, "i8"),
            ImcType::I32 => write!(f, "i32"),
            ImcType::I64 => write!(f, "i64"),
            ImcType::Fp32 => write!(f, "f32"),
            ImcType::Fp64 => write!(f, "f64"),
            ImcType::Enum | ImcType::Bitfield => write!(f, "enum"),
            ImcType::Raw => write!(f, "Vec<u8>"),
            ImcType::PlainText => write!(f, "String"),
            ImcType::Message => write!(f, "Message"),
            _ => unimplemented!(),
        }
    }
}

// impl fmt::Debug for ImcType {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, *self)
//     }
// }

pub fn convert(ftype: &str) -> ImcType {
    match ftype {
        "uint8_t" => ImcType::U8,
        "uint16_t" => ImcType::U16,
        "uint32_t" => ImcType::U32,
        "uint64_t" => ImcType::U64,
        "int8_t" => ImcType::I8,
        "int16_t" => ImcType::I16,
        "int32_t" => ImcType::I32,
        "int64_t" => ImcType::I64,
        "fp32_t" => ImcType::Fp32,
        "fp64_t" => ImcType::Fp64,
        "rawdata" => ImcType::Raw,
        "plaintext" => ImcType::PlainText,
        "message" => ImcType::Message,
        "message-list" => ImcType::MessageList,
        _ => panic!("unknown type {}", ftype),
    }
}

pub fn get_init_string(field: &Field) -> String {
    if !field.default_value.is_none() {
        // @fixme wtf...
        return field.default_value.as_ref().unwrap().parse().unwrap();
    }

    let default_v = field.default_value.clone();
    match default_v {
        Some(value) => value,
        None => match field.ftype {
            ImcType::U8
            | ImcType::U16
            | ImcType::U32
            | ImcType::U64
            | ImcType::I8
            | ImcType::I16
            | ImcType::I32
            | ImcType::I64 => "0".to_string(),
            ImcType::Fp32 | ImcType::Fp64 => "0.0".to_string(),
            ImcType::Raw => "vec![]".to_string(),
            ImcType::PlainText => "String::from(\"\")".to_string(),
            ImcType::Message => {
                if field.msg_type.is_none() {
                    panic!("unkown message-type")
                }
                format!("{}::new()", field.msg_type.as_ref().unwrap())
            }
            ImcType::MessageList => {
                if field.msg_type.is_none() {
                    panic!("unkown message-type")
                }
                "vec![]".to_string()
            }
            _ => panic!("unknown type {}", field.ftype),
        },
    }
}

pub mod Serialization {
    use crate::engine::Tokens::Field;
    use crate::engine::Types;
    use crate::engine::Types::ImcType;

    pub fn get_fn_string(field: &Field) -> String {
        if !field.msg_type.is_none() {
            // @fixme wtf...
            panic!("don't know how to serialize this yet...")
        }

        match &field.ftype {
            ImcType::Raw | ImcType::PlainText => String::from("serialize_string!"),
            ImcType::U8 => format!("put_u8"),
            ImcType::Enum | ImcType::Bitfield => panic!("what to do with bitfield and enum.."),
            v => format!("put_{}_le", v),
            _ => panic!("unhandled type"),
        }
    }
}
