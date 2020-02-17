pub struct Data {
    pub desc:String,
    pub id :String,
    pub enums:String,
    pub name :String,
    pub abbrev :String,
    pub source :String,
    pub flags :String,
    pub category :String,
    pub fields :Vec<FieldData>,
    pub fields_init :String,
    pub clear :String,
    pub fixed_serialization_size :String,
    pub fixed_size :String,
    pub field_serialization :String,
}

pub struct FieldData {
    pub field_desc :String,
    pub field_name :String,
    pub field_abbrev :String,
    pub field_type :String,
    pub field_unit :String,
    pub field_msg_type :String,
    pub field_enum :Vec<EnumData>,
    pub field_enum_prefix :String,
    pub field_ser_size :usize,
    pub field_max: String,
    pub field_min: String
}

pub struct EnumData {
    pub id :String,
    pub name :String,
    pub abbrev :String,
}

fn parse_type(ftype :&str, msg_type :Option<String>) -> (String, String) {
    let mut rust_type :String;

    match ftype {
        "uint8_t" => ("u8".to_string(), "0".to_string()),
        "uint16_t" => ("u16".to_string(), "0".to_string()),
        "uint32_t" => ("u32".to_string(), "0".to_string()),
        "uint64_t" => ("u64".to_string(), "0".to_string()),
        "int8_t" => ("i8".to_string(), "0".to_string()),
        "int16_t" => ("i16".to_string(), "0".to_string()),
        "int32_t" => ("i32".to_string(), "0".to_string()),
        "int64_t" => ("i64".to_string(), "0".to_string()),
        "fp32_t" => ("f32".to_string(), "0.0".to_string()),
        "fp64_t" => ("f64".to_string(), "0.0".to_string()),
        "rawdata" => ("Vec<u8>".to_string(), "vec![]".to_string()),
        "plaintext" => ("&'static str".to_string(), "".to_string()),
        "message" => {
            if msg_type.is_none() {panic!("unknown message-type")}
            ("Message".to_string(), msg_type.unwrap())
        }
        "message-list" => {
            if msg_type.is_none() {panic!("unkown message-type")}
            (format!("Vec<{}>", msg_type.unwrap()), "vec![]".to_string())
        },
        _ => panic!("unknown type {}", ftype)
    }
}

impl FieldData {
    pub(crate) fn new() -> FieldData {
        FieldData {
            field_desc: String::from(""),
            field_name: String::from(""),
            field_abbrev: String::from(""),
            field_type: String::from(""),
            field_unit: String::from(""),
            field_msg_type: String::from(""),
            field_enum: vec![],
            field_enum_prefix: "".to_string(),
            field_max: "".to_string(),
            field_min: "".to_string(),
            field_ser_size: 0
        }
    }

    fn as_str(&self) -> String {
        let mut msg_type_opt :Option<String> = None;
        if !self.field_msg_type.is_empty() {
            msg_type_opt = Option::from(self.field_msg_type.to_string());
        }


        let type_info = parse_type(&self.field_type, msg_type_opt);

        format!("// {}\npub {} :{}",
                self.field_desc,
                self.field_abbrev,
                type_info.0,
        )

        // TODO field initialization
    }
}

impl Data {
    pub fn new() -> Data {
        Data {
            desc: "".to_string(),
            id: "".to_string(),
            enums: "".to_string(),
            name: "".to_string(),
            abbrev: "".to_string(),
            source: "".to_string(),
            flags: "".to_string(),
            category: "".to_string(),
            fields: vec![],
            fields_init: "".to_string(),
            clear: "".to_string(),
            fixed_serialization_size: "".to_string(),
            fixed_size: "".to_string(),
            field_serialization: "".to_string()
        }
    }
}