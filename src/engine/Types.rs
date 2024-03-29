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
                    write!(f, "Vec<Box<dyn Message>>")
                }
            }
            _ => unimplemented!(),
        }
    }
}

pub fn get_fixed_size(field: &Field) -> Option<i32> {
    match field.ftype.type_enum {
        ImcTypeEnum::U8 | ImcTypeEnum::I8 => Option::from(1),
        ImcTypeEnum::U16 | ImcTypeEnum::I16 => Option::from(2),
        ImcTypeEnum::U32 | ImcTypeEnum::I32 => Option::from(4),
        ImcTypeEnum::U64 | ImcTypeEnum::I64 => Option::from(8),
        ImcTypeEnum::Fp32 => Option::from(4),
        ImcTypeEnum::Fp64 => Option::from(8),
        ImcTypeEnum::Enum | ImcTypeEnum::Bitfield => Option::from(1),
        ImcTypeEnum::Unknown
        | ImcTypeEnum::PlainText
        | ImcTypeEnum::Raw
        | ImcTypeEnum::Message
        | ImcTypeEnum::MessageList => Option::None,
    }
}

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
    let default_v = field.default_value.clone();
    match default_v {
        Some(value) => format!("{} as {}", value, field.ftype),
        None => String::from("Default::default()"),
    }
}

pub fn get_serialization_string(field: &Field) -> String {
    match &field.ftype.type_enum {
        ImcTypeEnum::PlainText => {
            format!("serialize_bytes!(bfr, self._{}.as_bytes())", field.abbrev)
        }
        ImcTypeEnum::Raw => format!("serialize_bytes!(bfr, self._{}.as_slice())", field.abbrev),
        ImcTypeEnum::U8 => format!("bfr.put_u8(self._{})", field.abbrev),
        ImcTypeEnum::Enum | ImcTypeEnum::Bitfield => panic!("what to do with bitfield and enum.."),
        ImcTypeEnum::Message => format!(
            "match &self._{} {{\n
                 Some(field) => field.serialize(bfr),\n
                 None => {{}}\n
            }}\n",
            field.abbrev
        ),
        ImcTypeEnum::MessageList => format!(
            "for msg in self._{}.iter() {{\nmsg.serialize(bfr);\n}}",
            field.abbrev
        ),
        ImcTypeEnum::I8 => format!("bfr.put_i8(self._{})", field.abbrev),
        _v => format!("bfr.put_{}_le(self._{})", field.ftype, field.abbrev),
        _ => panic!("unhandled type"),
    }
}

pub fn get_clear_string(field: &Field) -> String {
    match &field.ftype.type_enum {
        ImcTypeEnum::MessageList => format!(
            "for msg in self._{}.iter_mut() {{\nmsg.clear();\n}}",
            field.abbrev
        ),
        ImcTypeEnum::Message => format!(
            // yikes
            "match &mut self._{} {{\n
                 Some(field) => field.clear(),\n
                 None => {{}}\n
            }}\n",
            field.abbrev
        ),
        _primitive_type => format!("self._{} = Default::default();", field.abbrev),
    }
}
