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
    pub fixed_serialization_size :usize,
    pub fixed_size :usize,
}

pub struct Field {
    // Field's description
    pub desc:String,
    // Field's name
    pub name:String,
    // Field's abbreviation
    pub abbrev:String,
    // Field's type
    pub ftype:String,
    // Field's unit
    pub unit:String,
    // Field's default value (optional)
    pub default_value:Option<String>,
    // Field's IMC message type (optional)
    pub msg_type:Option<String>,
    // Enumerators values
    pub enums:Vec<Enum>,
    // Enumerators' prefix
    pub enum_prefix:String,
    // Field's Serialization size
    pub ser_size :usize,
    // Field maximum value (optional)
    pub max_value: Option<String>,
    // Field minimum value (optional)
    pub min_value: Option<String>,
    // Read only field
    pub read_only: bool
}

pub struct Enum {
    pub id :String,
    pub name :String,
    pub abbrev :String,
}

impl Field {
    pub(crate) fn new() -> Field {
        Field {
            desc: String::from(""),
            name: String::from(""),
            abbrev: String::from(""),
            ftype: String::from(""),
            unit: String::from(""),
            default_value: Option::None,
            msg_type: Option::None,
            enums: vec![],
            enum_prefix: String::from(""),
            max_value: Option::None,
            min_value: Option::None,
            ser_size: 0,
            read_only: false
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
            fixed_serialization_size: 0,
            fixed_size: 0,
        }
    }
}