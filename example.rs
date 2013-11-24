#[link(name="example")];

#[feature(globs)];
#[feature(macro_rules)];

extern mod argparse;

use std::os;

use argparse::ArgumentParser;
use argparse::arg::*;
use argparse::value::*;

#[path="src/argparse/macros.rs"]
mod macros;

fn main_test(raw_args: &[~str])
{
    let mut parser = ArgumentParser::new();
    parser.description = Some("Example!");

    let opts = ~[
        create_arg!("-h", "--help"; ty = ArgTyBool, help = Some("Show this help and exit.")),
        create_arg!("-S", "--source"),
        create_arg!("filename"),
        create_arg!("count"; ty = ArgTyInteger, default = Some(ArgValInteger(0))),
    ];
    parser.add_arguments(opts);

    let args = match parser.parse_args(raw_args.tail()) {
        Ok(args) => args,
        Err(e) => {
            println!("Error: {}", e.to_str());
            parser.print_help();
            fail!();
        }
    };

    if args.get::<bool>("help")
    {
        parser.print_help();
        return;
    }

    let source: ~str = match args.find("source") {
        Some(s) => s,
        None => ~"source",
    };

    let filename: ~str = args.get("filename");
    let count: int = args.get("count");

    println!("Source: {}", source);
    println!("Filename: {}", filename);
    println!("Count: {}", count);
}

fn main()
{
    main_test(os::args());
}
