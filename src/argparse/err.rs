
pub enum ArgumentError<'a>
{
    UnknownArgument(~str),
    NotEnoughArguments(~str),
    MissingArgument(~[&'a str]),
    BadArgumentFormat(~str),
    NotImplementedYet(&'a str),
}

impl<'a> ToStr for ArgumentError<'a>
{
    fn to_str(&self) -> ~str
    {
        match *self
        {
            UnknownArgument(ref name) => format!("Unknown argument: {}", *name),
            NotEnoughArguments(ref msg) => format!("Not enough arguments: {}", *msg),
            MissingArgument(ref args) => format!("Missing arguments: {}", (*args).connect(" ")),
            BadArgumentFormat(ref name) => format!("Bad argument format: {}", *name),
            NotImplementedYet(name) => format!("Not implemented yet: {}", name),
        }
    }
}
