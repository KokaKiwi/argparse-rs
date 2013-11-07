#[link(
    name = "argparse",
    vers = "0.1.0",
    uuid = "e21f6c0a-deac-4d66-8dc9-b116410d67df"
)];

#[author = "KokaKiwi <kokakiwi@kokakiwi.net>"];
#[license = "MIT"];

#[crate_type = "lib"];

use std::vec;

#[deriving(Clone)]
pub struct ArgumentParser
{
    args: ~[Argument],
}

#[deriving(Clone)]
pub struct Argument
{
    short_names: ~[char],
    long_names: ~[~str],
}

impl ArgumentParser
{
    pub fn new() -> ArgumentParser
    {
        ArgumentParser {
            args: ~[],
        }
    }

    pub fn add_argument(&mut self, arg: Argument)
    {
        self.args.push(arg);
    }

    pub fn add_arguments(&mut self, args: &[Argument])
    {
        self.args = vec::append(self.args.clone(), args);
    }

    pub fn parse_args(&self, _args: &[~str])
    {
        // NOTHING! \o/
    }
}

impl Argument
{
    pub fn new(short_names: ~[char], long_names: ~[~str]) -> Argument
    {
        Argument {
            short_names: short_names,
            long_names: long_names,
        }
    }

    pub fn new_short(short: char) -> Argument
    {
        Argument {
            short_names: ~[short],
            long_names: ~[],
        }
    }

    pub fn new_long(long: ~str) -> Argument
    {
        Argument {
            short_names: ~[],
            long_names: ~[long],
        }
    }
}
