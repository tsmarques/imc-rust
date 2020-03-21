pub struct Message {
    pub desc:String,
    pub id :String,
    pub enums:String,
    pub name :String,
    pub abbrev :String,
    pub source :String,
    pub flags :String,
    pub category :String,
    pub fields :Vec<Field>,
    pub fields_init :String,
    pub clear :String,
    pub fixed_serialization_size :String,
    pub fixed_size :String,
    pub field_serialization :String,
}

pub struct Field {
    pub field_desc :String,
    pub field_name :String,
    pub field_abbrev :String,
    pub field_type :String,
    pub field_unit :String,
    pub field_default_value :Option<String>,
    pub field_msg_type :Option<String>,
    pub field_enum :Vec<Enum>,
    pub field_enum_prefix :String,
    pub field_ser_size :usize,
    pub field_max: Option<String>,
    pub field_min: Option<String>,
    pub is_fixed: bool
}

pub struct Enum {
    pub id :String,
    pub name :String,
    pub abbrev :String,
}

impl Field {
    pub(crate) fn new() -> Field {
        Field {
            field_desc: String::from(""),
            field_name: String::from(""),
            field_abbrev: String::from(""),
            field_type: String::from(""),
            field_unit: String::from(""),
            field_default_value: Option::None,
            field_msg_type: Option::None,
            field_enum: vec![],
            field_enum_prefix: String::from(""),
            field_max: Option::None,
            field_min: Option::None,
            field_ser_size: 0,
            is_fixed: false
        }
    }
}

impl Message {
    pub fn new() -> Message {
        Message {
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