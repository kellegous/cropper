use std::error::Error;

use clap::{Parser, Subcommand};
use cropper::{crop, info, preview};

#[derive(Debug, Parser)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Preview(preview::Args),
    Crop(crop::Args),
    Info(info::Args),
}

impl Command {
    fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Command::Preview(args) => preview::execute(args),
            Command::Crop(args) => crop::execute(args),
            Command::Info(args) => info::execute(args),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    args.command.execute()
}
