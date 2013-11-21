
#[deriving(Clone, Eq)]
pub enum ArgumentValue
{
    ArgValBool(bool),
    ArgValString(~str),
    ArgValInteger(int),
}

pub trait ToArgValue
{
    fn to_arg_value(value: ArgumentValue) -> Option<Self>;
}

impl ToArgValue for bool
{
    fn to_arg_value(value: ArgumentValue) -> Option<bool>
    {
        match value
        {
            ArgValBool(b) => Some(b),
            _ => None,
        }
    }
}

impl ToArgValue for ~str
{
    fn to_arg_value(value: ArgumentValue) -> Option<~str>
    {
        match value
        {
            ArgValString(s) => Some(s),
            _ => None,
        }
    }
}

impl ToArgValue for int
{
    fn to_arg_value(value: ArgumentValue) -> Option<int>
    {
        match value
        {
            ArgValInteger(n) => Some(n),
            _ => None,
        }
    }
}
