use crate::engine::Tokens::Field;

pub fn convert(field :&Field) -> String {
    let ftype = field.ftype.as_str();
    match ftype {
        "uint8_t" => "u8".to_string(),
        "uint16_t" => "u16".to_string(),
        "uint32_t" => "u32".to_string(),
        "uint64_t" => "u64".to_string(),
        "int8_t" => "i8".to_string(),
        "int16_t" => "i16".to_string(),
        "int32_t" => "i32".to_string(),
        "int64_t" => "i64".to_string(),
        "fp32_t" => "f32".to_string(),
        "fp64_t" => "f64".to_string(),
        "rawdata" => "Vec<u8>".to_string(),
        "plaintext" => "&'static str".to_string(),
        "message" |
        "message-list" => {
            if field.msg_type.is_none() { panic!("unkown message-type") }
            format!("Vec<{}>", field.msg_type.as_ref().unwrap())
        },
        _ => panic!("unknown type {}", ftype)
    }
}

pub fn get_init_string(field :&Field) -> String {
    if !field.default_value.is_none() {
        // @fixme wtf...
        return field.default_value.as_ref().unwrap().parse().unwrap();
    }

    let default_v = field.default_value.clone();
    match default_v {
        Some(value) => value,
        None => {
            let ftype = field.ftype.as_str();
            match ftype {
                "uint8_t"  | "uint16_t" |
                "uint32_t" | "uint64_t" |
                "int8_t"   | "int16_t"  |
                "int32_t"  | "int64_t" => "0".to_string(),
                "fp32_t" | "fp64_t" => "0.0".to_string(),
                "rawdata" => "vec![]".to_string(),
                "plaintext" => "String::from(\"\")".to_string(),
                "message" => {
                    if field.msg_type.is_none() { panic!("unkown message-type") }
                    format!("{}::new()", field.msg_type.as_ref().unwrap())
                }
                "message-list" => {
                    if field.msg_type.is_none() { panic!("unkown message-type") }
                    "vec![]".to_string()
                },
                _ => panic!("unknown type {}", ftype)
            }
        },
    }
}

pub mod Serialization {
    use crate::engine::Tokens::Field;
    use crate::engine::Types;

    pub fn get_fn_string(field: &Field) -> String {
        if !field.msg_type.is_none() {
            // @fixme wtf...
            panic!("don't know how to serialize this yet...")
        }

        let ftype = Types::convert(field);
        match ftype.as_str() {
            "plaintext" => String::from("serialize_string!"),
            "enum-def" | "bitfield-def" => panic!("unhandled type.."),
            "u8" => format!("put_u8"),
            v => format!("put_{}_le", v),
        }
    }
}