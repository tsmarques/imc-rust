mod MessageBuilder;

extern crate xml;
extern crate clap;

use clap::{Arg, App};
use std::fs::File;
use std::io::BufReader;
use xml::EventReader;
use xml::reader::{XmlEvent, Error};
use crate::MessageBuilder::{FieldData, EnumData, MessageData};
use xml::attribute::OwnedAttribute;

fn parse_global_enum(out :&mut Vec<FieldData>, parser : &mut EventReader<BufReader<File>>) {
    let mut i = 0;
    loop {
        match parser.next() {
            Ok(XmlEvent::StartElement { name, attributes, ..}) => {
                match name.local_name.as_str() {
                    "def" => {
                        out.push(FieldData::new());
                        for attr in attributes {
                            let attr_name = attr.name.local_name.as_str();
                            let attr_value = attr.value.trim().to_string();
                            match attr_name {
                                "name" => out[i].field_name = attr_value,
                                "abbrev" => out[i].field_abbrev = attr_value,
                                "prefix" => out[i].field_enum_prefix = attr_value,
                                _ => panic!("unknown field {}", )
                            }
                        }
                    },
                    "value" => parse_field_enum(&mut out[i], &attributes),
                    _ => panic!("global enum: unknown name {}", name)
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                match name.local_name.as_str() {
                    "enumerations" | "bitfields" => break,
                    "def" => i+=1,
                    "value" => {},
                    _ => panic!("unkwnon end element {}", name.local_name)
                }
            }
            _ => {},
        }
    }
}

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
            "bitfield-def" => {}, // TODO handle global bitfield
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

fn parse_message_attributes(message :&mut MessageBuilder::MessageData, attributes :&Vec<OwnedAttribute>) {
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


fn parse_messages(ctx : &mut Context, parser : &mut EventReader<BufReader<File>>) {
    let mut i = 0;

    loop {
        match parser.next() {
            Err(err) => panic!("parse_message: error: {}", err),
            Ok(XmlEvent::StartElement {name, attributes, ..}) => {
                match name.local_name.as_str() {
                    "message" => {
                        ctx.messages.push(MessageBuilder::MessageData::new());
                        parse_message_attributes(&mut ctx.messages[i], &attributes);

                        let ret = parse_message(parser);
                        ctx.messages[i].fields = ret.1;
                        ctx.messages[i].desc = ret.0;
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
    }
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

fn parse(xml_path : &str) -> Context {
    let file = File::open(xml_path).unwrap();
    let file = BufReader::new(file);

    let mut parser = EventReader::new(file);
    let mut depth = 0;

    // footer, header, flags, message-group, bitfields, enumerations, units, serialization, types,
    // description

    let mut ctx :Context = Context {
        global_enums: vec![],
        global_bitfields: vec![],
        units: vec![],
        messages: vec![]
    };
    loop {
        let e = parser.next();
        match e {
            Ok(XmlEvent::StartDocument {..}) => {},
            Ok(XmlEvent::StartElement {name, attributes, ..}) => {
                match name.local_name.as_str() {
                    "type" |
                    "description" | "types" | "serialization" |
                    "units" | "message-groups" | "flags" | "header"
                    => { ignore_tag(name.local_name.as_str(), &mut parser) }, // ignore, for now, this headers
                    "enumerations" => parse_global_enum(&mut ctx.global_enums, &mut parser),
                    "bitfields" => parse_global_enum(&mut ctx.global_bitfields, &mut parser),
                    "messages" => {
                        let ret = attributes.iter().find(|x| x.name.local_name == "version");
                        match ret {
                            None => panic!("parse: missing \"version\" field in \"message\" : {}", name.local_name),
                            Some(v) => println!("Generating IMC version {}", v),
                        }
                    },
                    "footer" => {
                        ignore_tag("footer", &mut parser);
                        parse_messages(&mut ctx, &mut parser);
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

    ctx
}

struct Context {
    pub global_enums :Vec<FieldData>,
    pub global_bitfields :Vec<FieldData>,
    pub units :Vec<(String, String)>,
    pub messages :Vec<MessageData>
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
    let ctx :Context;
    match ret {
        Some(v) => ctx = parse(v),
        None => panic!("missing path to IMC definition. Use --imc option")
    }
}

#[test]
fn full_format() {
    let ctx = parse("/home/tsm/ws/imc-rust/src/xml/tests/test-imc-full.xml");

    assert_eq!(ctx.messages.len(), 3);
    assert_global_enums(&ctx);
    assert_global_bitfields(&ctx);

    let mut i = 0;
    let mut n_fields = 0;

    assert_eq!(ctx.messages[i].name, "Entity State");
    assert_eq!(ctx.messages[i].abbrev, "EntityState");
    assert_eq!(ctx.messages[i].source, "vehicle");
    assert_eq!(ctx.messages[i].category, "Core");
    assert_eq!(ctx.messages[i].desc, "State reported by an entity in the vehicle. The source entity is
            identified in the message header.");

    assert_eq!(ctx.messages[i].fields[0].field_name, "State");
    assert_eq!(ctx.messages[i].fields[0].field_abbrev, "state");
    assert_eq!(ctx.messages[i].fields[0].field_desc, "State of entity.");
    assert_eq!(ctx.messages[i].fields[0].field_type, "uint8_t");
    assert_eq!(ctx.messages[i].fields[0].field_unit, "Enumerated");
    assert_eq!(ctx.messages[i].fields[0].field_enum_prefix, "ESTA");
    assert_eq!(ctx.messages[i].fields[0].field_enum.len(), 5);
    assert_eq!(ctx.messages[i].fields[0].field_enum[0].id, "0");
    assert_eq!(ctx.messages[i].fields[0].field_enum[0].name, "Bootstrapping");
    assert_eq!(ctx.messages[i].fields[0].field_enum[0].abbrev, "BOOT");
    assert_eq!(ctx.messages[i].fields[0].field_enum[1].id, "1");
    assert_eq!(ctx.messages[i].fields[0].field_enum[1].name, "Normal Operation");
    assert_eq!(ctx.messages[i].fields[0].field_enum[1].abbrev, "NORMAL");
    assert_eq!(ctx.messages[i].fields[0].field_enum[2].id, "2");
    assert_eq!(ctx.messages[i].fields[0].field_enum[2].name, "Fault");
    assert_eq!(ctx.messages[i].fields[0].field_enum[2].abbrev, "FAULT");
    assert_eq!(ctx.messages[i].fields[0].field_enum[3].id, "3");
    assert_eq!(ctx.messages[i].fields[0].field_enum[3].name, "Error");
    assert_eq!(ctx.messages[i].fields[0].field_enum[3].abbrev, "ERROR");
    assert_eq!(ctx.messages[i].fields[0].field_enum[4].id, "4");
    assert_eq!(ctx.messages[i].fields[0].field_enum[4].name, "Failure");
    assert_eq!(ctx.messages[i].fields[0].field_enum[4].abbrev, "FAILURE");
    n_fields += 1;

    assert_eq!(ctx.messages[i].fields[1].field_name, "Flags");
    assert_eq!(ctx.messages[i].fields[1].field_abbrev, "flags");
    assert_eq!(ctx.messages[i].fields[1].field_desc, "Complementary entity state flags.");
    assert_eq!(ctx.messages[i].fields[1].field_type, "uint8_t");
    assert_eq!(ctx.messages[i].fields[1].field_unit, "Bitfield");
    assert_eq!(ctx.messages[i].fields[1].field_enum_prefix, "EFLA");
    assert_eq!(ctx.messages[i].fields[1].field_enum.len(), 1);
    assert_eq!(ctx.messages[i].fields[1].field_enum[0].id, "0x01");
    assert_eq!(ctx.messages[i].fields[1].field_enum[0].name, "Human Intervention Required");
    assert_eq!(ctx.messages[i].fields[1].field_enum[0].abbrev, "HUMAN_INTERVENTION");
    n_fields += 1;

    assert_eq!(ctx.messages[i].fields[2].field_name, "Complementary description");
    assert_eq!(ctx.messages[i].fields[2].field_abbrev, "description");
    assert_eq!(ctx.messages[i].fields[2].field_desc, "Complementary human-readable description of entity state.");
    assert_eq!(ctx.messages[i].fields[2].field_type, "plaintext");
    assert!(ctx.messages[i].fields[2].field_unit.is_empty());
    assert!(ctx.messages[i].fields[2].field_enum_prefix.is_empty());
    assert_eq!(ctx.messages[i].fields[2].field_enum.len(), 0);
    n_fields += 1;

    // make sure we tested all fields
    assert_eq!(ctx.messages[i].fields.len(), 3);
    assert_eq!(ctx.messages[i].fields.len(), n_fields);

    i += 1;
    n_fields = 0;
    assert_eq!(ctx.messages[i].name, "Transport Bindings");
    assert_eq!(ctx.messages[i].abbrev, "TransportBindings");
    assert_eq!(ctx.messages[i].source, "vehicle");
    assert_eq!(ctx.messages[i].category, "Core");
    assert_eq!(ctx.messages[i].desc, "Message generated when tasks bind to messages.");
    assert_eq!(ctx.messages[i].fields[0].field_name, "Consumer name");
    assert_eq!(ctx.messages[i].fields[0].field_abbrev, "consumer");
    assert_eq!(ctx.messages[i].fields[0].field_desc, "The name of the consumer (e.g. task name).");
    assert_eq!(ctx.messages[i].fields[0].field_type, "plaintext");
    assert!(ctx.messages[i].fields[0].field_unit.is_empty());
    assert!(ctx.messages[i].fields[0].field_enum_prefix.is_empty());
    assert_eq!(ctx.messages[i].fields[0].field_enum.len(), 0);
    n_fields += 1;

    assert_eq!(ctx.messages[i].fields[1].field_name, "Message Identifier");
    assert_eq!(ctx.messages[i].fields[1].field_abbrev, "message_id");
    assert_eq!(ctx.messages[i].fields[1].field_desc, "The id of the message to be listened to.");
    assert_eq!(ctx.messages[i].fields[1].field_type, "uint16_t");
    assert!(ctx.messages[i].fields[1].field_unit.is_empty());
    assert!(ctx.messages[i].fields[1].field_enum_prefix.is_empty());
    assert_eq!(ctx.messages[i].fields[1].field_enum.len(), 0);
    n_fields += 1;

    assert_eq!(ctx.messages[i].fields.len(), 2);
    assert_eq!(ctx.messages[i].fields.len(), n_fields);
}

fn assert_global_enums(ctx :&Context) {
    assert_eq!(ctx.global_enums.len(), 4);

    assert_eq!(ctx.global_enums[0].field_name, "Speed Units");
    assert_eq!(ctx.global_enums[0].field_abbrev, "SpeedUnits");
    assert_eq!(ctx.global_enums[0].field_enum_prefix, "SUNITS");
    assert_eq!(ctx.global_enums[0].field_enum.len(), 3);
    assert_eq!(ctx.global_enums[0].field_enum[0].id, "0");
    assert_eq!(ctx.global_enums[0].field_enum[0].name, "Meters per second");
    assert_eq!(ctx.global_enums[0].field_enum[0].abbrev, "METERS_PS");
    assert_eq!(ctx.global_enums[0].field_enum[1].id, "1");
    assert_eq!(ctx.global_enums[0].field_enum[1].name, "RPM");
    assert_eq!(ctx.global_enums[0].field_enum[1].abbrev, "RPM");
    assert_eq!(ctx.global_enums[0].field_enum[2].id, "2");
    assert_eq!(ctx.global_enums[0].field_enum[2].name, "Percentage");
    assert_eq!(ctx.global_enums[0].field_enum[2].abbrev, "PERCENTAGE");

    assert_eq!(ctx.global_enums[1].field_name, "System Type");
    assert_eq!(ctx.global_enums[1].field_abbrev, "SystemType");
    assert_eq!(ctx.global_enums[1].field_enum_prefix, "SYSTEMTYPE");
    assert_eq!(ctx.global_enums[1].field_enum.len(), 9);
    assert_eq!(ctx.global_enums[1].field_enum[0].id, "0");
    assert_eq!(ctx.global_enums[1].field_enum[0].name, "CCU");
    assert_eq!(ctx.global_enums[1].field_enum[0].abbrev, "CCU");
    assert_eq!(ctx.global_enums[1].field_enum[1].id, "1");
    assert_eq!(ctx.global_enums[1].field_enum[1].name, "Human-portable Sensor");
    assert_eq!(ctx.global_enums[1].field_enum[1].abbrev, "HUMANSENSOR");
    assert_eq!(ctx.global_enums[1].field_enum[2].id, "2");
    assert_eq!(ctx.global_enums[1].field_enum[2].name, "UUV");
    assert_eq!(ctx.global_enums[1].field_enum[2].abbrev, "UUV");
    assert_eq!(ctx.global_enums[1].field_enum[3].id, "3");
    assert_eq!(ctx.global_enums[1].field_enum[3].name, "USV");
    assert_eq!(ctx.global_enums[1].field_enum[3].abbrev, "USV");
    assert_eq!(ctx.global_enums[1].field_enum[4].id, "4");
    assert_eq!(ctx.global_enums[1].field_enum[4].name, "UAV");
    assert_eq!(ctx.global_enums[1].field_enum[4].abbrev, "UAV");
    assert_eq!(ctx.global_enums[1].field_enum[5].id, "5");
    assert_eq!(ctx.global_enums[1].field_enum[5].name, "UGV");
    assert_eq!(ctx.global_enums[1].field_enum[5].abbrev, "UGV");
    assert_eq!(ctx.global_enums[1].field_enum[6].id, "6");
    assert_eq!(ctx.global_enums[1].field_enum[6].name, "Static sensor");
    assert_eq!(ctx.global_enums[1].field_enum[6].abbrev, "STATICSENSOR");
    assert_eq!(ctx.global_enums[1].field_enum[7].id, "7");
    assert_eq!(ctx.global_enums[1].field_enum[7].name, "Mobile sensor");
    assert_eq!(ctx.global_enums[1].field_enum[7].abbrev, "MOBILESENSOR");
    assert_eq!(ctx.global_enums[1].field_enum[8].id, "8");
    assert_eq!(ctx.global_enums[1].field_enum[8].name, "Wireless Sensor Network");
    assert_eq!(ctx.global_enums[1].field_enum[8].abbrev, "WSN");

    assert_eq!(ctx.global_enums[2].field_name, "Z Units");
    assert_eq!(ctx.global_enums[2].field_abbrev, "ZUnits");
    assert_eq!(ctx.global_enums[2].field_enum_prefix, "Z");
    assert_eq!(ctx.global_enums[2].field_enum.len(), 4);
    assert_eq!(ctx.global_enums[2].field_enum[0].id, "0");
    assert_eq!(ctx.global_enums[2].field_enum[0].name, "None");
    assert_eq!(ctx.global_enums[2].field_enum[0].abbrev, "NONE");
    assert_eq!(ctx.global_enums[1].field_enum[1].id, "1");
    assert_eq!(ctx.global_enums[2].field_enum[1].name, "Depth");
    assert_eq!(ctx.global_enums[2].field_enum[1].abbrev, "DEPTH");
    assert_eq!(ctx.global_enums[2].field_enum[2].id, "2");
    assert_eq!(ctx.global_enums[2].field_enum[2].name, "Altitude");
    assert_eq!(ctx.global_enums[2].field_enum[2].abbrev, "ALTITUDE");
    assert_eq!(ctx.global_enums[2].field_enum[3].id, "3");
    assert_eq!(ctx.global_enums[2].field_enum[3].name, "Height");
    assert_eq!(ctx.global_enums[2].field_enum[3].abbrev, "HEIGHT");

    assert_eq!(ctx.global_enums[3].field_name, "RSSI Units");
    assert_eq!(ctx.global_enums[3].field_abbrev, "RSSIUnits");
    assert_eq!(ctx.global_enums[3].field_enum_prefix, "RSSIUNITS");
    assert_eq!(ctx.global_enums[3].field_enum.len(), 2);
    assert_eq!(ctx.global_enums[3].field_enum[0].id, "0");
    assert_eq!(ctx.global_enums[3].field_enum[0].name, "Decibel");
    assert_eq!(ctx.global_enums[3].field_enum[0].abbrev, "dB");
    assert_eq!(ctx.global_enums[3].field_enum[1].id, "1");
    assert_eq!(ctx.global_enums[3].field_enum[1].name, "Percentage");
    assert_eq!(ctx.global_enums[3].field_enum[1].abbrev, "PERCENTAGE");
}

fn assert_global_bitfields(ctx :&Context) {
    assert_eq!(ctx.global_bitfields.len(), 2);

    assert_eq!(ctx.global_bitfields[0].field_enum.len(), 14);
    assert_eq!(ctx.global_bitfields[0].field_enum_prefix, "CL");
    assert_eq!(ctx.global_bitfields[0].field_name, "Control Loops Mask");
    assert_eq!(ctx.global_bitfields[0].field_abbrev, "CLoopsMask");

    assert_eq!(ctx.global_bitfields[0].field_enum[0].id, "0x00000000");
    assert_eq!(ctx.global_bitfields[0].field_enum[0].abbrev, "NONE");
    assert_eq!(ctx.global_bitfields[0].field_enum[0].name, "None");

    assert_eq!(ctx.global_bitfields[0].field_enum[1].id, "0x00000001");
    assert_eq!(ctx.global_bitfields[0].field_enum[1].abbrev, "PATH");
    assert_eq!(ctx.global_bitfields[0].field_enum[1].name, "Path Control");

    assert_eq!(ctx.global_bitfields[0].field_enum[2].id, "0x00000002");
    assert_eq!(ctx.global_bitfields[0].field_enum[2].abbrev, "TELEOPERATION");
    assert_eq!(ctx.global_bitfields[0].field_enum[2].name, "Teleoperation Control");

    assert_eq!(ctx.global_bitfields[0].field_enum[3].id, "0x00000004");
    assert_eq!(ctx.global_bitfields[0].field_enum[3].abbrev, "ALTITUDE");
    assert_eq!(ctx.global_bitfields[0].field_enum[3].name, "Altitude Control");

    assert_eq!(ctx.global_bitfields[0].field_enum[4].id, "0x00000008");
    assert_eq!(ctx.global_bitfields[0].field_enum[4].abbrev, "DEPTH");
    assert_eq!(ctx.global_bitfields[0].field_enum[4].name, "Depth Control");

    assert_eq!(ctx.global_bitfields[0].field_enum[5].id, "0x00000010");
    assert_eq!(ctx.global_bitfields[0].field_enum[5].abbrev, "ROLL");
    assert_eq!(ctx.global_bitfields[0].field_enum[5].name, "Roll Control");

    assert_eq!(ctx.global_bitfields[0].field_enum[6].id, "0x00000020");
    assert_eq!(ctx.global_bitfields[0].field_enum[6].abbrev, "PITCH");
    assert_eq!(ctx.global_bitfields[0].field_enum[6].name, "Pitch Control");

    assert_eq!(ctx.global_bitfields[0].field_enum[7].id, "0x00000040");
    assert_eq!(ctx.global_bitfields[0].field_enum[7].abbrev, "YAW");
    assert_eq!(ctx.global_bitfields[0].field_enum[7].name, "Yaw Control");

    assert_eq!(ctx.global_bitfields[0].field_enum[8].id, "0x00000080");
    assert_eq!(ctx.global_bitfields[0].field_enum[8].abbrev, "SPEED");
    assert_eq!(ctx.global_bitfields[0].field_enum[8].name, "Speed Control");

    assert_eq!(ctx.global_bitfields[0].field_enum[9].id, "0x00000100");
    assert_eq!(ctx.global_bitfields[0].field_enum[9].abbrev, "YAW_RATE");
    assert_eq!(ctx.global_bitfields[0].field_enum[9].name, "Yaw Rate Control");

    assert_eq!(ctx.global_bitfields[0].field_enum[10].id, "0x00000400");
    assert_eq!(ctx.global_bitfields[0].field_enum[10].abbrev, "TORQUE");
    assert_eq!(ctx.global_bitfields[0].field_enum[10].name, "Torque Control");

    assert_eq!(ctx.global_bitfields[0].field_enum[11].id, "0x00000800");
    assert_eq!(ctx.global_bitfields[0].field_enum[11].abbrev, "FORCE");
    assert_eq!(ctx.global_bitfields[0].field_enum[11].name, "Force Control");

    assert_eq!(ctx.global_bitfields[0].field_enum[12].id, "0x80000000");
    assert_eq!(ctx.global_bitfields[0].field_enum[12].abbrev, "NO_OVERRIDE");
    assert_eq!(ctx.global_bitfields[0].field_enum[12].name, "Non-overridable control");

    assert_eq!(ctx.global_bitfields[0].field_enum[13].id, "0xFFFFFFFF");
    assert_eq!(ctx.global_bitfields[0].field_enum[13].abbrev, "ALL");
    assert_eq!(ctx.global_bitfields[0].field_enum[13].name, "All");

    assert_eq!(ctx.global_bitfields[1].field_enum.len(), 7);
    assert_eq!(ctx.global_bitfields[1].field_enum_prefix, "OPL");
    assert_eq!(ctx.global_bitfields[1].field_name, "Operational Limits Mask");
    assert_eq!(ctx.global_bitfields[1].field_abbrev, "OpLimitsMask");

    assert_eq!(ctx.global_bitfields[1].field_enum[0].id, "0x01");
    assert_eq!(ctx.global_bitfields[1].field_enum[0].name, "Maximum Depth");
    assert_eq!(ctx.global_bitfields[1].field_enum[0].abbrev, "MAX_DEPTH");

    assert_eq!(ctx.global_bitfields[1].field_enum[1].id, "0x02");
    assert_eq!(ctx.global_bitfields[1].field_enum[1].name, "Minimum Altitude");
    assert_eq!(ctx.global_bitfields[1].field_enum[1].abbrev, "MIN_ALT");

    assert_eq!(ctx.global_bitfields[1].field_enum[2].id, "0x04");
    assert_eq!(ctx.global_bitfields[1].field_enum[2].name, "Maximum Altitude");
    assert_eq!(ctx.global_bitfields[1].field_enum[2].abbrev, "MAX_ALT");

    assert_eq!(ctx.global_bitfields[1].field_enum[3].id, "0x08");
    assert_eq!(ctx.global_bitfields[1].field_enum[3].name, "Minimum Speed");
    assert_eq!(ctx.global_bitfields[1].field_enum[3].abbrev, "MIN_SPEED");

    assert_eq!(ctx.global_bitfields[1].field_enum[4].id, "0x10");
    assert_eq!(ctx.global_bitfields[1].field_enum[4].name, "Maximum Speed");
    assert_eq!(ctx.global_bitfields[1].field_enum[4].abbrev, "MAX_SPEED");

    assert_eq!(ctx.global_bitfields[1].field_enum[5].id, "0x20");
    assert_eq!(ctx.global_bitfields[1].field_enum[5].name, "Maximum Vertical Rate");
    assert_eq!(ctx.global_bitfields[1].field_enum[5].abbrev, "MAX_VRATE");

    assert_eq!(ctx.global_bitfields[1].field_enum[6].id, "0x40");
    assert_eq!(ctx.global_bitfields[1].field_enum[6].name, "Operation Area");
    assert_eq!(ctx.global_bitfields[1].field_enum[6].abbrev, "AREA");
}