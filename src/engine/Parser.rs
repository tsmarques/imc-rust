use crate::engine::Types::ImcTypeEnum;
use crate::engine::{Tokens, Types};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use xml::attribute::OwnedAttribute;
use xml::reader::{Error, XmlEvent};
use xml::EventReader;

pub struct Context {
    pub version: String,
    pub header: Tokens::Message,
    pub footer: Tokens::Message,
    pub global_enums: Vec<Tokens::Field>,
    pub global_bitfields: Vec<Tokens::Field>,
    pub messages: Vec<Tokens::Message>,
    // Map a given message to its message-group if any
    pub message_group: HashMap<String, String>,
    // Available message groups
    pub message_groups: HashSet<String>,
}

// Parse single message group
fn parse_group(
    out_map: &mut HashMap<String, String>,
    group_abbrev: &String,
    parser: &mut EventReader<BufReader<File>>,
) {
    loop {
        match parser.next() {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                if name.local_name != "message-type" {
                    panic!(
                        "error: parser: group: was expecting \"message-type\" but got \"{}\"",
                        name.local_name
                    );
                }

                if attributes.len() != 1 {
                    panic!("error: parser: group: \"message-type\" should have only 1 attribute.. got {} instead",
                           attributes.len())
                }

                let attr_name = attributes.get(0).unwrap().name.local_name.clone();
                if attr_name != "abbrev" {
                    panic!("error: parser: group: \"message-type\" should have \"abbrev\" field.. got {} instead",
                           attr_name)
                }

                // @fixme need clone?
                out_map.insert(
                    attributes.get(0).unwrap().value.clone(),
                    group_abbrev.clone(),
                );
            }
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == "message-type" {
                    continue;
                } else if name.local_name == "message-group" {
                    break;
                } else {
                    panic!(
                        "error: parser: group: was expecting \"</message-type>\".. got \"</{}>\"",
                        name.local_name
                    );
                }
            }
            _ => {}
        }
    }
}

// Parse all existing message groups
fn parse_message_groups(ctx: &mut Context, parser: &mut EventReader<BufReader<File>>) {
    loop {
        match parser.next() {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                if name.local_name == "message-group" {
                    if attributes.len() != 2 {
                        panic!("error: parser: group: \"message-group\" should have 2 attributes.. got {} instead",
                               attributes.len())
                    }

                    let group_abbrev = &attributes.get(1).unwrap().value;
                    ctx.message_groups.insert(group_abbrev.parse().unwrap());

                    parse_group(&mut ctx.message_group, &group_abbrev, parser);
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == "message-groups" {
                    break;
                } else {
                    panic!("error: parser: groups: was expecting \"</message-group>\" or \"</message-groups>\".. got \"</{}>\"",
                           name.local_name);
                }
            }
            _ => {}
        }
    }
}

fn parse_global_enum(out: &mut Vec<Tokens::Field>, parser: &mut EventReader<BufReader<File>>) {
    let mut i = 0;
    loop {
        match parser.next() {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => match name.local_name.as_str() {
                "def" => {
                    out.push(Tokens::Field::new());
                    for attr in attributes {
                        let attr_name = attr.name.local_name.as_str();
                        let attr_value = attr.value.trim().to_string();
                        match attr_name {
                            "name" => out[i].name = attr_value,
                            "abbrev" => out[i].abbrev = attr_value,
                            "prefix" => out[i].enum_prefix = attr_value,
                            _ => panic!("unknown field {}",),
                        }
                    }
                }
                "value" => parse_field_enum(&mut out[i], &attributes),
                _ => panic!("global enum: unknown name {}", name),
            },
            Ok(XmlEvent::EndElement { name }) => match name.local_name.as_str() {
                "enumerations" | "bitfields" => break,
                "def" => i += 1,
                "value" => {}
                _ => panic!("unkwnon end element {}", name.local_name),
            },
            _ => {}
        }
    }
}

fn parse_field_enum(field: &mut Tokens::Field, attr: &Vec<OwnedAttribute>) {
    let mut field_enum: Tokens::Enum = Tokens::Enum {
        id: "".to_string(),
        name: "".to_string(),
        abbrev: "".to_string(),
    };

    for a in attr {
        match a.name.local_name.as_str() {
            "id" => field_enum.id = a.value.clone(),
            "name" => field_enum.name = a.value.clone(),
            "abbrev" => field_enum.abbrev = a.value.clone(),
            _ => panic!(
                "enum : {} : unhandled enum attribute {}",
                field.name, a.name.local_name
            ),
        }
    }

    field.enums.push(field_enum);
}

