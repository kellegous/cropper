use std::{error::Error, fs};

use cairo::ImageSurface;
use serde::Serialize;

#[derive(Debug, clap::Args)]
pub struct Args {
    src: String,
}

#[derive(Debug, Serialize)]
struct Info {
    width: i32,
    height: i32,
}

pub fn execute(args: Args) -> Result<(), Box<dyn Error>> {
    let src = ImageSurface::create_from_png(&mut fs::File::open(&args.src)?)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&Info {
            width: src.width(),
            height: src.height(),
        })?
    );
    Ok(())
}
