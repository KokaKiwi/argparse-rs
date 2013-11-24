use std::os;

use ArgumentParser;

pub trait HelpFormatter
{
    fn format_usage(&self, parser: &ArgumentParser) -> ~str
    {
        let cli_args = os::args();
        let program = cli_args[0].as_slice();

        self.format_usage_program(program, parser)
    }

    fn format_help(&self, parser: &ArgumentParser) -> ~str
    {
        let mut lines = ~[];

        lines.push(self.format_usage(parser));
        lines.push(~"");
        lines.push(~"Options:");

        let args = self.format_args(parser);
        for arg in args.iter()
        {
            lines.push(format!("  {}", *arg));
        }

        lines.connect("\n")
    }

    fn format_usage_program(&self, program: &str, parser: &ArgumentParser) -> ~str;
    fn format_args(&self, parser: &ArgumentParser) -> ~[~str];
}
