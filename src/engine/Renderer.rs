use rustache::Render;
use crate::engine::Tokens::{Message, Field};
use std::env;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Cursor, Read, Write};
use crate::engine::{Tokens, Types};

pub struct RendererArguments<'a> {
    pub templates_dir :&'a Path,
    pub imc_output_dir :&'a Path
}

enum RenderType {
    Header = 0,
    Message = 1,
    Constants = 2,
}

fn get_template_file(args :&RendererArguments, template_type :RenderType) -> PathBuf {
    let mut tmp = PathBuf::from(args.templates_dir);
    match template_type {
        RenderType::Header => tmp.push("Header.rs.in"),
        RenderType::Message => tmp.push("Message.rs.in"),
        RenderType::Constants => tmp.push("imc.rs.in"),
        _ => panic!("unknown template type...")
    }

    tmp
}

fn get_output_file(imc_path :&PathBuf, type_name :&str) -> PathBuf {
    let mut out_filepath = imc_path.clone();
    let path = ["../imc/", type_name, ".rs"].join("");
    out_filepath.push(path);

    out_filepath
}

fn render_fields_string(fields :&Vec<Field>) -> String {
    let mut fields_str :String= String::from("");

    let mut padding = 0;
    for field in fields {
        let ftype = Types::convert(&field);

        let descr_str = format!("// {}\n", field.field_desc);
        fields_str.push_str(format!("{:>width$}", descr_str, width = padding + descr_str.len()).as_str());
        padding = 4;

        let type_str = format!("pub {} :{},\n", field.field_abbrev, ftype);
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
fn render_fields_initialization_string(fields :&Vec<Field>) -> String {
    let mut fields_str :String= String::from("");
    let mut padding = 0;
    for field in fields {
        let init_str = Types::get_init_string(&field);
        let str = format!("{}: {},\n", field.field_abbrev, init_str);
        fields_str.push_str(format!("{:>width$}", str, width=padding + str.len()).as_str());

        padding = 12;
    }

    // remove last \n
    if !fields_str.is_empty() {
        fields_str.pop().unwrap();
    }

    fields_str
}

fn render_fields_serialization_string(fields :&Vec<Field>) -> String {
    let mut fields_str :String= String::from("");
    let mut padding = 0;
    for field in fields {
        let ser_fn = Types::Serialization::get_fn_string(&field);
        let str = format!("bfr.{}(self.{});\n", ser_fn, field.field_abbrev);
        fields_str.push_str(format!("{:>width$}", str, width=padding + str.len()).as_str());

        padding = 8;
    }

    // remove last \n
    if !fields_str.is_empty() {
        fields_str.pop().unwrap();
    }

    fields_str
}

pub fn render_header(args :&RendererArguments, header :Message) {
    let template_filepath = get_template_file(args, RenderType::Header);
    let mut data = rustache::HashBuilder::new();
    let mut out = Cursor::new(Vec::new());

    let fields_str = render_fields_string(&header.fields);
    let fields_init_str = render_fields_initialization_string(&header.fields);
    let fields_serialization_str = render_fields_serialization_string(&header.fields);

    data = data.insert("header_fields", fields_str)
    .insert("header_fields_init", fields_init_str)
    .insert("header_serialize", fields_serialization_str);

    let mut content;
    match File::open(template_filepath) {
        Ok(mut file) => {
            content = String::new();
            file.read_to_string(&mut content).unwrap();
            data.render(content.as_str(), &mut out).unwrap();
        },
        Err(error) => panic!("failed to read header template file")
    }

    println!("{}", String::from_utf8(out.into_inner()).unwrap());
    // println!("{}", get_output_file(&args.imc_output_dir, "HeaderTest"));
    // let path = get_output_file("HeaderTest.rs");
    // match File::create(path) {
    //     Ok(mut file) => {
    //         file.write(content.as_ref()).unwrap();
    //     }
    //     Err(err) => panic!("can't open out file")
    // }
}