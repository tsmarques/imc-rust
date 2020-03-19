extern crate xml;
extern crate clap;

mod engine;

use clap::{Arg, App};

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
    println!(".. sync number: {}", ctx.header.fields.get(0).unwrap().field_default_value);
    println!(".. parsed");
    println!("   .. {} messages", ctx.messages.len());
    println!("   .. {} global enumerators", ctx.global_enums.len());
    println!("   .. {} global bitfields", ctx.global_bitfields.len());
}