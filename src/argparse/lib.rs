#[link(
    uuid = "e21f6c0a-deac-4d66-8dc9-b116410d67df"
)];

#[license = "MIT"];

#[crate_id = "github.com/KokaKiwi/argparse#0.1.0"];
#[crate_type = "lib"];

pub use parser::ArgumentParser;

mod parser;
pub mod arg;

pub mod format;

pub mod err;

pub mod result;
pub mod value;
