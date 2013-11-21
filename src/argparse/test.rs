#[feature(globs)];
#[feature(macro_rules)];

extern mod argparse;

use argparse::ArgumentParser;
use argparse::arg::*;
use argparse::value::*;

mod macros;

macro_rules! parse_args(
    ($parser:ident: $($arg:expr),*) => {
        match $parser.parse_args(&[$(~$arg),*]) {
            Ok(args) => args,
            Err(e) => fail!(e.to_str()),
        }
    };

    ($parser:ident) => (parse_args!($parser:));
)

#[test]
fn test_no_args()
{
    let parser = ArgumentParser::new();

    let args = parse_args!(parser);
    assert_eq!(args.len(), 0);
}

#[test]
fn test_bool_opt()
{
    let mut parser = ArgumentParser::new();
    parser.add_argument(create_arg!("-b", "--bool"; ty = ArgTyBool));

    let args = parse_args!(parser);
    assert_eq!(args.get::<bool>("bool"), false);

    let args = parse_args!(parser: "-b");
    assert_eq!(args.get::<bool>("bool"), true);

    let args = parse_args!(parser: "--bool");
    assert_eq!(args.get::<bool>("bool"), true);
}

#[test]
fn test_str_opt()
{
    let mut parser = ArgumentParser::new();
    parser.add_argument(create_arg!("-s", "--str"; default = Some(ArgValString(~"default"))));

    let args = parse_args!(parser);
    assert_eq!(args.get::<~str>("str"), ~"default");

    let args = parse_args!(parser: "-s", "toto");
    assert_eq!(args.get::<~str>("str"), ~"toto");

    let args = parse_args!(parser: "--str", "toto");
    assert_eq!(args.get::<~str>("str"), ~"toto");
}

#[test]
#[should_fail]
fn test_str_opt_missing_required()
{
    let mut parser = ArgumentParser::new();
    parser.add_argument(create_arg!("-s", "--str"; required = true));

    parse_args!(parser);
}

#[test]
fn test_int_opt()
{
    let mut parser = ArgumentParser::new();
    parser.add_argument(create_arg!("-i", "--int"; ty = ArgTyInteger, default = Some(ArgValInteger(42))));

    let args = parse_args!(parser);
    assert_eq!(args.get::<int>("int"), 42);

    let args = parse_args!(parser: "-i", "32");
    assert_eq!(args.get::<int>("int"), 32);

    let args = parse_args!(parser: "--int", "32");
    assert_eq!(args.get::<int>("int"), 32);
}

#[test]
#[should_fail]
fn test_int_opt_bad_format()
{
    let mut parser = ArgumentParser::new();
    parser.add_argument(create_arg!("-i", "--int"; ty = ArgTyInteger));

    parse_args!(parser: "-i", "toto");
}

#[test]
#[should_fail]
fn test_int_opt_missing_required()
{
    let mut parser = ArgumentParser::new();
    parser.add_argument(create_arg!("-i", "--int"; ty = ArgTyInteger, required = true));

    parse_args!(parser);
}

#[test]
fn test_positional_str()
{
    let mut parser = ArgumentParser::new();
    parser.add_argument(create_arg!("pos"; default = Some(ArgValString(~"default"))));

    let args = parse_args!(parser);
    assert_eq!(args.get::<~str>("pos"), ~"default");

    let args = parse_args!(parser: "toto");
    assert_eq!(args.get::<~str>("pos"), ~"toto");
}

#[test]
#[should_fail]
fn test_positional_str_missing_no_default()
{
    let mut parser = ArgumentParser::new();
    parser.add_argument(create_arg!("pos"));

    parse_args!(parser);
}

#[test]
fn test_positional_int()
{
    let mut parser = ArgumentParser::new();
    parser.add_argument(create_arg!("pos"; ty = ArgTyInteger, default = Some(ArgValInteger(42))));

    let args = parse_args!(parser);
    assert_eq!(args.get::<int>("pos"), 42);

    let args = parse_args!(parser: "32");
    assert_eq!(args.get::<int>("pos"), 32);
}