fn parse_field_attributes(field: &mut Tokens::Field, attr: &Vec<OwnedAttribute>) {
    for attr in attr {
        let value = attr.value.trim().to_string();
        match attr.name.local_name.as_str() {
            "name" => field.name = value,
            "abbrev" => field.abbrev = value,
            "type" => field.ftype.type_enum = Types::convert(value.as_str()),
            "message-type" => field.ftype.message_type = Option::from(value),
            "unit" => field.unit = value,
            "prefix" => field.enum_prefix = value,
            "max" => field.max_value = Option::from(value),
            "min" => field.min_value = Option::from(value),
            "enum-def" => {} // TODO handle global enumerator
            "value" => field.default_value = Option::from(value),
            "bitfield-def" => {} // TODO handle global bitfield
            "fixed" => field.read_only = (value == "true"),
            _ => panic!("unhandled field attribute : {}", attr.name),
        }
    }
}

fn parse_description(parser: &mut EventReader<BufReader<File>>) -> String {
    let mut desc: String = String::from("");
    loop {
        match parser.next() {
            Ok(XmlEvent::Characters(content)) => {
                desc = content.trim().to_string();
            }
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == "description" {
                    break;
                }
            }
            _ => {}
        }
    }

    desc
}

fn parse_message(parser: &mut EventReader<BufReader<File>>) -> (String, Vec<Tokens::Field>) {
    let mut fields = vec![];
    // flag if next <description> is from <message ...>
    let mut is_message_description = true;
    let mut message_descr: String = String::from("");

    let mut i = 0;
    loop {
        match parser.next() {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => match name.local_name.as_str() {
                "field" => {
                    is_message_description = false;
                    fields.push(Tokens::Field::new());
                    parse_field_attributes(&mut fields[i], &attributes)
                }
                "description" => {
                    if !is_message_description {
                        fields[i].desc = parse_description(parser);
                        is_message_description = true;
                    } else {
                        message_descr = parse_description(parser);
                        is_message_description = false;
                    }
                }
                "value" => match fields[i].unit.as_str() {
                    "Enumerated" | "Bitfield" => parse_field_enum(&mut fields[i], &attributes),
                    _ => panic!("parse_field_enum: was expecting enum"),
                },
                _ => panic!("parse field : unknown name {}", name.local_name),
            },
            Ok(XmlEvent::EndElement { name }) => match name.local_name.as_str() {
                "field" => i += 1,
                "message" => break,
                "value" => {}
                _ => panic!("parse_fields: unknown termination \"{}\"", name.local_name),
            },
            Ok(_) => {}
            Err(e) => {
                panic!("parse field error: {}", e);
                break;
            }
        }
    }

    (message_descr, fields)
}

fn parse_message_attributes(message: &mut Tokens::Message, attributes: &Vec<OwnedAttribute>) {
    for attr in attributes {
        let value = attr.value.trim().to_string();
        match attr.name.local_name.as_str() {
            "id" => message.id = value,
            "name" => message.name = value,
            "abbrev" => message.abbrev = value,
            "source" => message.source = value,
            "flags" => message.flags = value,
            "category" => message.category = value,
            "used-by" => {}
            _ => panic!("unhandled field attribute : {}", attr.name),
        }
    }
}

fn parse_messages(ctx: &mut Context, parser: &mut EventReader<BufReader<File>>) {
    let mut i = 0;

    loop {
        match parser.next() {
            Err(err) => panic!("parse_message: error: {}", err),
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => match name.local_name.as_str() {
                "message" => {
                    ctx.messages.push(Tokens::Message::new());
                    parse_message_attributes(&mut ctx.messages[i], &attributes);

                    let ret = parse_message(parser);
                    ctx.messages[i].fields = ret.1;
                    ctx.messages[i].desc = ret.0;
                    i += 1;
                }
                _ => panic!("parse_message: unexpected tag: {}", name.local_name),
            },
            Ok(XmlEvent::Whitespace(_)) => {}
            Ok(XmlEvent::EndElement { .. }) => {}
            Ok(XmlEvent::EndDocument) => break,
            Ok(_) => panic!("parse_message: unhandled event"),
        }
    }
}

