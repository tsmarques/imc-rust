use crate::engine::Tokens::Field;
use std::fmt;
use std::fmt::Formatter;

#[derive(PartialEq, Debug)]
pub enum ImcTypeEnum {
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

impl ImcTypeEnum {
    fn size(&self) -> Option<i32> {
        match *self {
            ImcTypeEnum::U8 | ImcTypeEnum::I8 => Option::from(1),
            ImcTypeEnum::U16 | ImcTypeEnum::I16 => Option::from(2),
            ImcTypeEnum::U32 | ImcTypeEnum::I32 => Option::from(4),
            ImcTypeEnum::U64 | ImcTypeEnum::I64 => Option::from(8),
            ImcTypeEnum::Fp32 => Option::from(4),
            ImcTypeEnum::Fp64 => Option::from(8),
            ImcTypeEnum::Enum | ImcTypeEnum::Bitfield => Option::from(1),
            ImcTypeEnum::Unknown
            | ImcTypeEnum::Raw
            | ImcTypeEnum::Message
            | ImcTypeEnum::MessageList => Option::None,
            _ => panic!("unknown type"),
        }
    }
}

pub struct ImcType {
    pub(crate) type_enum: ImcTypeEnum,
    pub(crate) message_type: Option<String>,
}

impl fmt::Display for ImcType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.type_enum {
            ImcTypeEnum::U8 => write!(f, "u8"),
            ImcTypeEnum::U16 => write!(f, "u16"),
            ImcTypeEnum::U32 => write!(f, "u32"),
            ImcTypeEnum::U64 => write!(f, "u64"),
            ImcTypeEnum::I8 => write!(f, "i8"),
            ImcTypeEnum::I16 => write!(f, "i16"),
            ImcTypeEnum::I32 => write!(f, "i32"),
            ImcTypeEnum::I64 => write!(f, "i64"),
            ImcTypeEnum::Fp32 => write!(f, "f32"),
            ImcTypeEnum::Fp64 => write!(f, "f64"),
            ImcTypeEnum::Enum | ImcTypeEnum::Bitfield => write!(f, "enum"),
            ImcTypeEnum::Raw => write!(f, "Vec<u8>"),
            ImcTypeEnum::PlainText => write!(f, "String"),
            ImcTypeEnum::Message => {
                if self.message_type.is_some() {
                    // @fixme how to avoid clone?
                    write!(f, "Option<Box<{}>>", self.message_type.clone().unwrap())
                } else {
                    // for some reason type wasn't specified
                    write!(f, "Option<Box<dyn Message>>")
                }
            }
            ImcTypeEnum::MessageList => {
                if self.message_type.is_some() {
                    // @fixme how to avoid clone?
                    write!(f, r"Vec<Box<{}>>", self.message_type.clone().unwrap())
                } else {
                    // for some reason type wasn't specified
                    write!(f, "{}", "Vec<Box<dyn Message>>")
                }
            }
            _ => unimplemented!(),
        }
    }
}

// impl fmt::Debug for ImcType {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, *self)
//     }
// }

pub fn convert(ftype: &str) -> ImcTypeEnum {
    match ftype {
        "uint8_t" => ImcTypeEnum::U8,
        "uint16_t" => ImcTypeEnum::U16,
        "uint32_t" => ImcTypeEnum::U32,
        "uint64_t" => ImcTypeEnum::U64,
        "int8_t" => ImcTypeEnum::I8,
        "int16_t" => ImcTypeEnum::I16,
        "int32_t" => ImcTypeEnum::I32,
        "int64_t" => ImcTypeEnum::I64,
        "fp32_t" => ImcTypeEnum::Fp32,
        "fp64_t" => ImcTypeEnum::Fp64,
        "rawdata" => ImcTypeEnum::Raw,
        "plaintext" => ImcTypeEnum::PlainText,
        "message" => ImcTypeEnum::Message,
        "message-list" => ImcTypeEnum::MessageList,
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
        None => match field.ftype.type_enum {
            ImcTypeEnum::U8
            | ImcTypeEnum::U16
            | ImcTypeEnum::U32
            | ImcTypeEnum::U64
            | ImcTypeEnum::I8
            | ImcTypeEnum::I16
            | ImcTypeEnum::I32
            | ImcTypeEnum::I64 => "0".to_string(),
            ImcTypeEnum::Fp32 | ImcTypeEnum::Fp64 => "0.0".to_string(),
            ImcTypeEnum::Raw => "vec![]".to_string(),
            ImcTypeEnum::PlainText => "String::new()".to_string(),
            ImcTypeEnum::Message => {
                if field.ftype.message_type.is_none() {
                    format!("Option::None")
                } else {
                    format!("{}::new()", field.ftype.message_type.as_ref().unwrap())
                }
            }
            ImcTypeEnum::MessageList => "vec![]".to_string(),
            _ => panic!("unknown type {}", field.ftype),
        },
    }
}
