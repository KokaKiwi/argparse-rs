#[link(
    name = "argparse-sample",
    vers = "0.1.0",
    uuid = "e9b76b27-4720-4b58-a0a4-d7a4e408e76a"
)];

#[author = "KokaKiwi <kokakiwi@kokakiwi.net>"];
#[license = "MIT"];

#[crate_type = "bin"];

extern mod argparse(vers = "0.1.0");

use std::os;
use argparse::{ArgumentParser, Argument};

fn main()
{
    let cli_args = os::args().clone();
    let mut parser = ArgumentParser::new();

    let opts = ~[
        Argument::new_short('h'),
        Argument::new_short('v'),
    ];
    parser.add_arguments(opts);

    // let args = parser.parse_args(cli_args.slice_from(1));
}