fn parse_special(
    out: &mut Vec<Tokens::Field>,
    desc: &mut String,
    parser: &mut EventReader<BufReader<File>>,
) {
    let mut i = 0;
    let mut is_header_desc = true;
    loop {
        match parser.next() {
            Err(err) => panic!("parse_special: error: {}", err),
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => match name.local_name.as_str() {
                "description" => {
                    if is_header_desc {
                        *desc = parse_description(parser);
                        is_header_desc = false;
                    } else {
                        out[i].desc = parse_description(parser);
                    }
                }
                "field" => {
                    out.push(Tokens::Field::new());
                    parse_field_attributes(&mut out[i], &attributes)
                }
                _ => {}
            },
            Ok(XmlEvent::EndElement { name }) => match name.local_name.as_str() {
                "field" => i += 1,
                "header" | "footer" => break,
                _ => panic!("parse_special: unknown {}", name.local_name),
            },
            _ => {}
        }
    }
}

fn ignore_tag(xml_tag: &str, parser: &mut EventReader<BufReader<File>>) {
    loop {
        match parser.next() {
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == xml_tag {
                    break;
                }
            }
            _ => {}
        }
    }
}

pub(crate) fn parse(xml_path: &str) -> Context {
    let file = File::open(xml_path).unwrap();
    let file = BufReader::new(file);

    let mut parser = EventReader::new(file);

    let mut ctx: Context = Context {
        version: "".to_string(),
        header: Tokens::Message::new(),
        footer: Tokens::Message::new(),
        global_enums: vec![],
        global_bitfields: vec![],
        messages: vec![],
        message_group: HashMap::new(),
        message_groups: HashSet::new(),
    };

    loop {
        let e = parser.next();
        match e {
            Ok(XmlEvent::StartDocument { .. }) => {}
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                match name.local_name.as_str() {
                    "type" | "description" | "types" | "serialization" | "units" | "flags" => {
                        ignore_tag(name.local_name.as_str(), &mut parser)
                    } // ignore, for now, this headers
                    "message-groups" => parse_message_groups(&mut ctx, &mut parser),
                    "enumerations" => parse_global_enum(&mut ctx.global_enums, &mut parser),
                    "bitfields" => parse_global_enum(&mut ctx.global_bitfields, &mut parser),
                    "header" => {
                        parse_special(&mut ctx.header.fields, &mut ctx.header.desc, &mut parser)
                    }
                    "messages" => {
                        let ret = attributes.iter().find(|x| x.name.local_name == "version");
                        match ret {
                            None => panic!(
                                "parse: missing \"version\" field in \"message\" : {}",
                                name.local_name
                            ),
                            Some(v) => ctx.version = v.value.clone(),
                        }
                    }
                    "footer" => {
                        parse_special(&mut ctx.footer.fields, &mut ctx.footer.desc, &mut parser);
                        parse_messages(&mut ctx, &mut parser);
                        break;
                    }
                    _ => panic!("parse: unhandled tag {}", name.local_name),
                }
            }
            Ok(XmlEvent::EndElement { name }) => {}
            Ok(XmlEvent::EndDocument) => break,
            Ok(XmlEvent::Whitespace(..)) | Ok(XmlEvent::Characters(..)) => {}
            _ => panic!("parse: unhandled event"),
        }
    }

    ctx
}

