use crate::engine;
use crate::engine::Tokens::{Field, Message};
use crate::engine::Types::ImcTypeEnum;
use crate::engine::{Parser, Tokens, Types};
use rustache::{Render, VecBuilder};
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{Cursor, Error, Read, Write};
use std::path::{Path, PathBuf};

pub struct RendererArguments<'a> {
    pub templates_dir: &'a Path,
    pub imc_output_dir: &'a Path,
}

enum RenderType {
    Header = 0,
    Message = 1,
    Constants = 2,
    MessageGroup = 3,
}

fn read_template_file(
    args: &RendererArguments,
    template_type: RenderType,
) -> Result<String, Error> {
    let mut tmp = PathBuf::from(args.templates_dir);
    match template_type {
        RenderType::Header => tmp.push("Header.rs.mustache"),
        RenderType::Message => tmp.push("Message.rs.mustache"),
        RenderType::Constants => tmp.push("mod.rs.mustache"),
        RenderType::MessageGroup => tmp.push("MessageGroup.mustache"),
        _ => panic!("unknown template type..."),
    }

    let mut content;
    match File::open(tmp) {
        Ok(mut file) => {
            content = String::new();
            file.read_to_string(&mut content)?;
            Result::Ok(content)
        }
        Err(error) => Result::Err(error),
    }
}

fn render_file(args: &RendererArguments, filename: &str, data: &String) {
    // let mut out_filepath = imc_path.clone();
    let mut out_filepath = PathBuf::new();
    out_filepath.push(args.imc_output_dir);
    let path = ["../imc/", filename, ".rs"].join("");
    out_filepath.push(path);

    match File::create(out_filepath) {
        Ok(mut file) => {
            file.write(data.as_ref()).unwrap();
        }
        Err(err) => panic!("can't open out file"),
    }
}

fn render_fields<'a>(fields: &Vec<Tokens::Field>) -> Option<rustache::VecBuilder<'a>> {
    if fields.len() == 0 {
        return Option::None;
    }

    let mut data: rustache::VecBuilder = rustache::VecBuilder::new();
    for field in fields {
        let mut field_data = rustache::HashBuilder::new();
        field_data = field_data
            .insert("imc-message-field-abbrev", field.abbrev.clone())
            .insert("imc-message-field-type", format!("{}", field.ftype));

        let desc_ret = render_description(&field.desc);
        if desc_ret.is_some() {
            field_data = field_data.insert("imc-message-field-desc", desc_ret.unwrap());
        }

        data = data.push(field_data);
    }

    Option::from(data)
}

// @todo initialize fields
//       perhaps fill default-value while parsing xml?
fn render_fields_initialization<'a>(
    fields: &Vec<Tokens::Field>,
) -> Option<rustache::VecBuilder<'a>> {
    if fields.len() == 0 {
        return Option::None;
    }

    let mut data = rustache::VecBuilder::new();
    for field in fields {
        data = data.push(
            rustache::HashBuilder::new()
                .insert("imc-message-field-abbrev", field.abbrev.clone())
                .insert("imc-message-field-init", Types::get_init_string(&field)),
        );
    }

    Option::from(data)
}

fn render_fields_serialization<'a>(
    fields: &Vec<Tokens::Field>,
) -> Option<rustache::VecBuilder<'a>> {
    if fields.is_empty() {
        return Option::None;
    }

    let mut data = rustache::VecBuilder::new();
    for field in fields {
        let mut ser_str = match &field.ftype.type_enum {
            ImcTypeEnum::Raw | ImcTypeEnum::PlainText => {
                format!("serialize_string!(bfr, self.{})", field.abbrev)
            }
            ImcTypeEnum::U8 => format!("bfr.put_u8(self.{})", field.abbrev),
            ImcTypeEnum::Enum | ImcTypeEnum::Bitfield => panic!("what to do with bitfield and enum.."),
            v => format!("bfr.put_{}_le(self.{})", field.ftype, field.abbrev),
            _ => panic!("unhandled type"),
        };

        data = data.push(rustache::HashBuilder::new().insert("serialization-fn", ser_str));
    }

    Option::from(data)
}

pub fn render_description<'a>(desc: &String) -> Option<VecBuilder<'a>> {
    if desc.is_empty() {
        return Option::None;
    }

    let mut data = rustache::VecBuilder::new();
    let mut parts: Vec<&str> = desc.split('\n').collect::<Vec<&str>>();
    for line in parts.iter() {
        let trim_line = line.trim();
        data = data.push(rustache::HashBuilder::new().insert("desc-line", trim_line));
    }

    Option::from(data)
}

// @todo
pub fn render_enums<'a>(fields: Vec<Tokens::Field>) -> Option<rustache::VecBuilder<'a>> {
    if fields.is_empty() {
        return Option::None;
    }

    let mut enum_builder: rustache::VecBuilder = rustache::VecBuilder::new();
    for field in fields {
        // field is not enum
        if field.enums.is_empty() {
            continue;
        }

        let mut enum_values: rustache::VecBuilder = rustache::VecBuilder::new();
        for value in field.enums {
            enum_values = enum_values.push(
                rustache::HashBuilder::new()
                    .insert("enum-desc", format!("// {}", value.name.trim()))
                    .insert(
                        "enum-name",
                        format!("{}_{}", field.enum_prefix, value.abbrev.trim()),
                    )
                    .insert("enum-value", value.id.trim()),
            );
        }

        enum_builder = enum_builder.push(
            rustache::HashBuilder::new()
                .insert("enum-abbrev", field.abbrev)
                .insert("enum-values", enum_values),
        );
    }

    Option::from(enum_builder)
}

