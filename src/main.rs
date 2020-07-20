#![allow(non_snake_case)]

extern crate clap;
extern crate xml;

mod engine;

use crate::engine::Renderer::RendererArguments;
use clap::{App, Arg};




use std::path::{PathBuf};

fn main() {
    let matches = App::new("IMC Rust")
        .version("0.1.0")
        .author("Tiago Marques <tmarques@oceanscan-mst.com>")
        .about("Rust IMC bindings generator")
        .arg(
            Arg::with_name("imc")
                .short("i")
                .long("imc")
                .takes_value(true)
                .required(true)
                .help("Full path to IMC.xml file"),
        )
        .arg(
            Arg::with_name("output-dir")
                .short("o")
                .long("output-dir")
                .takes_value(true)
                .required(true)
                .help("Full path to output directory"),
        )
        .arg(
            Arg::with_name("template-dir")
                .short("t")
                .long("template-dir")
                .takes_value(true)
                .required(true)
                .help("Full path to templates directory"),
        )
        .arg(
            Arg::with_name("only-message")
                .short("m")
                .long("only-message")
                .takes_value(true)
                .required(false)
                .help("Render only the selected message. Debug only"),
        )
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
        None => panic!("missing path to IMC definition. Use --imc option"),
    }

    println!("\n.. This was a triumph");
    println!(".. version: {}", ctx.version);
    println!(
        ".. sync number: {}",
        ctx.header
            .fields
            .get(0)
            .unwrap()
            .default_value
            .as_ref()
            .unwrap()
    );
    println!(".. parsed");
    println!("   .. {} messages", ctx.messages.len());
    println!("   .. {} global enumerators", ctx.global_enums.len());
    println!("   .. {} global bitfields", ctx.global_bitfields.len());
    println!("   .. {} message groups", ctx.message_groups.len());
    println!("      .. {:?}", ctx.message_groups);
    println!("      .. {:?}", ctx.message_group);

    // render IMC messages
    let rnd_args: RendererArguments = RendererArguments {
        templates_dir: templates_path.as_path(),
        imc_output_dir: out_imc_path.as_path(),
    };

    println!(".. templates from {}", rnd_args.templates_dir.display());
    println!(".. rendering header");
    engine::Renderer::render_header(&rnd_args, &ctx.header);

    println!(".. rendering message groups");
    engine::Renderer::render_message_groups(&rnd_args, &ctx.message_groups);

    println!(".. rendering IMC file");
    engine::Renderer::render_imc_file(&rnd_args, &ctx);

    let ret = matches.value_of("only-message");
    if ret.is_none() {
        println!(".. rendering messages");
        for msg in &ctx.messages {
            println!("   .. {}", msg.abbrev);
            engine::Renderer::render_message(&rnd_args, msg, &ctx);
        }
    } else {
        let msg_abbrev = ret.unwrap();
        println!(".. DEBUG rendering only {}", msg_abbrev);

        let it = &mut ctx.messages.iter();
        let mut found = false;
        let mut ret = it.next();
        while ret.is_some() && !found {
            let m = ret.unwrap();

            if m.abbrev == msg_abbrev {
                found = true;
                engine::Renderer::render_message(&rnd_args, &m, &ctx);
            }

            ret = it.next();
        }

        if !found {
            println!("   .. ERROR can't find message");
        }
    }

    println!(".. finished");
}
