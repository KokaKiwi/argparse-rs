use ArgumentParser;
use arg;
use value;
use super::traits::HelpFormatter;

pub struct DefaultHelpFormatter;

impl HelpFormatter for DefaultHelpFormatter
{
    fn format_usage_program(&self, program: &str, parser: &ArgumentParser) -> ~str
    {
        let mut parts = ~[];

        parts.push(~"Usage:");
        parts.push(program.into_owned());

        for arg in parser.args.iter()
        {
            parts.push(self.format_usage_arg(arg));
        }

        parts.connect(" ")
    }

    fn format_args(&self, parser: &ArgumentParser) -> ~[~str]
    {
        let mut args = ~[];

        for arg in parser.args.iter()
        {
            let (left, right) = self.format_arg(arg);
            args.push(if right.len() > 0 {
                format!("{:20s} {:s}", left, right.trim())
            } else {
                left
            });
        }

        args
    }
}

impl DefaultHelpFormatter
{
    fn format_usage_arg(&self, arg: &arg::Argument) -> ~str
    {
        let mut s = arg.values[0].into_owned();

        if !arg.is_positional()
        {
            match arg.opts.ty
            {
                arg::ArgTyString | arg::ArgTyInteger => {
                    s = format!("{} value", s);
                }
                _ => {}
            }
        }

        if !arg.opts.required
        {
            s = format!("[{}]", s);
        }

        s
    }

    fn format_arg(&self, arg: &arg::Argument) -> (~str, ~str)
    {
        let mut left = ~[];
        let mut right = ~[];

        for value in arg.values.iter()
        {
            left.push(value.into_owned());
        }

        match arg.opts.help
        {
            Some(s) => {
                right.push(s.into_owned());
            }
            None => {}
        }

        if arg.opts.ty != arg::ArgTyBool
        {
            match arg.opts.default
            {
                Some(ref value) => {
                    right.push(format!(" (default: {:s})", self.format_value(value)));
                }
                None => {}
            }
        }

        (left.connect(", "), right.concat())
    }

    fn format_value(&self, value: &value::ArgumentValue) -> ~str
    {
        match *value
        {
            value::ArgValBool(b) => b.to_str(),
            value::ArgValString(ref s) => s.to_owned(),
            value::ArgValInteger(n) => n.to_str(),
        }
    }
}