// @todo test message type and message list
#[test]
fn full_format() {
    let ctx = parse("/home/tsm/ws/imc-rust/src/engine/tests/test-imc-full.xml");

    assert_eq!(ctx.messages.len(), 3);
    assert_global_enums(&ctx);
    assert_global_bitfields(&ctx);
    assert_header(&ctx);
    assert_footer(&ctx);

    let mut i = 0;
    let mut n_fields = 0;

    assert_eq!(ctx.messages[i].name, "Entity State");
    assert_eq!(ctx.messages[i].abbrev, "EntityState");
    assert_eq!(ctx.messages[i].source, "vehicle");
    assert_eq!(ctx.messages[i].category, "Core");
    assert_eq!(
        ctx.messages[i].desc,
        "State reported by an entity in the vehicle. The source entity is
            identified in the message header."
    );

    assert_eq!(ctx.messages[i].fields[0].name, "State");
    assert_eq!(ctx.messages[i].fields[0].abbrev, "state");
    assert_eq!(ctx.messages[i].fields[0].desc, "State of entity.");
    assert_eq!(ctx.messages[i].fields[0].ftype.type_enum, ImcTypeEnum::U8);
    assert_eq!(ctx.messages[i].fields[0].unit, "Enumerated");
    assert_eq!(ctx.messages[i].fields[0].enum_prefix, "ESTA");
    assert_eq!(ctx.messages[i].fields[0].read_only, false);
    assert_eq!(ctx.messages[i].fields[0].enums.len(), 5);
    assert_eq!(ctx.messages[i].fields[0].enums[0].id, "0");
    assert_eq!(ctx.messages[i].fields[0].enums[0].name, "Bootstrapping");
    assert_eq!(ctx.messages[i].fields[0].enums[0].abbrev, "BOOT");
    assert_eq!(ctx.messages[i].fields[0].enums[1].id, "1");
    assert_eq!(ctx.messages[i].fields[0].enums[1].name, "Normal Operation");
    assert_eq!(ctx.messages[i].fields[0].enums[1].abbrev, "NORMAL");
    assert_eq!(ctx.messages[i].fields[0].enums[2].id, "2");
    assert_eq!(ctx.messages[i].fields[0].enums[2].name, "Fault");
    assert_eq!(ctx.messages[i].fields[0].enums[2].abbrev, "FAULT");
    assert_eq!(ctx.messages[i].fields[0].enums[3].id, "3");
    assert_eq!(ctx.messages[i].fields[0].enums[3].name, "Error");
    assert_eq!(ctx.messages[i].fields[0].enums[3].abbrev, "ERROR");
    assert_eq!(ctx.messages[i].fields[0].enums[4].id, "4");
    assert_eq!(ctx.messages[i].fields[0].enums[4].name, "Failure");
    assert_eq!(ctx.messages[i].fields[0].enums[4].abbrev, "FAILURE");
    n_fields += 1;

    assert_eq!(ctx.messages[i].fields[1].name, "Flags");
    assert_eq!(ctx.messages[i].fields[1].abbrev, "flags");
    assert_eq!(
        ctx.messages[i].fields[1].desc,
        "Complementary entity state flags."
    );
    assert_eq!(ctx.messages[i].fields[1].ftype.type_enum, ImcTypeEnum::U8);
    assert_eq!(ctx.messages[i].fields[1].unit, "Bitfield");
    assert_eq!(ctx.messages[i].fields[1].enum_prefix, "EFLA");
    assert_eq!(ctx.messages[i].fields[1].read_only, false);
    assert_eq!(ctx.messages[i].fields[1].enums.len(), 1);
    assert_eq!(ctx.messages[i].fields[1].enums[0].id, "0x01");
    assert_eq!(
        ctx.messages[i].fields[1].enums[0].name,
        "Human Intervention Required"
    );
    assert_eq!(
        ctx.messages[i].fields[1].enums[0].abbrev,
        "HUMAN_INTERVENTION"
    );
    n_fields += 1;

    assert_eq!(ctx.messages[i].fields[2].name, "Complementary description");
    assert_eq!(ctx.messages[i].fields[2].abbrev, "description");
    assert_eq!(
        ctx.messages[i].fields[2].desc,
        "Complementary human-readable description of entity state."
    );
    assert_eq!(ctx.messages[i].fields[2].ftype.type_enum, ImcTypeEnum::PlainText);
    assert_eq!(ctx.messages[i].fields[2].read_only, false);
    assert!(ctx.messages[i].fields[2].unit.is_empty());
    assert!(ctx.messages[i].fields[2].enum_prefix.is_empty());
    assert_eq!(ctx.messages[i].fields[2].enums.len(), 0);
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
    assert_eq!(
        ctx.messages[i].desc,
        "Message generated when tasks bind to messages."
    );
    assert_eq!(ctx.messages[i].fields[0].name, "Consumer name");
    assert_eq!(ctx.messages[i].fields[0].abbrev, "consumer");
    assert_eq!(
        ctx.messages[i].fields[0].desc,
        "The name of the consumer (e.g. task name)."
    );
    assert_eq!(ctx.messages[i].fields[0].ftype.type_enum, ImcTypeEnum::PlainText);
    assert_eq!(ctx.messages[i].fields[0].read_only, false);
    assert!(ctx.messages[i].fields[0].unit.is_empty());
    assert!(ctx.messages[i].fields[0].enum_prefix.is_empty());
    assert_eq!(ctx.messages[i].fields[0].enums.len(), 0);
    n_fields += 1;

    assert_eq!(ctx.messages[i].fields[1].name, "Message Identifier");
    assert_eq!(ctx.messages[i].fields[1].abbrev, "message_id");
    assert_eq!(
        ctx.messages[i].fields[1].desc,
        "The id of the message to be listened to."
    );
    assert_eq!(ctx.messages[i].fields[1].ftype.type_enum, ImcTypeEnum::U16);
    assert_eq!(ctx.messages[i].fields[1].read_only, false);
    assert!(ctx.messages[i].fields[1].unit.is_empty());
    assert!(ctx.messages[i].fields[1].enum_prefix.is_empty());
    assert_eq!(ctx.messages[i].fields[1].enums.len(), 0);
    n_fields += 1;

    assert_eq!(ctx.messages[i].fields.len(), 2);
    assert_eq!(ctx.messages[i].fields.len(), n_fields);
}

