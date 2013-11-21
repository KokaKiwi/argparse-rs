use arg;
use result;
use err;
use value;

pub struct ArgumentParser<'self>
{
    args: ~[arg::Argument<'self>],
}

struct ArgumentResults
{
    real: result::ArgumentResults,
    free: ~[~str],
}

impl<'self> ArgumentParser<'self>
{
    pub fn new() -> ArgumentParser
    {
        ArgumentParser {
            args: ~[],
        }
    }

    pub fn add_argument(&mut self, arg: arg::Argument<'self>)
    {
        self.args.push(arg);
    }

    pub fn add_arguments(&mut self, args: &'self [arg::Argument<'self>])
    {
        for arg in args.iter()
        {
            self.add_argument(arg.clone());
        }
    }

    pub fn parse_args(&self, mut args: &[~str]) -> Result<result::ArgumentResults, err::ArgumentError<'self>>
    {
        let mut proxy = ArgumentResults::new();

        // Parse args
        while args.len() > 0
        {
            let leading = args[0].as_slice();
            match self.parse_leading(&mut proxy, leading, args.tail())
            {
                Ok(count) => {
                    args = args.slice_from(count + 1);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        let ArgumentResults {
            real: mut results,
            free: free
        } = proxy;
        let mut free = free.as_slice();

        // Check positional args
        for arg in self.args.iter()
        {
            if arg.is_positional()
            {
                if free.len() > 0
                {
                    let s = free[0].as_slice();
                    let value = match self.parse_value(arg, s) {
                        Ok(value) => value,
                        Err(e) => {
                            return Err(e);
                        }
                    };

                    let result = result::ArgumentResult {
                        names: arg.extract_names(),
                        value: value,
                    };
                    results.list.push(result);

                    free = free.tail();
                }
            }
        }

        // Check missing args
        for arg in self.args.iter()
        {
            let names = arg.extract_names();
            let name = names[0].to_owned();

            if !results.has(name)
            {
                match arg.opts.default
                {
                    Some(ref value) => {
                        let result = result::ArgumentResult {
                            names: names,
                            value: value.clone(),
                        };

                        results.list.push(result);
                    }
                    None => {
                        if arg.opts.required
                        {
                            return Err(err::MissingArgument(arg.values.clone()));
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    fn parse_leading(&self, proxy: &mut ArgumentResults, leading: &str, mut args: &[~str]) -> Result<uint, err::ArgumentError>
    {
        let mut count = 0u;

        if leading.starts_with("--")
        {
            match self.parse_arg(leading, args)
            {
                Ok((n, result)) => {
                    count += n;
                    proxy.real.list.push(result);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        else if leading.starts_with("-")
        {
            for c in leading.slice_from(1).iter()
            {
                let leading = format!("-{}", c);

                match self.parse_arg(leading, args)
                {
                    Ok((n, result)) => {
                        count += n;
                        proxy.real.list.push(result);

                        args = args.slice_from(n);
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
        else
        {
            proxy.free.push(leading.into_owned());
        }

        Ok(count)
    }

    fn parse_arg(&self, leading: &str, args: &[~str]) -> Result<(uint, result::ArgumentResult), err::ArgumentError>
    {
        for arg in self.args.iter()
        {
            for val in arg.values.iter()
            {
                if val.as_slice() == leading
                {
                    let mut count = 0;
                    let value = match arg.opts.ty {
                        arg::ArgTyBool => value::ArgValBool(true),
                        arg::ArgTyString => {
                            count = 1;

                            if args.len() == 0
                            {
                                return Err(err::NotEnoughArguments(format!("{}", leading)));
                            }

                            match self.parse_value(arg, args[0])
                            {
                                Ok(value) => value,
                                Err(e) => {
                                    return Err(e);
                                }
                            }
                        }
                        arg::ArgTyInteger => {
                            count = 1;

                            if args.len() == 0
                            {
                                return Err(err::NotEnoughArguments(format!("{}", leading)));
                            }

                            match self.parse_value(arg, args[0])
                            {
                                Ok(value) => value,
                                Err(e) => {
                                    return Err(e);
                                }
                            }
                        }
                    };

                    let result = result::ArgumentResult {
                        names: (*arg).extract_names(),
                        value: value,
                    };

                    return Ok((count, result));
                }
            }
        }

        Err(err::UnknownArgument(leading.into_owned()))
    }

    fn parse_value(&self, arg: &arg::Argument, s: &str) -> Result<value::ArgumentValue, err::ArgumentError>
    {
        match arg.opts.ty
        {
            arg::ArgTyString => Ok(value::ArgValString(s.to_owned())),
            arg::ArgTyInteger => {
                let n: int = match from_str(s) {
                    Some(n) => n,
                    None => {
                        return Err(err::BadArgumentFormat(format!("Expected integer, got: {}", s)))
                    }
                };

                Ok(value::ArgValInteger(n))
            },
            _ => fail!(),
        }
    }

    pub fn print_help(&self)
    {

    }
}

impl ArgumentResults
{
    pub fn new() -> ArgumentResults
    {
        ArgumentResults {
            real: result::ArgumentResults::new(),
            free: ~[],
        }
    }
}
