use rustache::Render;
use crate::engine::Tokens::{Message, Field};
use std::env;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Cursor, Read, Write, Error};
use crate::engine::{Tokens, Types, Parser};
use crate::engine;

pub struct RendererArguments<'a> {
    pub templates_dir :&'a Path,
    pub imc_output_dir :&'a Path
}

enum RenderType {
    Header = 0,
    Message = 1,
    Constants = 2,
}

fn read_template_file(args :&RendererArguments, template_type :RenderType) -> Result<String, Error> {
    let mut tmp = PathBuf::from(args.templates_dir);
    match template_type {
        RenderType::Header => tmp.push("Header.rs.in"),
        RenderType::Message => tmp.push("Message.rs.in"),
        RenderType::Constants => tmp.push("mod.rs.in"),
        _ => panic!("unknown template type...")
    }

    let mut content;
    match File::open(tmp) {
        Ok(mut file) => {
            content = String::new();
            file.read_to_string(&mut content)?;
            Result::Ok(content)
        },
        Err(error) => Result::Err(error)
    }
}

fn render_file(args :&RendererArguments, filename :&str, data :&String) {
    // let mut out_filepath = imc_path.clone();
    let mut out_filepath = PathBuf::new();
    out_filepath.push(args.imc_output_dir);
    let path = ["../imc/", filename, ".rs"].join("");
    out_filepath.push(path);

    match File::create(out_filepath) {
        Ok(mut file) => { file.write(data.as_ref()).unwrap();  }
        Err(err)     => panic!("can't open out file")
    }
}

fn render_fields_string(fields :&Vec<Tokens::Field>) -> String {
    let mut fields_str :String= String::from("");

    let mut padding = 0;
    for field in fields {
        let descr_str = format!("// {}\n", field.desc);
        fields_str.push_str(format!("{:>width$}", descr_str, width = padding + descr_str.len()).as_str());
        padding = 4;

        let type_str = format!("pub {} :{},\n", field.abbrev, field.ftype);
        fields_str.push_str(format!("{:>width$}", type_str, width = padding + type_str.len()).as_str());
    }

    // remove last \n
    if !fields_str.is_empty() {
        fields_str.pop().unwrap();
    }

    fields_str
}

// @todo initialize fields
//       perhaps fill default-value while parsing xml?
fn render_fields_initialization_string(fields :&Vec<Tokens::Field>) -> String {
    let mut fields_str :String= String::from("");
    let mut padding = 0;
    for field in fields {
        let init_str = Types::get_init_string(&field);
        let str = format!("{}: {},\n", field.abbrev, init_str);
        fields_str.push_str(format!("{:>width$}", str, width=padding + str.len()).as_str());

        padding = 12;
    }

    // remove last \n
    if !fields_str.is_empty() {
        fields_str.pop().unwrap();
    }

    fields_str
}

fn render_fields_serialization_string(fields :&Vec<Tokens::Field>) -> String {
    let mut fields_str :String= String::from("");
    let mut padding = 0;
    for field in fields {
        let ser_fn = Types::Serialization::get_fn_string(&field);
        let str = format!("bfr.{}(self.{});\n", ser_fn, field.abbrev);
        fields_str.push_str(format!("{:>width$}", str, width=padding + str.len()).as_str());

        padding = 8;
    }

    // remove last \n
    if !fields_str.is_empty() {
        fields_str.pop().unwrap();
    }

    fields_str
}

pub fn render_header(args :&RendererArguments, header :&Tokens::Message) {
    let fields_str = render_fields_string(&header.fields);
    let fields_init_str = render_fields_initialization_string(&header.fields);
    let fields_serialization_str = render_fields_serialization_string(&header.fields);

    let mut data = rustache::HashBuilder::new()
    .insert("header_fields", fields_str)
    .insert("header_fields_init", fields_init_str)
    .insert("header_serialize", fields_serialization_str);

    let mut out = Cursor::new(Vec::new());
    match read_template_file(args, RenderType::Header) {
        Ok(content) => { data.render(content.as_str(), &mut out).unwrap(); },
        Err(error) => panic!("failed to read header template file: {}", error)
    }

    let rendered_data = String::from_utf8(out.into_inner()).unwrap();
    render_file(&args, "Header", &rendered_data);
}

pub fn render_imc_file(args :&RendererArguments, ctx :&Parser::Context) {
    let mut data = rustache::HashBuilder::new()
    .insert("imc_version", ctx.version.clone())
    .insert("imc_sync_number", ctx.header.fields.get(0).unwrap().default_value.as_ref().unwrap().as_str())
    .insert("imc_header_size", ctx.header.fixed_serialization_size.to_string())
    .insert("imc_footer_size", ctx.header.fixed_serialization_size.to_string());

    let mut out = Cursor::new(Vec::new());
    match read_template_file(args, RenderType::Constants) {
        Ok(content) => { data.render(content.as_str(), &mut out).unwrap(); },
        Err(error) => panic!("failed to read header template file")
    }

    let rendered_data = String::from_utf8(out.into_inner()).unwrap();
    render_file(&args, "mod", &rendered_data);
}