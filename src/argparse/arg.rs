use value;

#[deriving(Clone, Eq)]
pub struct Argument<'a>
{
    values: ~[&'a str],
    opts: ArgumentOptions<'a>,
}

#[deriving(Clone, Eq)]
pub struct ArgumentOptions<'a>
{
    required: bool,
    ty: ArgumentType,

    default: Option<value::ArgumentValue>,
    help: Option<&'a str>,
}

#[deriving(Clone, Eq)]
pub enum ArgumentType
{
    ArgTyBool,
    ArgTyString,
    ArgTyInteger,
}

impl<'a> Argument<'a>
{
    #[allow(unused_mut)] #[inline]
    pub fn new(values: ~[&'a str], mut opts: ArgumentOptions<'a>) -> Argument<'a>
    {
        let mut arg = Argument {
            values: values,
            opts: opts,
        };

        if arg.opts.ty == ArgTyBool
        {
            arg.opts.default = Some(value::ArgValBool(false));
        }

        if arg.is_positional() && arg.opts.default == None
        {
            arg.opts.required = true;
        }

        arg
    }

    pub fn extract_names(&self) -> ~[~str]
    {
        let mut names = ~[];

        for val in self.values.iter()
        {
            if val.starts_with("--")
            {
                names.push(val.slice_from(2).into_owned());
            }
            else if val.starts_with("-")
            {
                names.push(val.slice_from(1).into_owned());
            }
            else
            {
                names.push(val.into_owned());
            }
        }

        names
    }

    pub fn is_positional(&self) -> bool
    {
        self.values.len() == 1 && !self.values[0].starts_with("-")
    }
}

impl<'a> ArgumentOptions<'a>
{
    #[inline]
    pub fn default() -> ArgumentOptions
    {
        ArgumentOptions {
            required: false,
            ty: ArgTyString,

            default: None,
            help: None,
        }
    }
}
