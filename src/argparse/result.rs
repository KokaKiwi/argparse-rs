use value::{ArgumentValue, ToArgValue};

pub struct ArgumentResults
{
    list: ~[ArgumentResult],
}

pub struct ArgumentResult
{
    names: ~[~str],
    value: ArgumentValue,
}

impl ArgumentResults
{
    pub fn new() -> ArgumentResults
    {
        ArgumentResults {
            list: ~[],
        }
    }

    pub fn len(&self) -> uint
    {
        self.list.len()
    }

    pub fn find<T: ToArgValue>(&self, name: &str) -> Option<T>
    {
        for result in self.list.iter()
        {
            for r_name in result.names.iter()
            {
                if name == *r_name
                {
                    let value = result.value.clone();
                    return ToArgValue::to_arg_value(value);
                }
            }
        }

        None
    }

    pub fn has(&self, name: &str) -> bool
    {
        for result in self.list.iter()
        {
            for r_name in result.names.iter()
            {
                if name == *r_name
                {
                    return true;
                }
            }
        }

        return false;
    }

    pub fn get<T: ToArgValue>(&self, name: &str) -> T
    {
        match self.find(name) {
            Some(v) => v,
            None => fail!("No arg named: {}", name),
        }
    }
}
