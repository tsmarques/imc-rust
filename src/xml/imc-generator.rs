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
        abbrev: "".to_string(),
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
            "enum-def" => {}, // TODO handle global enumerator
            "value" => field.field_default_value = value,
            "bitfield-def" => {},
            _ => { panic!("unhandled field attribute : {}", attr.name) }
        }
    }
}

fn parse_message(parser : &mut EventReader<BufReader<File>>) -> (String, Vec<FieldData>) {
    let mut fields = vec![];
    // flag if next <description> is from <message ...>
    let mut is_message_description = true;
    let mut message_descr :String = String::from("");

    let mut i = 0;
    loop {
        match parser.next() {
            Ok(XmlEvent::StartElement { name, attributes, .. }) =>
                match name.local_name.as_str() {
                    "field" => {
                        is_message_description = false;
                        fields.push(FieldData::new());
                        parse_field_attributes(&mut fields[i], &attributes)
                    },
                    "description" => {},
                    "value" => {
                        match fields[i].field_unit.as_str() {
                            "Enumerated" | "Bitfield" => parse_field_enum(&mut fields[i], &attributes),
                            _ => panic!("parse_field_enum: was expecting enum"),
                        }
                    }
                    _ => panic!("parse field : unknown name {}", name.local_name)
                },
            Ok(XmlEvent::Characters(content)) => {
                let description = content.trim().to_string();
                if !is_message_description {
                    fields[i].field_desc = description;
                    is_message_description = true;
                }
                else {
                    message_descr = description;
                    is_message_description = false;
                }
            },
            Ok(XmlEvent::EndElement { name }) =>
                match name.local_name.as_str() {
                    "field" => i+=1,
                    "message" => { break },
                    "description" | "value" => {},
                    _ => panic!("parse_fields: unknown termination \"{}\"", name.local_name),
                },
            Ok(_) => {},
            Err(e) => {
                panic!("parse field error: {}", e);
                break;
            }
        }
    }

    (message_descr, fields)
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
            "used-by" => {},
            _ => { panic!("unhandled field attribute : {}", attr.name) }
        }
    }
}


fn parse_messages(parser : &mut EventReader<BufReader<File>>) -> Vec<MessageBuilder::Data> {
    let mut messages :Vec<MessageBuilder::Data> = vec![];
    let mut i = 0;

    let mut e = parser.next();
    loop {
        match e {
            Err(e) => panic!("parse_message: error: {}", e),
            Ok(XmlEvent::StartElement {name, attributes, ..}) => {
                match name.local_name.as_str() {
                    "message" => {
                        messages.push(MessageBuilder::Data::new());
                        parse_message_attributes(&mut messages[i], &attributes);

                        let ret = parse_message(parser);
                        messages[i].fields = ret.1;
                        messages[i].desc = ret.0;
                        i+=1;
                    } ,
                    _ => {panic!("parse_message: unexpected tag: {}", name.local_name)},
                }
            },
            Ok(XmlEvent::Whitespace(_)) => {},
            Ok(XmlEvent::EndElement { .. }) => {},
            Ok(XmlEvent::EndDocument) => break,
            Ok(_) => panic!("parse_message: unhandled event"),
        }

        e = parser.next();
    }

    messages
}

fn ignore_tag(xml_tag :&str, parser : &mut EventReader<BufReader<File>>) {
    loop {
        match parser.next() {
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == xml_tag {
                    break;
                }
            }
            _ => {},
        }
    }
}