fn assert_global_enums(ctx: &Context) {
    assert_eq!(ctx.global_enums.len(), 4);

    assert_eq!(ctx.global_enums[0].name, "Speed Units");
    assert_eq!(ctx.global_enums[0].abbrev, "SpeedUnits");
    assert_eq!(ctx.global_enums[0].enum_prefix, "SUNITS");
    assert_eq!(ctx.global_enums[0].enums.len(), 3);
    assert_eq!(ctx.global_enums[0].enums[0].id, "0");
    assert_eq!(ctx.global_enums[0].enums[0].name, "Meters per second");
    assert_eq!(ctx.global_enums[0].enums[0].abbrev, "METERS_PS");
    assert_eq!(ctx.global_enums[0].enums[1].id, "1");
    assert_eq!(ctx.global_enums[0].enums[1].name, "RPM");
    assert_eq!(ctx.global_enums[0].enums[1].abbrev, "RPM");
    assert_eq!(ctx.global_enums[0].enums[2].id, "2");
    assert_eq!(ctx.global_enums[0].enums[2].name, "Percentage");
    assert_eq!(ctx.global_enums[0].enums[2].abbrev, "PERCENTAGE");

    assert_eq!(ctx.global_enums[1].name, "System Type");
    assert_eq!(ctx.global_enums[1].abbrev, "SystemType");
    assert_eq!(ctx.global_enums[1].enum_prefix, "SYSTEMTYPE");
    assert_eq!(ctx.global_enums[1].enums.len(), 9);
    assert_eq!(ctx.global_enums[1].enums[0].id, "0");
    assert_eq!(ctx.global_enums[1].enums[0].name, "CCU");
    assert_eq!(ctx.global_enums[1].enums[0].abbrev, "CCU");
    assert_eq!(ctx.global_enums[1].enums[1].id, "1");
    assert_eq!(ctx.global_enums[1].enums[1].name, "Human-portable Sensor");
    assert_eq!(ctx.global_enums[1].enums[1].abbrev, "HUMANSENSOR");
    assert_eq!(ctx.global_enums[1].enums[2].id, "2");
    assert_eq!(ctx.global_enums[1].enums[2].name, "UUV");
    assert_eq!(ctx.global_enums[1].enums[2].abbrev, "UUV");
    assert_eq!(ctx.global_enums[1].enums[3].id, "3");
    assert_eq!(ctx.global_enums[1].enums[3].name, "USV");
    assert_eq!(ctx.global_enums[1].enums[3].abbrev, "USV");
    assert_eq!(ctx.global_enums[1].enums[4].id, "4");
    assert_eq!(ctx.global_enums[1].enums[4].name, "UAV");
    assert_eq!(ctx.global_enums[1].enums[4].abbrev, "UAV");
    assert_eq!(ctx.global_enums[1].enums[5].id, "5");
    assert_eq!(ctx.global_enums[1].enums[5].name, "UGV");
    assert_eq!(ctx.global_enums[1].enums[5].abbrev, "UGV");
    assert_eq!(ctx.global_enums[1].enums[6].id, "6");
    assert_eq!(ctx.global_enums[1].enums[6].name, "Static sensor");
    assert_eq!(ctx.global_enums[1].enums[6].abbrev, "STATICSENSOR");
    assert_eq!(ctx.global_enums[1].enums[7].id, "7");
    assert_eq!(ctx.global_enums[1].enums[7].name, "Mobile sensor");
    assert_eq!(ctx.global_enums[1].enums[7].abbrev, "MOBILESENSOR");
    assert_eq!(ctx.global_enums[1].enums[8].id, "8");
    assert_eq!(ctx.global_enums[1].enums[8].name, "Wireless Sensor Network");
    assert_eq!(ctx.global_enums[1].enums[8].abbrev, "WSN");

    assert_eq!(ctx.global_enums[2].name, "Z Units");
    assert_eq!(ctx.global_enums[2].abbrev, "ZUnits");
    assert_eq!(ctx.global_enums[2].enum_prefix, "Z");
    assert_eq!(ctx.global_enums[2].enums.len(), 4);
    assert_eq!(ctx.global_enums[2].enums[0].id, "0");
    assert_eq!(ctx.global_enums[2].enums[0].name, "None");
    assert_eq!(ctx.global_enums[2].enums[0].abbrev, "NONE");
    assert_eq!(ctx.global_enums[1].enums[1].id, "1");
    assert_eq!(ctx.global_enums[2].enums[1].name, "Depth");
    assert_eq!(ctx.global_enums[2].enums[1].abbrev, "DEPTH");
    assert_eq!(ctx.global_enums[2].enums[2].id, "2");
    assert_eq!(ctx.global_enums[2].enums[2].name, "Altitude");
    assert_eq!(ctx.global_enums[2].enums[2].abbrev, "ALTITUDE");
    assert_eq!(ctx.global_enums[2].enums[3].id, "3");
    assert_eq!(ctx.global_enums[2].enums[3].name, "Height");
    assert_eq!(ctx.global_enums[2].enums[3].abbrev, "HEIGHT");

    assert_eq!(ctx.global_enums[3].name, "RSSI Units");
    assert_eq!(ctx.global_enums[3].abbrev, "RSSIUnits");
    assert_eq!(ctx.global_enums[3].enum_prefix, "RSSIUNITS");
    assert_eq!(ctx.global_enums[3].enums.len(), 2);
    assert_eq!(ctx.global_enums[3].enums[0].id, "0");
    assert_eq!(ctx.global_enums[3].enums[0].name, "Decibel");
    assert_eq!(ctx.global_enums[3].enums[0].abbrev, "dB");
    assert_eq!(ctx.global_enums[3].enums[1].id, "1");
    assert_eq!(ctx.global_enums[3].enums[1].name, "Percentage");
    assert_eq!(ctx.global_enums[3].enums[1].abbrev, "PERCENTAGE");
}

