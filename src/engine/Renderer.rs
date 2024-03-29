use crate::engine;

use crate::engine::Types::ImcTypeEnum;
use crate::engine::{Parser, Tokens, Types};
use rustache::{Render, VecBuilder};
use std::collections::HashSet;

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

fn render_file(args: &RendererArguments, filename: &str, data: &str) {
    // let mut out_filepath = imc_path.clone();
    let mut out_filepath = PathBuf::new();
    out_filepath.push(args.imc_output_dir);
    let path = ["../imc/", filename, ".rs"].join("");
    out_filepath.push(path);

    match File::create(out_filepath) {
        Ok(mut file) => {
            file.write_all(data.as_ref()).unwrap();
        }
        Err(_err) => panic!("can't open out file"),
    }
}

fn render_fields<'a>(fields: &[Tokens::Field]) -> Option<rustache::VecBuilder<'a>> {
    if fields.is_empty() {
        return Option::None;
    }

    let mut data: rustache::VecBuilder = rustache::VecBuilder::new();
    for field in fields {
        let mut field_data = rustache::HashBuilder::new();
        field_data = field_data
            .insert(
                "imc-message-field-abbrev",
                format!("_{}", field.abbrev.clone()),
            )
            .insert("imc-message-field-type", format!("{}", field.ftype));

        let desc_ret = render_description(&field.desc);
        if let Some(value) = desc_ret {
            field_data = field_data.insert("imc-message-field-desc", value);
        }

        data = data.push(field_data);
    }

    Option::from(data)
}

// @todo initialize fields
//       perhaps fill default-value while parsing xml?
fn render_fields_initialization<'a>(fields: &[Tokens::Field]) -> Option<rustache::VecBuilder<'a>> {
    if fields.is_empty() {
        return Option::None;
    }

    let mut data = rustache::VecBuilder::new();
    for field in fields {
        data = data.push(
            rustache::HashBuilder::new()
                .insert(
                    "imc-message-field-abbrev",
                    format!("_{}", field.abbrev.clone()),
                )
                .insert("imc-message-field-init", Types::get_init_string(&field)),
        );
    }

    Option::from(data)
}

fn render_fields_serialization<'a>(fields: &[Tokens::Field]) -> Option<rustache::VecBuilder<'a>> {
    if fields.is_empty() {
        return Option::None;
    }

    let mut data = rustache::VecBuilder::new();
    for field in fields {
        data = data.push(
            rustache::HashBuilder::new()
                .insert("serialization-fn", Types::get_serialization_string(&field)),
        );
    }

    Option::from(data)
}

pub fn render_fields_clear<'a>(fields: &[Tokens::Field]) -> Option<rustache::VecBuilder<'a>> {
    if fields.is_empty() {
        return Option::None;
    }

    let mut data = rustache::VecBuilder::new();
    for field in fields {
        data = data.push(
            rustache::HashBuilder::new()
                .insert("imc-message-field-clear", Types::get_clear_string(field)),
        );
    }

    Option::from(data)
}

pub fn render_imc_imports<'a>(
    msg: &Tokens::Message,
    ctx: &Parser::Context,
) -> Option<rustache::VecBuilder<'a>> {
    let mut data = rustache::VecBuilder::new();
    let mut has_imports = false;

    let mut imc_imports: HashSet<String> = HashSet::new();
    let mut group_imports: HashSet<String> = HashSet::new();

    // import trait / group
    if let Some(value) = ctx.message_group.get(msg.abbrev.clone().as_str()) {
        has_imports = true;
        group_imports.insert(value.clone());
    }

    // collect message fields
    for field in &msg.fields {
        match field.ftype.type_enum {
            ImcTypeEnum::Message | ImcTypeEnum::MessageList => {
                match field.ftype.message_type.as_ref() {
                    None => continue,
                    Some(v) => {
                        has_imports = true;

                        match ctx.message_groups.get(v.as_str()) {
                            None => {
                                imc_imports.insert(v.clone());
                            }
                            Some(_value) => {
                                group_imports.insert(_value.clone());
                            }
                        }
                    }
                }
            }
            _ => continue,
        }
    }

    for message_abbrev in imc_imports {
        data = data.push(
            rustache::HashBuilder::new()
                .insert("imc-import-module", message_abbrev.clone())
                .insert("imc-import-abbrev", message_abbrev.clone()),
        );
    }

    for group in group_imports {
        data = data.push(
            rustache::HashBuilder::new()
                .insert("imc-import-module", "MessageGroup")
                .insert("imc-import-abbrev", group),
        );
    }

    if has_imports {
        Option::from(data)
    } else {
        Option::None
    }
}

pub fn render_description<'a>(desc: &str) -> Option<VecBuilder<'a>> {
    if desc.is_empty() {
        return Option::None;
    }

    let mut data = rustache::VecBuilder::new();
    let parts: Vec<&str> = desc.split('\n').collect::<Vec<&str>>();
    for line in parts.iter() {
        let trim_line = line.trim();
        data = data.push(rustache::HashBuilder::new().insert("desc-line", trim_line));
    }

    Option::from(data)
}