fn parse(xml_path : &str) -> Vec<MessageBuilder::Data> {
    let file = File::open(xml_path).unwrap();
    let file = BufReader::new(file);

    let mut parser = EventReader::new(file);
    let mut depth = 0;

    // footer, header, flags, message-group, bitfields, enumerations, units, serialization, types,
    // description

    let mut messages :Vec<MessageBuilder::Data> = vec![];
    loop {
        let e = parser.next();
        match e {
            Ok(XmlEvent::StartDocument {..}) => {},
            Ok(XmlEvent::StartElement {name, attributes, ..}) => {
                match name.local_name.as_str() {
                    "type" |
                    "description" | "types" | "serialization" |
                    "units" | "enumerations" | "bitfields" |
                    "message-groups" | "flags" | "header"
                    => { ignore_tag(name.local_name.as_str(), &mut parser) }, // ignore, for now, this headers
                    "messages" => {
                        let ret = attributes.iter().find(|x| x.name.local_name == "version");
                        match ret {
                            None => panic!("parse: missing \"version\" field in \"message\" : {}", name.local_name),
                            Some(v) => println!("Generating IMC version {}", v),
                        }
                    },
                    "footer" => {
                        ignore_tag("footer", &mut parser);
                        messages = parse_messages(&mut parser);
                        break;
                    }
                    _ => panic!("parse: unhandled tag {}", name.local_name),
                }
            },
            Ok(XmlEvent::EndElement {name}) => {  },
            Ok(XmlEvent::EndDocument) => break,
            Ok(XmlEvent::Whitespace(..)) |
            Ok(XmlEvent::Characters(..)) => {},
            _ => panic!("parse: unhandled event"),
        }
    }

    messages
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
    let messages :Vec<MessageBuilder::Data>;
    match ret {
        Some(v) => messages = parse(v),
        None => panic!("missing path to IMC definition. Use --imc option")
    }
}