fn assert_header(ctx: &Context) {
    assert_eq!(ctx.header.fields.len(), 8);
    assert_eq!(ctx.header.fields[0].name, "Synchronization Number");
    assert_eq!(ctx.header.fields[0].abbrev, "sync");
    assert_eq!(ctx.header.fields[0].ftype.type_enum, ImcTypeEnum::U16);
    assert_eq!(
        ctx.header.fields[0].default_value.as_ref().unwrap(),
        "0xFE54"
    );
    assert!(ctx.header.fields[0].read_only);

    assert_eq!(ctx.header.fields[1].name, "Message Identification Number");
    assert_eq!(ctx.header.fields[1].abbrev, "mgid");
    assert_eq!(ctx.header.fields[1].ftype.type_enum, ImcTypeEnum::U16);
    assert_eq!(ctx.header.fields[1].read_only, false);

    assert_eq!(ctx.header.fields[2].name, "Message size");
    assert_eq!(ctx.header.fields[2].abbrev, "size");
    assert_eq!(ctx.header.fields[2].ftype.type_enum, ImcTypeEnum::U16);
    assert_eq!(ctx.header.fields[2].unit, "byte");
    assert_eq!(ctx.header.fields[2].read_only, false);

    assert_eq!(ctx.header.fields[3].name, "Time stamp");
    assert_eq!(ctx.header.fields[3].abbrev, "timestamp");
    assert_eq!(ctx.header.fields[3].ftype.type_enum, ImcTypeEnum::Fp64);
    assert_eq!(ctx.header.fields[3].unit, "s");
    assert_eq!(ctx.header.fields[3].read_only, false);

    assert_eq!(ctx.header.fields[4].name, "Source Address");
    assert_eq!(ctx.header.fields[4].abbrev, "src");
    assert_eq!(ctx.header.fields[4].ftype.type_enum, ImcTypeEnum::U16);
    assert_eq!(ctx.header.fields[4].read_only, false);

    assert_eq!(ctx.header.fields[5].name, "Source Entity");
    assert_eq!(ctx.header.fields[5].abbrev, "src_ent");
    assert_eq!(ctx.header.fields[5].ftype.type_enum, ImcTypeEnum::U8);
    assert_eq!(ctx.header.fields[5].read_only, false);

    assert_eq!(ctx.header.fields[6].name, "Destination Address");
    assert_eq!(ctx.header.fields[6].abbrev, "dst");
    assert_eq!(ctx.header.fields[6].ftype.type_enum, ImcTypeEnum::U16);
    assert_eq!(ctx.header.fields[6].read_only, false);

    assert_eq!(ctx.header.fields[7].name, "Destination Entity");
    assert_eq!(ctx.header.fields[7].abbrev, "dst_ent");
    assert_eq!(ctx.header.fields[7].ftype.type_enum, ImcTypeEnum::U8);
    assert_eq!(ctx.header.fields[7].read_only, false);
}