// @todo
pub fn render_enums<'a>(fields: &[Tokens::Field]) -> Option<rustache::VecBuilder<'a>> {
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
        for value in &field.enums {
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

        let mut enum_abbrev = field.name.clone();
        enum_abbrev.retain(|c| c != ' ');

        enum_builder = enum_builder.push(
            rustache::HashBuilder::new()
                .insert("enum-abbrev", enum_abbrev)
                .insert("enum-values", enum_values),
        );
    }

    Option::from(enum_builder)
}

pub fn render_dynamic_serialization<'a>(
    fields: &[Tokens::Field],
) -> Option<rustache::VecBuilder<'a>> {
    if fields.is_empty() {
        return Option::None;
    }

    let mut data = rustache::VecBuilder::new();
    let mut is_empty = true;
    for field in fields {
        let ser_str =
            rustache::HashBuilder::new().insert("field-abbrev", field.abbrev.clone().to_string());
        match field.ftype.type_enum {
            ImcTypeEnum::PlainText | ImcTypeEnum::Raw => {
                is_empty = false;
                data = data.push(
                    rustache::HashBuilder::new()
                        .insert("imc-serialization-type-plaintext-raw?", ser_str),
                );
            }
            ImcTypeEnum::Message => {
                is_empty = false;
                data = data.push(
                    rustache::HashBuilder::new().insert("imc-serialization-type-message?", ser_str),
                );
            }
            ImcTypeEnum::MessageList => {
                is_empty = false;
                data = data.push(
                    rustache::HashBuilder::new()
                        .insert("imc-serialization-type-message-list?", ser_str),
                );
            }
            _ => continue,
        }
    }

    if !is_empty {
        Option::from(data)
    } else {
        Option::None
    }
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
    let data = rustache::HashBuilder::new()
        .insert("header_desc", render_description(&header.desc).unwrap())
        .insert("header-fields", render_fields(&header.fields).unwrap())
        .insert(
            "header-fields-init",
            render_fields_initialization(&header.fields).unwrap(),
        )
        .insert(
            "imc-serialization",
            render_fields_serialization(&header.fields).unwrap(),
        )
        .insert(
            "imc-message-clear",
            render_fields_clear(&header.fields).unwrap(),
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

    let mut pubmod_data = rustache::VecBuilder::new();
    for msg in &ctx.messages {
        pubmod_data = pubmod_data
            .push(rustache::HashBuilder::new().insert("imc_message_abbrev", msg.abbrev.clone()));
    }

    data = data.insert("imc-mod-messages", pubmod_data);

    let mut out = Cursor::new(Vec::new());
    match read_template_file(args, RenderType::Constants) {
        Ok(content) => {
            data.render(content.as_str(), &mut out).unwrap();
        }
        Err(_error) => panic!("failed to read header template file"),
    }

    let rendered_data = String::from_utf8(out.into_inner()).unwrap();
    render_file(&args, "mod", &rendered_data);
}

pub fn render_message(
    args: &RendererArguments,
    msg: &Tokens::Message,
    ctx: &engine::Parser::Context,
) {
    let msg_abbrev = msg.abbrev.clone();

    let mut data = rustache::HashBuilder::new();

    // description
    if let Some(value) = render_description(&msg.desc) {
        data = data.insert("imc-message-desc", value);
    }

    data = data
        .insert("imc_message_id", msg.id.clone())
        .insert("imc_message_abbrev", msg.abbrev.clone())
        .insert(
            "imc_message_dynamic_serialization_size",
            "unimplemented!();",
        );

    // message group
    if let Some(value) = ctx.message_group.get(msg.abbrev.clone().as_str()) {
        data = data
            .insert("imc-has-message-group", true)
            .insert("imc_message_abbrev", msg.abbrev.clone())
            .insert("imc-group-abbrev", value.clone());
    }

    // fields
    if let Some(value) = render_fields(&msg.fields) {
        data = data.insert("imc-message-fields", value);
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

        // clear fields
        data = data.insert(
            "imc-message-clear",
            render_fields_clear(&msg.fields).unwrap(),
        );

        // imports
        if let Some(value) = render_imc_imports(&msg, &ctx) {
            data = data.insert("imc-has-message-imports", value);
        }

        // fixed serialization size
        let mut fixed_size = 0;
        for field in &msg.fields {
            match Types::get_fixed_size(field) {
                None => {}
                Some(size) => fixed_size += size,
            }
        }

        data = data.insert("imc-message-fields-fixed-serialization-size", fixed_size);
        let dyn_size = render_dynamic_serialization(&msg.fields);
        match dyn_size {
            None => {}
            Some(v) => data = data.insert("imc-message-dynamic-serialization-size", v),
        }
    }

    // enumerator
    if let Some(value) = render_enums(&msg.fields) {
        data = data.insert("imc_message_enums", value);
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
