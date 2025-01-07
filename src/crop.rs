use std::{error::Error, fs};

use cairo::{Context, ImageSurface};

use crate::Rect;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[clap(long="inset", value_parser = Rect::from_args, default_value_t = Rect::default())]
    rect: Rect,

    #[clap()]
    src: String,

    #[clap()]
    dst: String,
}

pub fn execute(args: Args) -> Result<(), Box<dyn Error>> {
    let src = ImageSurface::create_from_png(&mut fs::File::open(&args.src)?)?;

    let rect = Rect::from_xywh(0, 0, src.width() as i64, src.height() as i64).inset(&args.rect);
    if rect.is_empty() {
        return Err("empty rect".into());
    }

    let dst = ImageSurface::create(
        cairo::Format::ARgb32,
        rect.width() as i32,
        rect.height() as i32,
    )?;
    let ctx = Context::new(&dst)?;
    ctx.set_source_surface(&src, -rect.left() as f64, -rect.top() as f64)?;
    ctx.paint()?;

    dst.write_to_png(&mut fs::File::create(&args.dst)?)?;

    Ok(())
}