fn assert_footer(ctx: &Context) {
    assert_eq!(ctx.footer.fields.len(), 1);
    assert_eq!(ctx.footer.fields[0].name, "Check Sum (CRC-16-IBM)");
    assert_eq!(ctx.footer.fields[0].abbrev, "crc16");
    assert_eq!(ctx.footer.fields[0].ftype.type_enum, ImcTypeEnum::U16);
    assert_eq!(ctx.footer.fields[0].read_only, false);
}

fn assert_global_bitfields(ctx: &Context) {
    assert_eq!(ctx.global_bitfields.len(), 2);

    assert_eq!(ctx.global_bitfields[0].enums.len(), 14);
    assert_eq!(ctx.global_bitfields[0].enum_prefix, "CL");
    assert_eq!(ctx.global_bitfields[0].name, "Control Loops Mask");
    assert_eq!(ctx.global_bitfields[0].abbrev, "CLoopsMask");

    assert_eq!(ctx.global_bitfields[0].enums[0].id, "0x00000000");
    assert_eq!(ctx.global_bitfields[0].enums[0].abbrev, "NONE");
    assert_eq!(ctx.global_bitfields[0].enums[0].name, "None");

    assert_eq!(ctx.global_bitfields[0].enums[1].id, "0x00000001");
    assert_eq!(ctx.global_bitfields[0].enums[1].abbrev, "PATH");
    assert_eq!(ctx.global_bitfields[0].enums[1].name, "Path Control");

    assert_eq!(ctx.global_bitfields[0].enums[2].id, "0x00000002");
    assert_eq!(ctx.global_bitfields[0].enums[2].abbrev, "TELEOPERATION");
    assert_eq!(
        ctx.global_bitfields[0].enums[2].name,
        "Teleoperation Control"
    );

    assert_eq!(ctx.global_bitfields[0].enums[3].id, "0x00000004");
    assert_eq!(ctx.global_bitfields[0].enums[3].abbrev, "ALTITUDE");
    assert_eq!(ctx.global_bitfields[0].enums[3].name, "Altitude Control");

    assert_eq!(ctx.global_bitfields[0].enums[4].id, "0x00000008");
    assert_eq!(ctx.global_bitfields[0].enums[4].abbrev, "DEPTH");
    assert_eq!(ctx.global_bitfields[0].enums[4].name, "Depth Control");

    assert_eq!(ctx.global_bitfields[0].enums[5].id, "0x00000010");
    assert_eq!(ctx.global_bitfields[0].enums[5].abbrev, "ROLL");
    assert_eq!(ctx.global_bitfields[0].enums[5].name, "Roll Control");

    assert_eq!(ctx.global_bitfields[0].enums[6].id, "0x00000020");
    assert_eq!(ctx.global_bitfields[0].enums[6].abbrev, "PITCH");
    assert_eq!(ctx.global_bitfields[0].enums[6].name, "Pitch Control");

    assert_eq!(ctx.global_bitfields[0].enums[7].id, "0x00000040");
    assert_eq!(ctx.global_bitfields[0].enums[7].abbrev, "YAW");
    assert_eq!(ctx.global_bitfields[0].enums[7].name, "Yaw Control");

    assert_eq!(ctx.global_bitfields[0].enums[8].id, "0x00000080");
    assert_eq!(ctx.global_bitfields[0].enums[8].abbrev, "SPEED");
    assert_eq!(ctx.global_bitfields[0].enums[8].name, "Speed Control");

    assert_eq!(ctx.global_bitfields[0].enums[9].id, "0x00000100");
    assert_eq!(ctx.global_bitfields[0].enums[9].abbrev, "YAW_RATE");
    assert_eq!(ctx.global_bitfields[0].enums[9].name, "Yaw Rate Control");

    assert_eq!(ctx.global_bitfields[0].enums[10].id, "0x00000400");
    assert_eq!(ctx.global_bitfields[0].enums[10].abbrev, "TORQUE");
    assert_eq!(ctx.global_bitfields[0].enums[10].name, "Torque Control");

    assert_eq!(ctx.global_bitfields[0].enums[11].id, "0x00000800");
    assert_eq!(ctx.global_bitfields[0].enums[11].abbrev, "FORCE");
    assert_eq!(ctx.global_bitfields[0].enums[11].name, "Force Control");

    assert_eq!(ctx.global_bitfields[0].enums[12].id, "0x80000000");
    assert_eq!(ctx.global_bitfields[0].enums[12].abbrev, "NO_OVERRIDE");
    assert_eq!(
        ctx.global_bitfields[0].enums[12].name,
        "Non-overridable control"
    );

    assert_eq!(ctx.global_bitfields[0].enums[13].id, "0xFFFFFFFF");
    assert_eq!(ctx.global_bitfields[0].enums[13].abbrev, "ALL");
    assert_eq!(ctx.global_bitfields[0].enums[13].name, "All");

    assert_eq!(ctx.global_bitfields[1].enums.len(), 7);
    assert_eq!(ctx.global_bitfields[1].enum_prefix, "OPL");
    assert_eq!(ctx.global_bitfields[1].name, "Operational Limits Mask");
    assert_eq!(ctx.global_bitfields[1].abbrev, "OpLimitsMask");

    assert_eq!(ctx.global_bitfields[1].enums[0].id, "0x01");
    assert_eq!(ctx.global_bitfields[1].enums[0].name, "Maximum Depth");
    assert_eq!(ctx.global_bitfields[1].enums[0].abbrev, "MAX_DEPTH");

    assert_eq!(ctx.global_bitfields[1].enums[1].id, "0x02");
    assert_eq!(ctx.global_bitfields[1].enums[1].name, "Minimum Altitude");
    assert_eq!(ctx.global_bitfields[1].enums[1].abbrev, "MIN_ALT");

    assert_eq!(ctx.global_bitfields[1].enums[2].id, "0x04");
    assert_eq!(ctx.global_bitfields[1].enums[2].name, "Maximum Altitude");
    assert_eq!(ctx.global_bitfields[1].enums[2].abbrev, "MAX_ALT");

    assert_eq!(ctx.global_bitfields[1].enums[3].id, "0x08");
    assert_eq!(ctx.global_bitfields[1].enums[3].name, "Minimum Speed");
    assert_eq!(ctx.global_bitfields[1].enums[3].abbrev, "MIN_SPEED");

    assert_eq!(ctx.global_bitfields[1].enums[4].id, "0x10");
    assert_eq!(ctx.global_bitfields[1].enums[4].name, "Maximum Speed");
    assert_eq!(ctx.global_bitfields[1].enums[4].abbrev, "MAX_SPEED");

    assert_eq!(ctx.global_bitfields[1].enums[5].id, "0x20");
    assert_eq!(
        ctx.global_bitfields[1].enums[5].name,
        "Maximum Vertical Rate"
    );
    assert_eq!(ctx.global_bitfields[1].enums[5].abbrev, "MAX_VRATE");

    assert_eq!(ctx.global_bitfields[1].enums[6].id, "0x40");
    assert_eq!(ctx.global_bitfields[1].enums[6].name, "Operation Area");
    assert_eq!(ctx.global_bitfields[1].enums[6].abbrev, "AREA");
}