pub fn render_message_groups(args: &RendererArguments, groups: &HashSet<String>) {
    let mut groups_vec = rustache::VecBuilder::new();
    let mut data = rustache::HashBuilder::new();

    for group in groups {
        groups_vec = groups_vec
            .push(rustache::HashBuilder::new().insert("imc-group-abbrev", group.as_str()));
    }

    data = data.insert("imc-message-groups", groups_vec);

    let mut out = Cursor::new(Vec::new());
    match read_template_file(args, RenderType::MessageGroup) {
        Ok(content) => {
            data.render(content.as_str(), &mut out).unwrap();
        }
        Err(error) => panic!("failed to read message groups' template file: {}", error),
    }

    let rendered_data = String::from_utf8(out.into_inner()).unwrap();
    render_file(&args, "MessageGroup", &rendered_data);
}

pub fn render_header(args: &RendererArguments, header: &Tokens::Message) {
    let mut data = rustache::HashBuilder::new()
        .insert("header_desc", render_description(&header.desc).unwrap())
        .insert("header-fields", render_fields(&header.fields).unwrap())
        .insert(
            "header-fields-init",
            render_fields_initialization(&header.fields).unwrap(),
        )
        .insert(
            "imc-serialization",
            render_fields_serialization(&header.fields).unwrap(),
        );

    let mut out = Cursor::new(Vec::new());
    match read_template_file(args, RenderType::Header) {
        Ok(content) => {
            data.render(content.as_str(), &mut out).unwrap();
        }
        Err(error) => panic!("failed to read header template file: {}", error),
    }

    let rendered_data = String::from_utf8(out.into_inner()).unwrap();
    render_file(&args, "Header", &rendered_data);
}

pub fn render_imc_file(args: &RendererArguments, ctx: &Parser::Context) {
    let mut data = rustache::HashBuilder::new()
        .insert("imc_version", ctx.version.clone())
        .insert(
            "imc_sync_number",
            ctx.header
                .fields
                .get(0)
                .unwrap()
                .default_value
                .as_ref()
                .unwrap()
                .as_str(),
        )
        .insert(
            "imc_header_size",
            ctx.header.fixed_serialization_size.to_string(),
        )
        .insert(
            "imc_footer_size",
            ctx.header.fixed_serialization_size.to_string(),
        );

    let mut out = Cursor::new(Vec::new());
    match read_template_file(args, RenderType::Constants) {
        Ok(content) => {
            data.render(content.as_str(), &mut out).unwrap();
        }
        Err(error) => panic!("failed to read header template file"),
    }

    let rendered_data = String::from_utf8(out.into_inner()).unwrap();
    render_file(&args, "mod", &rendered_data);
}

pub fn render_message(args: &RendererArguments, msg: Tokens::Message, group: Option<&String>) {
    let msg_abbrev = msg.abbrev.clone();

    let mut data = rustache::HashBuilder::new();

    // message group
    if group.is_some() {
        data = data
            .insert("imc-has-message-group", true)
            .insert("imc_message_abbrev", msg.abbrev.clone())
            .insert("imc-group-abbrev", group.unwrap().clone());
    }

    // description
    let desc_ret = render_description(&msg.desc);
    if desc_ret.is_some() {
        data = data.insert("imc-message-desc", desc_ret.unwrap());
    }

    data = data
        .insert("imc_message_id", msg.id)
        .insert("imc_message_abbrev", msg.abbrev.clone())
        .insert(
            "imc_message_fixed_serialization_size",
            msg.fixed_serialization_size.to_string(),
        )
        .insert(
            "imc_message_dynamic_serialization_size",
            "unimplemented!();",
        );

    // fields
    let mut ret = render_fields(&msg.fields);
    if ret.is_some() {
        data = data.insert("imc-message-fields", ret.unwrap());
        // fields' initialization
        data = data.insert(
            "imc-message-fields-init",
            render_fields_initialization(&msg.fields).unwrap(),
        );

        // fields' serialization
        data = data.insert(
            "imc-serialization",
            render_fields_serialization(&msg.fields).unwrap(),
        );
    }

    // enumerator
    ret = render_enums(msg.fields);
    if ret.is_some() {
        data = data.insert("imc_message_enums", ret.unwrap());
    }

    let mut out = Cursor::new(Vec::new());
    match read_template_file(args, RenderType::Message) {
        Ok(content) => {
            data.render(content.as_str(), &mut out).unwrap();
        }
        Err(error) => panic!("failed to read msg template file: {}", error),
    }

    let rendered_data = String::from_utf8(out.into_inner()).unwrap();
    render_file(&args, msg_abbrev.as_str(), &rendered_data);
}
