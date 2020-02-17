mod MessageBuilder;

extern crate xml;
extern crate clap;

use clap::{Arg, App};
use std::fs::File;
use std::io::BufReader;
use xml::EventReader;
use xml::reader::{XmlEvent, Error};
use crate::MessageBuilder::{FieldData, EnumData};
use xml::attribute::OwnedAttribute;

fn parse_field_enum(field :&mut FieldData, attr :&Vec<OwnedAttribute>) {
    let mut field_enum :EnumData = EnumData {
        id: "".to_string(),
        name: "".to_string(),
        abbrev: "".to_string()
    };

    for a in attr {
        match a.name.local_name.as_str() {
            "id" => field_enum.id = a.value.clone(),
            "name" => field_enum.name = a.value.clone(),
            "abbrev" => field_enum.abbrev = a.value.clone(),
            _ => panic!("enum : {} : unhandled enum attribute {}", field.field_name, a.name.local_name)
        }
    }

    field.field_enum.push(field_enum);
}

fn parse_field_attributes(field :&mut FieldData, attr :&Vec<OwnedAttribute>) {
    for attr in attr {
        let value = attr.value.trim().to_string();
        match attr.name.local_name.as_str() {
            "name" => field.field_name = value,
            "abbrev" => field.field_abbrev = value,
            "type" => field.field_type = value,
            "message-type" => field.field_msg_type = value,
            "unit" => field.field_unit = value,
            "prefix" => field.field_enum_prefix = value,
            "max" => field.field_max = value,
            "min" => field.field_min = value,
            _ => { panic!("unhandled field attribute : {}", attr.name) }
        }
    }
}

fn parse_fields(parser : &mut EventReader<BufReader<File>>) -> Vec<FieldData> {
    let mut fields = vec![];

    let mut i = 0;
    loop {
        match parser.next() {
            Ok(XmlEvent::StartElement { name, attributes, .. }) =>
                match name.local_name.as_str() {
                    "field" => {
                        fields.push(FieldData::new());
                        parse_field_attributes(&mut fields[i], &attributes)
                    },
                    "description" => {},
                    "value" => parse_field_enum(&mut fields[i], &attributes),
                    _ => panic!("parse field : unknown name {}", name.local_name)
                },
            Ok(XmlEvent::Characters(content)) => fields[i].field_desc = content.trim().to_string(),
            Ok(XmlEvent::EndElement { name }) =>
                match name.local_name.as_str() {
                    "field" => i+=1,
                    "message" => { break },
                    "description" => {},
                    _ => panic!("parse_fields: unknown termination \"{}\"", name.local_name),
                },
            Ok(_) => {},
            Err(e) => {
                panic!("parse field error: {}", e);
                break;
            }
        }
    }

    fields
}

fn parse_message_attributes(message :&mut MessageBuilder::Data, attributes :&Vec<OwnedAttribute>) {
    for attr in attributes {
        let value = attr.value.trim().to_string();
        match attr.name.local_name.as_str() {
            "id" => message.id = value,
            "name" => message.name = value,
            "abbrev" => message.abbrev = value,
            "source" => message.source = value,
            "flags" => message.flags = value,
            "category" => message.category = value,
            _ => { panic!("unhandled field attribute : {}", attr.name) }
        }
    }
}

// FIXME: requires that after a <message ..> a <description> comes
// before any <field ..> tag, otherwise it will parse incorrectly
fn parse_message(parser : &mut EventReader<BufReader<File>>) -> MessageBuilder::Data {
    let mut mb :MessageBuilder::Data = MessageBuilder::Data::new();
    let mut e = parser.next();
    loop {
        match e {
            Err(e) => panic!("parse_message: error: {}", e),
            Ok(XmlEvent::StartElement {name, attributes, ..}) => {
                match name.local_name.as_str() {
                    "message" => parse_message_attributes(&mut mb, &attributes) ,
                    "description" => { },
                    _ => {panic!("parse_message: unexpected tag: {}", name.local_name)},
                }
            },
            Ok(XmlEvent::Characters(content)) => mb.desc = content.trim().to_string(),
            Ok(XmlEvent::Whitespace(_)) => {},
            Ok(XmlEvent::EndElement { name }) =>
            // parse fields after description
                if name.local_name == "description" {
                    mb.fields = parse_fields(parser);
                    break;
                },
            Ok(_) => panic!("parse_message: unhandled event"),
        }

        e = parser.next();
    }

    mb
}

fn parse(xml_path : &str) {
    let file = File::open(xml_path).unwrap();
    let file = BufReader::new(file);

    let mut parser = EventReader::new(file);
    let mut depth = 0;

//    footer, header, flags, message-group, bitfields, enumerations, units, serialization, types,
    // description

//    loop {
//        match parser.next() {
//            Ok(XmlEvent::StartDocument) => {
//
//            }
//        }
//    }
//    for e in parser {
//        match e {
//            Ok(XmlEvent::StartElement { name, .. }) => {
//                println!("{}+{}", indent(depth), name);
//                depth += 1;
//            }
//            Ok(XmlEvent::EndElement { name }) => {
//                depth -= 1;
//                println!("{}-{}", indent(depth), name);
//            }
//            Err(e) => {
//                println!("Error: {}", e);
//                break;
//            }
//            _ => {}
//        }
//    }
}

fn main() {
    let matches = App::new("IMC Rust")
    .version("0.1.0")
    .author("Tiago Marques <tmarques@oceanscan-mst.com>")
    .about("Rust IMC bindings generator")
    .arg(Arg::with_name("imc")
    .long("imc")
    .takes_value(true)
    .help("Full path to IMC.xml file"))
    .get_matches();

    let ret = matches.value_of("imc");
    match ret {
        Some(v) => parse(v),
        None => panic!("missing path to IMC definition. Use --imc option")
    }
}