#[test]
fn full_format() {
    let messages = parse("/home/tsm/ws/imc-rust/src/xml/tests/test-imc-full.xml");

    assert_eq!(messages.len(), 3);
    let mut i = 0;
    let mut n_fields = 0;

    assert_eq!(messages[i].name, "Entity State");
    assert_eq!(messages[i].abbrev, "EntityState");
    assert_eq!(messages[i].source, "vehicle");
    assert_eq!(messages[i].category, "Core");
    assert_eq!(messages[i].desc, "State reported by an entity in the vehicle. The source entity is
            identified in the message header.");

    assert_eq!(messages[i].fields[0].field_name, "State");
    assert_eq!(messages[i].fields[0].field_abbrev, "state");
    assert_eq!(messages[i].fields[0].field_desc, "State of entity.");
    assert_eq!(messages[i].fields[0].field_type, "uint8_t");
    assert_eq!(messages[i].fields[0].field_unit, "Enumerated");
    assert_eq!(messages[i].fields[0].field_enum_prefix, "ESTA");
    assert_eq!(messages[i].fields[0].field_enum.len(), 5);
    assert_eq!(messages[i].fields[0].field_enum[0].id, "0");
    assert_eq!(messages[i].fields[0].field_enum[0].name, "Bootstrapping");
    assert_eq!(messages[i].fields[0].field_enum[0].abbrev, "BOOT");
    assert_eq!(messages[i].fields[0].field_enum[1].id, "1");
    assert_eq!(messages[i].fields[0].field_enum[1].name, "Normal Operation");
    assert_eq!(messages[i].fields[0].field_enum[1].abbrev, "NORMAL");
    assert_eq!(messages[i].fields[0].field_enum[2].id, "2");
    assert_eq!(messages[i].fields[0].field_enum[2].name, "Fault");
    assert_eq!(messages[i].fields[0].field_enum[2].abbrev, "FAULT");
    assert_eq!(messages[i].fields[0].field_enum[3].id, "3");
    assert_eq!(messages[i].fields[0].field_enum[3].name, "Error");
    assert_eq!(messages[i].fields[0].field_enum[3].abbrev, "ERROR");
    assert_eq!(messages[i].fields[0].field_enum[4].id, "4");
    assert_eq!(messages[i].fields[0].field_enum[4].name, "Failure");
    assert_eq!(messages[i].fields[0].field_enum[4].abbrev, "FAILURE");
    n_fields += 1;

    assert_eq!(messages[i].fields[1].field_name, "Flags");
    assert_eq!(messages[i].fields[1].field_abbrev, "flags");
    assert_eq!(messages[i].fields[1].field_desc, "Complementary entity state flags.");
    assert_eq!(messages[i].fields[1].field_type, "uint8_t");
    assert_eq!(messages[i].fields[1].field_unit, "Bitfield");
    assert_eq!(messages[i].fields[1].field_enum_prefix, "EFLA");
    assert_eq!(messages[i].fields[1].field_enum.len(), 1);
    assert_eq!(messages[i].fields[1].field_enum[0].id, "0x01");
    assert_eq!(messages[i].fields[1].field_enum[0].name, "Human Intervention Required");
    assert_eq!(messages[i].fields[1].field_enum[0].abbrev, "HUMAN_INTERVENTION");
    n_fields += 1;

    assert_eq!(messages[i].fields[2].field_name, "Complementary description");
    assert_eq!(messages[i].fields[2].field_abbrev, "description");
    assert_eq!(messages[i].fields[2].field_desc, "Complementary human-readable description of entity state.");
    assert_eq!(messages[i].fields[2].field_type, "plaintext");
    assert!(messages[i].fields[2].field_unit.is_empty());
    assert!(messages[i].fields[2].field_enum_prefix.is_empty());
    assert_eq!(messages[i].fields[2].field_enum.len(), 0);
    n_fields += 1;

    // make sure we tested all fields
    assert_eq!(messages[i].fields.len(), 3);
    assert_eq!(messages[i].fields.len(), n_fields);

    i += 1;
    n_fields = 0;
    assert_eq!(messages[i].name, "Transport Bindings");
    assert_eq!(messages[i].abbrev, "TransportBindings");
    assert_eq!(messages[i].source, "vehicle");
    assert_eq!(messages[i].category, "Core");
    assert_eq!(messages[i].desc, "Message generated when tasks bind to messages.");
    assert_eq!(messages[i].fields[0].field_name, "Consumer name");
    assert_eq!(messages[i].fields[0].field_abbrev, "consumer");
    assert_eq!(messages[i].fields[0].field_desc, "The name of the consumer (e.g. task name).");
    assert_eq!(messages[i].fields[0].field_type, "plaintext");
    assert!(messages[i].fields[0].field_unit.is_empty());
    assert!(messages[i].fields[0].field_enum_prefix.is_empty());
    assert_eq!(messages[i].fields[0].field_enum.len(), 0);
    n_fields += 1;

    assert_eq!(messages[i].fields[1].field_name, "Message Identifier");
    assert_eq!(messages[i].fields[1].field_abbrev, "message_id");
    assert_eq!(messages[i].fields[1].field_desc, "The id of the message to be listened to.");
    assert_eq!(messages[i].fields[1].field_type, "uint16_t");
    assert!(messages[i].fields[1].field_unit.is_empty());
    assert!(messages[i].fields[1].field_enum_prefix.is_empty());
    assert_eq!(messages[i].fields[1].field_enum.len(), 0);
    n_fields += 1;

    assert_eq!(messages[i].fields.len(), 2);
    assert_eq!(messages[i].fields.len(), n_fields);
}

//#[test]
//fn test_message() {
//    let file = File::open("/home/tsm/ws/imc-rust/src/xml/tests/test-imc-message.xml").unwrap();
//    let file = BufReader::new(file);
//
//    let mut parser = EventReader::new(file);
//// discard start document event
//    parser.next();
//    let message = parse_messages(&mut parser);
//
//    assert_eq!(message.id, "7");
//    assert_eq!(message.name, "CPU Usage");
//    assert_eq!(message.abbrev, "CpuUsage");
//    assert_eq!(message.source, "vehicle");
//    assert_eq!(message.flags, "periodic");
//    assert_eq!(message.category, "Core");
//
//    assert_eq!(message.fields.len(), 1);
//
//    assert_eq!(message.fields[0].field_name, "Usage percentage");
//    assert_eq!(message.fields[0].field_abbrev, "value");
//    assert_eq!(message.fields[0].field_type, "uint8_t");
//    assert_eq!(message.fields[0].field_max, "100");
//    assert_eq!(message.fields[0].field_unit, "%");
//
//    let estate = parse_messages(&mut parser);
//
//    assert_eq!(estate.id, "350");
//    assert_eq!(estate.name, "Estimated State");
//    assert_eq!(estate.abbrev, "EstimatedState");
//    assert_eq!(estate.source, "vehicle");
//    assert_eq!(estate.flags, "periodic");
//    assert_eq!(estate.category, "Navigation");
//
//    assert_eq!(estate.fields.len(), 20);
//}

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
