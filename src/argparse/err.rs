
pub enum ArgumentError<'self>
{
    UnknownArgument(~str),
    NotEnoughArguments(~str),
    MissingArgument(~[&'self str]),
    BadArgumentFormat(~str),
    NotImplementedYet(&'self str),
}

impl<'self> ToStr for ArgumentError<'self>
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
