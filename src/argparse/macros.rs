#[macro_escape];

macro_rules! create_arg(
    ($($value:expr),+; $($key:ident = $val:expr),*) => {
        {
            use argparse::arg::{Argument, ArgumentOptions};

            let values = ~[$($value),+];
            let opts = ArgumentOptions {
            $(
                $key: $val,
            )*
                .. ArgumentOptions::default()
            };

            Argument::new(values, opts)
        }
    };

    ($($value:expr),+) => {
        {
            use argparse::arg::{Argument, ArgumentOptions};

            let values = ~[$($value),*];

            Argument::new(values, ArgumentOptions::default())
        }
    };
)
