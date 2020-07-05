use crate::engine;
use crate::engine::Tokens::{Field, Message};
use crate::engine::{Parser, Tokens, Types};
use rustache::Render;
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

fn render_fields_string(fields: &Vec<Tokens::Field>) -> String {
    let mut fields_str: String = String::from("");

    let mut padding = 0;
    for field in fields {
        padding = 4;
        fields_str.push_str(render_description_string(&field.desc, padding).as_str());

        let type_str = format!("pub {} :{},\n\n", field.abbrev, field.ftype);
        fields_str
            .push_str(format!("{:>width$}", type_str, width = padding + type_str.len()).as_str());
    }

    // remove last \n
    if !fields_str.is_empty() {
        fields_str.pop().unwrap();
    }

    fields_str
}

// @todo initialize fields
//       perhaps fill default-value while parsing xml?
fn render_fields_initialization_string(fields: &Vec<Tokens::Field>) -> String {
    let mut fields_str: String = String::from("");
    let mut padding = 0;
    for field in fields {
        let init_str = Types::get_init_string(&field);
        let str = format!("{}: {},\n", field.abbrev, init_str);
        fields_str.push_str(format!("{:>width$}", str, width = padding + str.len()).as_str());

        padding = 12;
    }

    // remove last \n
    if !fields_str.is_empty() {
        fields_str.pop().unwrap();
    }

    fields_str
}

fn render_fields_serialization_string(fields: &Vec<Tokens::Field>) -> String {
    let mut fields_str: String = String::from("");
    let mut padding = 0;
    for field in fields {
        let ser_fn = Types::Serialization::get_fn_string(&field);
        let str = format!("bfr.{}(self.{});\n", ser_fn, field.abbrev);
        fields_str.push_str(format!("{:>width$}", str, width = padding + str.len()).as_str());

        padding = 8;
    }

    // remove last \n
    if !fields_str.is_empty() {
        fields_str.pop().unwrap();
    }

    fields_str
}

pub fn render_description_string(desc: &String, padding: usize) -> String {
    let mut desc_rend: String = String::from("");
    let mut parts = desc.split('\n').collect::<Vec<&str>>();

    for line in parts {
        let trim_line = line.trim();
        desc_rend = desc_rend + format!("{:>width$}", "// ", width = padding + 3).as_str();
        desc_rend = desc_rend + trim_line + "\n";
    }

    desc_rend
}

// @todo
pub fn render_enums<'a>(fields: Vec<Tokens::Field>) -> Option<rustache::VecBuilder<'a>> {
    let mut enum_builder: rustache::VecBuilder = rustache::VecBuilder::new();

    let mut is_empty = true;
    for field in fields {
        // field is not enum
        if field.enums.is_empty() {
            continue;
        }

        is_empty = false;

        let mut enum_values: rustache::VecBuilder = rustache::VecBuilder::new();
        for value in field.enums {
            enum_values = enum_values.push(
                rustache::HashBuilder::new()
                    .insert("enum-desc", format!("// {}", value.name.trim()))
                    .insert("enum-name", format!("{}_{}", field.enum_prefix, value.abbrev.trim()))
                    .insert("enum-value", value.id.trim()),
            );
        }

        enum_builder = enum_builder.push(
            rustache::HashBuilder::new()
                .insert("enum-abbrev", field.abbrev)
                .insert("enum-values", enum_values),
        );
    }

    if is_empty {
        Option::None
    } else {
        Option::from(enum_builder)
    }
}

pub fn render_header(args: &RendererArguments, header: &Tokens::Message) {
    let fields_str = render_fields_string(&header.fields);
    let fields_init_str = render_fields_initialization_string(&header.fields);
    let fields_serialization_str = render_fields_serialization_string(&header.fields);

    let mut data = rustache::HashBuilder::new()
        .insert("header_desc", render_description_string(&header.desc, 0))
        .insert("header_fields", fields_str)
        .insert("header_fields_init", fields_init_str)
        .insert("header_serialize", fields_serialization_str);

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

pub fn render_message(args: &RendererArguments, msg: Tokens::Message) {
    let fields_str = render_fields_string(&msg.fields);
    let fields_init_str = render_fields_initialization_string(&msg.fields);
    let fields_serialization_str = render_fields_serialization_string(&msg.fields);
    let msg_abbrev = msg.abbrev.clone();

    let mut data = rustache::HashBuilder::new()
        .insert("imc_message_desc", render_description_string(&msg.desc, 0))
        .insert("imc_message_id", msg.id)
        .insert("imc_message_abbrev", msg.abbrev)
        .insert("imc_message_fields", fields_str)
        .insert("imc_message_fields_init", fields_init_str)
        .insert(
            "imc_message_fixed_serialization_size",
            msg.fixed_serialization_size.to_string(),
        )
        .insert(
            "imc_message_dynamic_serialization_size",
            "unimplemented!();",
        )
        .insert("imc_message_serialize", fields_serialization_str);

    let ret = render_enums(msg.fields);
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
