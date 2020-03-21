extern crate xml;
extern crate clap;

mod engine;

use clap::{Arg, App};
use std::path::{Path, PathBuf};
use std::env;
use std::fs::File;
use std::io::{Cursor, Read};
use rustache::Render;
use crate::engine::Renderer::RendererArguments;

fn main() {
    let matches = App::new("IMC Rust")
    .version("0.1.0")
    .author("Tiago Marques <tmarques@oceanscan-mst.com>")
    .about("Rust IMC bindings generator")
    .arg(Arg::with_name("imc")
        .short("i")
        .long("imc")
        .takes_value(true)
        .required(true)
        .help("Full path to IMC.xml file"))
    .arg(Arg::with_name("output-dir")
        .short("o")
        .long("output-dir")
        .takes_value(true)
        .required(true)
        .help("Full path to output directory"))
    .arg(Arg::with_name("template-dir")
        .short("t")
        .long("template-dir")
        .takes_value(true)
        .required(true)
        .help("Full path to templates directory"))
    .get_matches();

    // handle program arguments
    let ret = matches.value_of("imc");
    let mut templates_path = PathBuf::new();
    templates_path.push(matches.value_of("template-dir").unwrap());

    let mut out_imc_path = PathBuf::new();
    out_imc_path.push(matches.value_of("output-dir").unwrap());

    // parse xml
    let ctx: engine::Parser::Context;
    match ret {
        Some(v) => {
            println!(".. parsing {}", v);
            ctx = engine::Parser::parse(v);
        }
        None => panic!("missing path to IMC definition. Use --imc option")
    }

    println!("\n.. This was a triumph");
    println!(".. version: {}", ctx.version);
    println!(".. sync number: {}", ctx.header.fields.get(0).unwrap().default_value.as_ref().unwrap());
    println!(".. parsed");
    println!("   .. {} messages", ctx.messages.len());
    println!("   .. {} global enumerators", ctx.global_enums.len());
    println!("   .. {} global bitfields", ctx.global_bitfields.len());

    // render IMC messages
    let mut rnd_args :RendererArguments = RendererArguments {
        templates_dir : templates_path.as_path(),
        imc_output_dir: out_imc_path.as_path()
    };


    println!(".. templates from {}", rnd_args.templates_dir.display());
    println!(".. generating header");

    engine::Renderer::render_header(&rnd_args, ctx.header);
}