#[test]
fn test_message() {
    let file = File::open("/home/tsm/ws/imc-rust/src/xml/tests/test-imc-message.xml").unwrap();
    let file = BufReader::new(file);

    let mut parser = EventReader::new(file);
// discard start document event
    parser.next();
    let message = parse_message(&mut parser);

    assert_eq!(message.id, "7");
    assert_eq!(message.name, "CPU Usage");
    assert_eq!(message.abbrev, "CpuUsage");
    assert_eq!(message.source, "vehicle");
    assert_eq!(message.flags, "periodic");
    assert_eq!(message.category, "Core");

    assert_eq!(message.fields.len(), 1);

    assert_eq!(message.fields[0].field_name, "Usage percentage");
    assert_eq!(message.fields[0].field_abbrev, "value");
    assert_eq!(message.fields[0].field_type, "uint8_t");
    assert_eq!(message.fields[0].field_max, "100");
    assert_eq!(message.fields[0].field_unit, "%");

    let estate = parse_message(&mut parser);

    assert_eq!(estate.id, "350");
    assert_eq!(estate.name, "Estimated State");
    assert_eq!(estate.abbrev, "EstimatedState");
    assert_eq!(estate.source, "vehicle");
    assert_eq!(estate.flags, "periodic");
    assert_eq!(estate.category, "Navigation");

    assert_eq!(estate.fields.len(), 20);
}

//#[test]
//fn test_field() {
//    let file = File::open("/home/tsm/ws/imc-rust/src/imc/xml/tests/test-imc-field.xml").unwrap();
//    let file = BufReader::new(file);
//
//    let mut parser = EventReader::new(file);
//
//// discard start document event
//    parser.next();
//    let fields = parse_fields(&mut parser);
//    let field = &fields[0];
//
//    assert_eq!(field.field_name, "Distance");
//    assert_eq!(field.field_desc, "Distance between current LBL Beacon position and filter estimation.");
//    assert_eq!(field.field_abbrev, "distance");
//    assert_eq!(field.field_type, "fp32_t");
//    assert_eq!(field.field_unit, "m");
//    assert!(field.field_msg_type.is_empty());
//    assert!(field.field_min.is_empty());
//    assert!((field.field_max.is_empty()));
//// todo
//    assert_eq!(field.field_ser_size, 0);
//
////    let field2 = parse_fields(&mut parser);
//
////    assert_eq!(field2.field_name, "State");
////    assert_eq!(field2.field_desc, "Alignment State.");
////    assert_eq!(field2.field_abbrev, "state");
////    assert_eq!(field2.field_type, "uint8_t");
////    assert_eq!(field2.field_unit, "Enumerated");
////// todo
////    assert_eq!(field2.field_ser_size, 0);
////    assert!(field2.field_msg_type.is_empty());
////    assert_eq!(field2.field_enum_prefix, "AS");
////    assert_eq!(field2.field_enum.len(), 8);
////    assert!(field2.field_min.is_empty());
////    assert!(field2.field_max.is_empty());
////
////    assert_eq!(field2.field_enum[0].id, "0");
////    assert_eq!(field2.field_enum[0].name, "Not Aligned");
////    assert_eq!(field2.field_enum[0].abbrev, "NOT_ALIGNED");
////
////    assert_eq!(field2.field_enum[1].id, "1");
////    assert_eq!(field2.field_enum[1].name, "Aligned");
////    assert_eq!(field2.field_enum[1].abbrev, "ALIGNED");
////
////    assert_eq!(field2.field_enum[2].id, "2");
////    assert_eq!(field2.field_enum[2].name, "Not Supported");
////    assert_eq!(field2.field_enum[2].abbrev, "NOT_SUPPORTED");
////
////    assert_eq!(field2.field_enum[3].id, "3");
////    assert_eq!(field2.field_enum[3].name, "Aligning");
////    assert_eq!(field2.field_enum[3].abbrev, "ALIGNING");
////
////    assert_eq!(field2.field_enum[4].id, "4");
////    assert_eq!(field2.field_enum[4].name, "Wrong Medium");
////    assert_eq!(field2.field_enum[4].abbrev, "WRONG_MEDIUM");
////
////    assert_eq!(field2.field_enum[5].id, "5");
////    assert_eq!(field2.field_enum[5].name, "Coarse Alignment");
////    assert_eq!(field2.field_enum[5].abbrev, "COARSE_ALIGNMENT");
////
////    assert_eq!(field2.field_enum[6].id, "6");
////    assert_eq!(field2.field_enum[6].name, "Fine Alignment");
////    assert_eq!(field2.field_enum[6].abbrev, "FINE_ALIGNMENT");
////
////    assert_eq!(field2.field_enum[7].id, "7");
////    assert_eq!(field2.field_enum[7].name, "System Ready");
////    assert_eq!(field2.field_enum[7].abbrev, "SYSTEM_READY");
////
////    let field3 = parse_fields(&mut parser);
////
////    assert_eq!(field3.field_name, "Usage percentage");
////    assert_eq!(field3.field_desc, "The CPU usage, in percentage, of the sending software.");
////    assert_eq!(field3.field_abbrev, "value");
////    assert_eq!(field3.field_type, "uint8_t");
////    assert_eq!(field3.field_unit, "%");
////// todo
////    assert_eq!(field3.field_ser_size, 0);
////    assert!(field3.field_msg_type.is_empty());
////    assert!(field3.field_enum_prefix.is_empty());
////    assert_eq!(field3.field_enum.len(), 0);
////    assert!(field3.field_min.is_empty());
////    assert_eq!(field3.field_max, "100");
//}
